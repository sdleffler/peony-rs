mod number;
mod number_lexer;

use std::{char, num::ParseIntError, str::CharIndices};

use itertools::{self, Itertools, MultiPeek};
use num::{BigInt, BigRational, BigUint};
use regex::{Regex, RegexSet};
use unicode_categories::UnicodeCategories;

use atom::Atom;

use self::number_lexer::Lexer as NumberLexer;

fn is_whitespace(c: char) -> bool {
    match c {
        '\t' | '\n' | '\x0b' | '\x0c' | '\r' | '\u{0085}' => true,
        _ => c.is_separator(),
    }
}

fn is_delimiter(c: char) -> bool {
    match c {
        '(' | ')' | '[' | ']' | '"' | ';' | '#' => true,
        _ => is_whitespace(c),
    }
}

fn is_constituent(c: char) -> bool {
    c.is_ascii_alphabetic()
        || (c as u32 > 127
            && (c.is_letter() || c.is_mark_nonspacing() || c.is_number_letter()
                || c.is_number_other() || c.is_punctuation_connector()
                || c.is_punctuation_dash() || c.is_punctuation_other()
                || c.is_symbol() || c.is_other_private_use()))
}

fn is_initial(c: char) -> bool {
    match c {
        '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' | '>' | '?' | '^' | '_' | '~' => true,
        _ => is_constituent(c),
    }
}

fn is_subsequent(c: char) -> bool {
    match c {
        '+' | '-' | '.' | '@' => true,
        _ => is_initial(c) || c.is_number() || c.is_mark(),
    }
}

fn is_start_peculiar(c: char) -> bool {
    match c {
        '+' | '-' | '.' => true,
        _ => false,
    }
}

fn is_intraline_whitespace(c: char) -> bool {
    c == '\t' || c.is_separator_space()
}

fn is_start_line_ending(c: char) -> bool {
    match c {
        '\n' | '\r' | '\u{0085}' | '\u{2028}' => true,
        _ => false,
    }
}

// Returns `n` characters for an n-character line ending, unless `n == 0`, in which case `s` does not start a line ending.
fn detect_line_ending(s: &str) -> Option<usize> {
    match s {
        "\r\n" | "\r\u{0085}" => Some(2),
        s if s.starts_with(is_start_line_ending) => Some(1),
        _ => None,
    }
}

pub type Spanned = (usize, Token, usize);

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "expected a delimiter")]
    ExpectedDelimiter,

    #[fail(display = "expected an identifier")]
    ExpectedIdentifier,

    #[fail(display = "'{:?}' is not valid as the starting character of an identifier", _0)]
    InvalidIdentifierStart(char),

    #[fail(display = "character '{:?}' is not valid as part of an identifier", _0)]
    InvalidIdentifierCont(char),

    #[fail(display = "unrecognized token")]
    UnrecognizedToken,

    #[fail(display = "expected a boolean")]
    ExpectedBoolean,

    #[fail(display = "expected end of nested comment ({} levels deep)", _0)]
    ExpectedEndOfNestedComment(u8),

    #[fail(display = "expected a character")]
    ExpectedCharacter,

    #[fail(display = "expected a character, character name, or hex code")]
    ExpectedCharacterCont,

    #[fail(display = "invalid hexadecimal character code: {}", _0)]
    InvalidHexCode(ParseIntError),

    #[fail(display = "invalid character code, no such unicode code point {}", _0)]
    InvalidCharacter(u32),

    #[fail(display = "unrecognized character name")]
    UnrecognizedCharacterName,

    #[fail(display = "expected a line ending")]
    ExpectedLineEnding,

    #[fail(display = "unexpected EOF")]
    UnexpectedEof,

    #[fail(display = "unrecognized escape sequence")]
    UnrecognizedEscape,

    #[fail(display = "unrecognized number prefix")]
    UnrecognizedNumberPrefix,

    #[fail(display = "expected a number")]
    ExpectedNumber,
}

#[derive(Debug, Fail)]
#[fail(display = "o no")]
pub struct Error {
    pub location: usize,
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind, location: usize) -> Self {
        Self { location, kind }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Precision {
    Single,
    Double,
}

impl Default for Precision {
    fn default() -> Self {
        Precision::Double
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UReal {
    Integer(BigUint),
    Rational(BigUint, BigUint),
    Decimal(Precision, BigUint, isize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sign {
    Pos,
    Neg,
}

impl Default for Sign {
    fn default() -> Self {
        Sign::Pos
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NanInf {
    Nan,
    Inf,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Real {
    Signed(Sign, UReal),
    NanInf(Sign, NanInf),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Number {
    Real(Real),
    Complex(Real, Real),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    Identifier(Atom),
    Number(Number),
    Character(char),
    String(Atom),

    True,  // #t/#T
    False, // #f/#F

    LParen, // (
    RParen, // )
    LBrack, // [
    RBrack, // ]

    Dot, // .

    VectorOpen,     // #(
    ByteVectorOpen, // #u8(

    Quote,           // '
    QuasiQuote,      // `
    Unquote,         // ,
    UnquoteSplicing, // ,@

    Syntax,           // #'
    QuasiSyntax,      // #`
    Unsyntax,         // #,
    UnsyntaxSplicing, // #,@
}

// fn identifier<'input>(text: &'input str, idx0: usize) -> Result<Spanned, Error> {
//     let check_unpeculiar = match text {
//         "+" | "-" | "..." => {
//             return Ok((idx0, Token::Identifier(Atom::from(text)), idx0 + text.len()))
//         }
//
//         s if s.starts_with("->") => &s[2..],
//         s => s,
//     };
//
//     let mut chars = check_unpeculiar.chars();
//     match chars.next() {
//         Some(c) if is_initial(c) => match chars.find(|&c| !is_subsequent(c)) {
//             None => Ok((idx0, Token::Identifier(Atom::from(text)), idx0 + text.len())),
//             Some(c) => return Err(Error::new(ErrorKind::InvalidIdentifierCont(c), idx0)),
//         },
//         Some(c) => return Err(Error::new(ErrorKind::InvalidIdentifierStart(c), idx0)),
//         None => return Err(Error::new(ErrorKind::ExpectedIdentifier, idx0)),
//     }
// }
//
// fn boolean<'input>(text: &'input str, idx0: usize) -> Result<Spanned, Error> {
//     lazy_static! {
//         static ref BOOL_SET: RegexSet = RegexSet::new(&[
//             r"^(#f|#F)$",
//             r"^(#t|#T)$",
//         ]).unwrap();
//     }
//
//     let token = match BOOL_SET.matches(text).into_iter().next() {
//         Some(0) => Token::False,
//         Some(1) => Token::True,
//         Some(_) => unreachable!(),
//         None => return Err(Error::new(ErrorKind::ExpectedBoolean, idx0)),
//     };
//
//     Ok((idx0, token, idx0 + text.len()))
// }
//
// fn number<'input>(text: &'input str, idx0: usize) -> Result<Spanned, Error> {
//     lazy_static! {
//         static ref PREFIX_SET: RegexSet = RegexSet::new(&[
//             r"#[bB]",         // radix 2
//             r"#[oO]",         // radix 8
//             r"#[dD]",         // radix 10
//             r"#[xX]",         // radix 16
//             r"#[iI]",         // inexact
//             r"#[eE]",         // exact
//         ]).unwrap();
//
//         static ref COMPLEX_SET: RegexSet = RegexSet::new(&[
//             r"^[^@]+@[^@]+$", // n@m
//             r"i$",            // other complex
//         ]).unwrap();
//     }
//
//     let prefix_matches = PREFIX_SET.matches(text);
//     match COMPLEX_SET.matches(text).into_iter().next() {
//         Some(0) => unimplemented!(),
//         Some(1) => unimplemented!(),
//         Some(_) => unreachable!(),
//         None => unimplemented!(),
//     }
// }
//
// fn character<'input>(text: &'input str, idx0: usize) -> Result<Spanned, Error> {
//     lazy_static! {
//         static ref RE_SET: RegexSet = RegexSet::new(&[
//             r"^nul$",
//             r"^alarm$",
//             r"^backspace$",
//             r"^tab$",
//             r"^(linefeed|newline)$",
//             r"^vtab$",
//             r"^page$",
//             r"^return$",
//             r"^esc$",
//             r"^space$",
//             r"^delete$",
//             r"^x[[:xdigit:]]+$",
//             r"^.$",
//         ]).unwrap();
//     }
//
//     if !text.starts_with(r"#\") {
//         Err(Error::new(ErrorKind::ExpectedCharacter, idx0))
//     } else {
//         let code = &text[2..];
//         let character = match RE_SET.matches(code).into_iter().next() {
//             Some(0) => '\0',                                     // nul
//             Some(1) => '\x07',                                   // alarm
//             Some(2) => '\x08',                                   // backspace
//             Some(3) => '\t',                                     // tab
//             Some(4) => '\n',                                     // (linefeed|newline)
//             Some(5) => '\x0b',                                   // vtab
//             Some(6) => '\x0c',                                   // page
//             Some(7) => '\r',                                     // return
//             Some(8) => unimplemented!("what the fuck is this?"), // esc
//             Some(9) => ' ',                                      // space
//             Some(10) => '\x7f',                                  // delete
//             Some(11) => {
//                 // hex scalar
//                 let scalar = u32::from_str_radix(&code[3..], 16)
//                     .map_err(|e| Error::new(ErrorKind::InvalidHexCode(e), idx0))?;
//                 char::from_u32(scalar)
//                     .ok_or_else(|| Error::new(ErrorKind::InvalidCharacter(scalar), idx0))?
//             }
//             Some(12) => code.chars().next().unwrap(),
//             Some(_) => unreachable!(),
//             None => return Err(Error::new(ErrorKind::ExpectedCharacterCont, idx0 + 2)),
//         };
//         Ok((idx0, Token::Character(character), idx0 + text.len()))
//     }
// }

enum NumberRadix {
    Radix2,
    Radix8,
    Radix10,
    Radix16,
}

// public due to lalrpop shenanigans
pub struct NumberFlags {
    // `Some` if specified exact or inexact; `None` if inferred from syntax.
    exact: Option<bool>,
    radix: NumberRadix,
}

impl NumberFlags {
    fn to_radix(&self) -> u32 {
        match self.radix {
            NumberRadix::Radix2 => 2,
            NumberRadix::Radix8 => 8,
            NumberRadix::Radix10 => 10,
            NumberRadix::Radix16 => 16,
        }
    }
}

pub struct Lexer<'input> {
    text: &'input str,
    chars: MultiPeek<CharIndices<'input>>,
    lookahead: String,
    location: Option<usize>,
    shift: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str, shift: usize) -> Self {
        let mut this = Self {
            text: input,
            chars: itertools::multipeek(input.char_indices()),
            lookahead: String::new(),
            location: None,
            shift,
        };

        this.bump(0);
        this
    }

    fn bump(&mut self, n: usize) -> Option<usize> {
        self.lookahead.clear();
        self.chars.by_ref().dropping(n);

        self.chars.reset_peek();
        if let Some(&(loc, c)) = self.chars.peek() {
            self.location = Some(loc);
            self.lookahead.push(c);
            self.lookahead.extend(self.chars.peek().map(|t| t.1));

            Some(loc)
        } else {
            self.location = None;

            None
        }
    }

    fn bump_until<F: FnMut(char) -> bool>(&mut self, mut stop: F) -> Option<usize> {
        self.bump_while(|c| !stop(c))
    }

    fn bump_while<F: FnMut(char) -> bool>(&mut self, mut keep_going: F) -> Option<usize> {
        self.lookahead.clear();
        self.chars
            .by_ref()
            .peeking_take_while(|&(_, c)| keep_going(c))
            .for_each(|_| ());

        self.chars.reset_peek();
        if let Some(&(loc, c)) = self.chars.peek() {
            self.location = Some(loc);
            self.lookahead.push(c);
            self.lookahead.extend(self.chars.peek().map(|t| t.1));

            Some(loc)
        } else {
            self.location = None;

            None
        }
    }

    fn bump_to(&mut self, bytes: usize) {
        self.lookahead.clear();
        self.chars
            .by_ref()
            .peeking_take_while(|&(idx0, _)| idx0 < bytes)
            .for_each(|_| ());

        self.chars.reset_peek();
        if let Some(&(loc, c)) = self.chars.peek() {
            self.location = Some(loc);
            self.lookahead.push(c);
            self.lookahead.extend(self.chars.peek().map(|t| t.1));
        } else {
            self.location = None;
        }
    }

    fn expect_delimiter<F: FnOnce() -> Result<Spanned, Error>>(
        &self,
        thunk: F,
    ) -> Result<Spanned, Error> {
        if self.lookahead.starts_with(is_delimiter) {
            thunk()
        } else {
            let idx0 = self.location.unwrap_or(self.text.len());
            Err(Error::new(ErrorKind::ExpectedDelimiter, idx0))
        }
    }

    fn simple_lexemes(&mut self, idx0: usize) -> Option<Result<Spanned, Error>> {
        let maybe_length_and_token = match &self.text[idx0..] {
            s if s.starts_with("#vu8(") => Some((5, Token::ByteVectorOpen)),
            s if s.starts_with("#,@") => Some((3, Token::UnsyntaxSplicing)),
            s if s.starts_with("#`") => Some((2, Token::QuasiSyntax)),
            s if s.starts_with("#'") => Some((2, Token::Syntax)),
            s if s.starts_with(",@") => Some((2, Token::UnquoteSplicing)),
            s if s.starts_with("#,") => Some((2, Token::Unsyntax)),
            s if s.starts_with("#(") => Some((2, Token::VectorOpen)),
            s if s.starts_with('(') => Some((1, Token::LParen)),
            s if s.starts_with(')') => Some((1, Token::RParen)),
            s if s.starts_with('[') => Some((1, Token::LBrack)),
            s if s.starts_with(']') => Some((1, Token::RBrack)),
            s if s.starts_with('\'') => Some((1, Token::Quote)),
            s if s.starts_with('`') => Some((1, Token::QuasiQuote)),
            s if s.starts_with(',') => Some((1, Token::Unquote)),
            s if s.starts_with('.') => Some((1, Token::Dot)),
            _ => None,
        };

        maybe_length_and_token.map(|(n, tok)| {
            self.bump(n);
            match tok {
                Token::Dot => self.expect_delimiter(|| Ok((idx0, tok, idx0 + n))), // `.` must be followed by a delimiter
                _ => Ok((idx0, tok, idx0 + n)),
            }
        })
    }

    fn identifier(&mut self, idx0: usize) -> Option<Result<Spanned, Error>> {
        if self.lookahead.starts_with(is_initial) {
            let (slice, idx1) = match self.bump_while(is_subsequent) {
                Some(idx1) => (&self.text[idx0..idx1], idx1),
                None => (&self.text[idx0..], self.text.len()),
            };

            Some(self.expect_delimiter(|| Ok((idx0, Token::Identifier(Atom::from(slice)), idx1))))
        } else if self.text[idx0..].starts_with("->") {
            self.bump(2);

            let (slice, idx1) = match self.bump_while(is_subsequent) {
                Some(idx1) => (&self.text[idx0..idx1], idx1),
                None => (&self.text[idx0..], self.text.len()),
            };

            Some(self.expect_delimiter(|| Ok((idx0, Token::Identifier(Atom::from(slice)), idx1))))
        } else if self.lookahead.starts_with(is_start_peculiar) {
            let (n, tok) = match &self.text[idx0..] {
                s if s.starts_with("+") => (1, Token::Identifier(atom!("+"))),
                s if s.starts_with("-") => (1, Token::Identifier(atom!("-"))),
                s if s.starts_with("...") => (3, Token::Identifier(atom!("..."))),
                _ => return Some(Err(Error::new(ErrorKind::ExpectedIdentifier, idx0))),
            };

            self.bump(n);
            Some(self.expect_delimiter(|| Ok((idx0, tok, idx0 + n))))
        } else {
            None
        }
    }

    fn boolean(&mut self, idx0: usize) -> Option<Result<Spanned, Error>> {
        match &*self.lookahead {
            "#t" | "#T" | "#f" | "#F" => {
                let token = match &*self.lookahead {
                    "#t" | "#T" => Token::True,
                    "#f" | "#F" => Token::False,
                    _ => unreachable!(),
                };
                self.bump(2);
                Some(self.expect_delimiter(|| Ok((idx0, token, idx0 + 2))))
            }
            _ => None,
        }
    }

    fn number_with_flags(
        &mut self,
        idx0: usize,
        number_flags: NumberFlags,
    ) -> Option<Result<Spanned, Error>> {
        let (slice, idx1) = match self.bump_until(is_delimiter) {
            Some(idx1) => (&self.text[idx0..idx1], idx1),
            None => (&self.text[idx0..], self.text.len()),
        };

        let number_lexer = NumberLexer::new(slice, idx0, number_flags.to_radix());
        match number::SpannedNumberParser::new().parse(&number_flags, number_lexer) {
            Ok((idx0, number, idx1)) => Some(Ok((idx0, Token::Number(number), idx1))),
            Err(err) => unimplemented!("o fuck: {:?}", err),
        }
    }

    fn number(&mut self, idx0: usize) -> Option<Result<Spanned, Error>> {
        lazy_static! {
            // Regex set to detect whether we're looking at a number, from its first two
            // characters.
            //
            // Three cases:
            // 1. The num has a prefix. The first two characters are `#` and some prefix code.
            //    All non-decimal numbers will start with a prefix indicating as such.
            // 2. The num starts with a sign. The next possibilities are either `i`,
            //    `<naninf>`, `<uinteger 10>`, or `<decimal 10>`. This boils down to the
            //    character class `[ni0-9.]`.
            // 3. The num is complex with either a `+`, `-`, `/`, or `@` on the inside. If the
            //    inner `+`/`-`/`@` shows up on the inside within two characters it will only
            //    do so after a decimal digit. By extending the separator set to include
            //    delimiters we also cover the case where the number is a single digit. Adding
            //    a point to the set, we also cover decimals of the form `\d.`. Adding the
            //    start of the mantissa width sublexeme and exponent markers, we cover
            //    single-digit decimals. And last but not least, by adding a second possible
            //    digit, we cover the case where a number is two decimal digits in a row.
            // 4. The number is a decimal starting with a decimal point.
            static ref NUMBER_START: RegexSet = RegexSet::new(&[
                r#"^#[iIeEbBoOdDxX]$"#,
                r#"^[+-][ni0-9.]$"#,
                r#"^[0-9][-+@/.|eEsSfFdDlL0-9()\[\]";#\t\n\x0b\x0c\r\u{0085}\pZ]$"#,
                r#"^[.][0-9]$"#,
            ]).unwrap();
        }

        if NUMBER_START.is_match(&self.lookahead) {
            let mut exact = None;
            let mut radix = None;

            while self.lookahead.starts_with("#") {
                match &self.lookahead[1..] {
                    s if s.eq_ignore_ascii_case("i") => exact = Some(false),
                    s if s.eq_ignore_ascii_case("e") => exact = Some(true),
                    s if s.eq_ignore_ascii_case("b") => radix = Some(NumberRadix::Radix2),
                    s if s.eq_ignore_ascii_case("o") => radix = Some(NumberRadix::Radix8),
                    s if s.eq_ignore_ascii_case("d") => radix = Some(NumberRadix::Radix10),
                    s if s.eq_ignore_ascii_case("x") => radix = Some(NumberRadix::Radix16),
                    _ => return Some(Err(Error::new(ErrorKind::UnrecognizedNumberPrefix, idx0))),
                }

                self.bump(2);
            }

            match self.location {
                Some(idx0) => self.number_with_flags(
                    idx0,
                    NumberFlags {
                        exact,
                        radix: radix.unwrap_or(NumberRadix::Radix10),
                    },
                ),
                None => Some(Err(Error::new(ErrorKind::ExpectedNumber, idx0))),
            }
        } else {
            None
        }
    }

    fn character(&mut self, idx0: usize) -> Option<Result<Spanned, Error>> {
        fn detect(i: &str, s: &str, c: char) -> Option<(usize, char)> {
            if i.starts_with(s) {
                Some((s.len(), c))
            } else {
                None
            }
        }

        if self.lookahead == r"#\" {
            self.bump(2);

            let sliced = &self.text[idx0 + 2..];
            let maybe_length_and_char = detect(sliced, "nul", '\0')
                .or_else(|| detect(sliced, "alarm", '\x07'))
                .or_else(|| detect(sliced, "backspace", '\x08'))
                .or_else(|| detect(sliced, "tab", '\t'))
                .or_else(|| detect(sliced, "linefeed", '\n'))
                .or_else(|| detect(sliced, "newline", '\n'))
                .or_else(|| detect(sliced, "vtab", '\x0b'))
                .or_else(|| detect(sliced, "page", '\x0c'))
                .or_else(|| detect(sliced, "return", '\r'))
                .or_else(|| detect(sliced, "esc", '\x1b'))
                .or_else(|| detect(sliced, "space", ' '))
                .or_else(|| detect(sliced, "delete", '\x7f'));

            if let Some((n, c)) = maybe_length_and_char {
                self.bump(n);
                Some(Ok((idx0, Token::Character(c), idx0 + n)))
            } else if self.lookahead.starts_with('x') {
                let (slice, idx1) = match self.bump_while(|c| c.is_ascii_hexdigit()) {
                    Some(idx1) => (&self.text[idx0..idx1], idx1),
                    None => (&self.text[idx0..], self.text.len()),
                };

                match slice[3..].parse::<u32>() {
                    Ok(n) => match char::from_u32(n) {
                        Some(c) => Some(Ok((idx0, Token::Character(c), idx1))),
                        None => Some(Err(Error::new(ErrorKind::InvalidCharacter(n), idx0))),
                    },
                    Err(e) => Some(Err(Error::new(ErrorKind::InvalidHexCode(e), idx0))),
                }
            } else {
                match self.lookahead.chars().next() {
                    Some(c) => {
                        self.bump(1);
                        Some(Ok((idx0, Token::Character(c), idx0 + c.len_utf8())))
                    }
                    None => Some(Err(Error::new(ErrorKind::ExpectedCharacter, idx0))),
                }
            }
        } else {
            None
        }
    }

    fn string(&mut self, idx0: usize) -> Option<Result<Spanned, Error>> {
        if self.lookahead.starts_with("\"") {
            let mut loc = match self.bump(1) {
                Some(loc) => loc,
                None => return Some(Err(Error::new(ErrorKind::UnexpectedEof, idx0))),
            };
            let mut buf = String::new();

            loop {
                let idx1 = match self.bump_until(|c| c == '\"' || c == '\\') {
                    Some(idx1) => idx1,
                    None => return Some(Err(Error::new(ErrorKind::UnexpectedEof, idx0))),
                };

                buf.push_str(&self.text[loc..idx1]);
                loc = idx1;

                if self.lookahead.starts_with('\\') {
                    if self.lookahead
                        .ends_with(|c| is_intraline_whitespace(c) || is_start_line_ending(c))
                    {
                        self.bump(1);
                        let break_loc = match self.bump_while(is_intraline_whitespace) {
                            Some(break_loc) => break_loc,
                            None => return Some(Err(Error::new(ErrorKind::UnexpectedEof, idx0))),
                        };

                        match detect_line_ending(&self.lookahead) {
                            Some(n) => {
                                self.bump(n);
                            }
                            None => {
                                println!("oops {:?}", self.lookahead);

                                return Some(Err(Error::new(
                                    ErrorKind::ExpectedLineEnding,
                                    break_loc,
                                )))
                            }
                        }

                        match self.bump_while(is_intraline_whitespace) {
                            Some(idx1) => loc = idx1,
                            None => return Some(Err(Error::new(ErrorKind::UnexpectedEof, idx0))),
                        }
                    } else if self.lookahead == r"\x" {
                        unimplemented!();
                    } else {
                        let c = match &*self.lookahead {
                            r#"\a"# => '\x07',
                            r#"\b"# => '\x08',
                            r#"\t"# => '\t',
                            r#"\n"# => '\n',
                            r#"\v"# => '\x0b',
                            r#"\f"# => '\x0c',
                            r#"\r"# => '\r',
                            r#"\""# => '\"',
                            r#"\\"# => '\\',
                            _ => return Some(Err(Error::new(ErrorKind::UnrecognizedEscape, idx0))),
                        };

                        buf.push(c);

                        match self.bump(2) {
                            Some(idx1) => loc = idx1,
                            None => return Some(Err(Error::new(ErrorKind::UnexpectedEof, idx0))),
                        }
                    }
                } else if let Some(n) = detect_line_ending(&self.lookahead) {
                    buf.push('\n');
                    match self.bump(n) {
                        Some(idx1) => loc = idx1,
                        None => return Some(Err(Error::new(ErrorKind::UnexpectedEof, idx0))),
                    }
                } else if self.lookahead.starts_with('\"') {
                    let idx1 = match self.bump(1) {
                        Some(idx1) => idx1,
                        None => self.text.len(),
                    };

                    return Some(Ok((idx0, Token::String(Atom::from(&*buf)), idx1)));
                } else {
                    unreachable!("lookahead must start with \" or \\");
                }
            }
        } else {
            None
        }
    }

    fn next_unshifted(&mut self) -> Option<Result<Spanned, Error>> {
        loop {
            let idx0 = match self.location {
                Some(idx0) => idx0,
                None => return None,
            };

            println!("position {}", idx0);

            // interlexeme space
            {
                if self.lookahead.starts_with(is_whitespace) {
                    self.bump_while(is_whitespace);
                    continue;
                }

                if self.lookahead.starts_with(';') {
                    self.bump_until(is_start_line_ending);
                    continue;
                }

                match &*self.lookahead {
                    "#|" => unimplemented!(), // nestable comment
                    "#;" => unimplemented!(), // datum comment
                    _ => {}
                }
            }

            return self.simple_lexemes(idx0)
                .or_else(|| self.boolean(idx0))
                .or_else(|| self.number(idx0))
                .or_else(|| self.identifier(idx0)) // Must come after `number` to resolve peculiar identifier ambiguities
                .or_else(|| self.character(idx0))
                .or_else(|| self.string(idx0))
                .or_else(|| Some(Err(Error::new(ErrorKind::UnrecognizedToken, idx0))));
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Spanned, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_unshifted() {
            None => None,
            Some(Ok((l, t, r))) => Some(Ok((l + self.shift, t, r + self.shift))),
            Some(Err(Error { location, kind })) => Some(Err(Error {
                location: location + self.shift,
                kind,
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_line_endings() {
        assert_eq!(detect_line_ending("\n"), Some(1));
        assert_eq!(detect_line_ending("\n\t"), Some(1));
        assert_eq!(detect_line_ending("\n "), Some(1));
    }

    #[test]
    fn letter_is_valid_constituent() {
        assert!(is_constituent('a'));
    }

    #[test]
    fn dash_is_valid_subsequent() {
        assert!(is_subsequent('-'));
    }

    #[test]
    fn is_constituent_not_true_for_dq() {
        assert!(!is_constituent('"'));
    }

    #[test]
    fn is_initial_not_true_for_dq() {
        assert!(!is_initial('"'));
    }

    #[test]
    fn simple() {
        use self::Token::*;

        let lexer = Lexer::new("()[]#(#vu8('`,,@.#'#`#,#,@", 0);
        assert_eq!(
            lexer
                .map(|r| r.map(|t| t.1))
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            vec![
                LParen,
                RParen,
                LBrack,
                RBrack,
                VectorOpen,
                ByteVectorOpen,
                Quote,
                QuasiQuote,
                Unquote,
                UnquoteSplicing,
                Dot,
                Syntax,
                QuasiSyntax,
                Unsyntax,
                UnsyntaxSplicing,
            ]
        );
    }

    #[test]
    fn string() {
        use self::Token::*;

        let plain_string = "this is a string";

        let multiline_string_raw = "this is a \\ \n\t  multiline string";
        let multiline_string_cooked = "this is a multiline string";

        let input = &[plain_string, multiline_string_raw]
            .iter()
            .map(|s| format!("\"{}\"", s))
            .join("\n");

        let lexer = Lexer::new(&input, 0);
        let tokens = match lexer
            .inspect(|t| println!("T-T-TOKENIZED: {:?}", t))
            .map(|r| r.map(|t| t.1))
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(tokens) => tokens,
            Err(err) => {
                let nearby = &input[err.location..];
                panic!("failed to lex, errored with {:?} on input of length {}:\n{}\nstarting from error position:\n{}", err, input.len(), input, nearby);
            }
        };
        assert_eq!(
            tokens,
            vec![
                String(Atom::from(plain_string)),
                String(Atom::from(multiline_string_cooked)),
            ]
        );
    }

    macro_rules! lex_test_files {
        (@test $name:ident $(#[$m:meta])* $test:ident) => {
            lex_test_files!(@test $name $(#[$m:meta])* $test where path = concat!(stringify!($name), "/", stringify!($test)));
        };
        (@test $name:ident $(#[$m:meta])* $test:ident where path = $path:expr) => {
            #[test]
            $(#[$m])*
            fn $test() {
                use std::{fs::File, io::Read, path::PathBuf};

                let string = {
                    let mut path = PathBuf::from("lexer-tests/");
                    path.push(String::from($path) + ".pn");
                    let mut file = File::open(path).unwrap();
                    let mut buf = String::new();
                    file.read_to_string(&mut buf).unwrap();
                    buf
                };

                let lexer = Lexer::new(&string, 0);
                let mut tokens_parsed = 0;

                for token_res in lexer {
                    match token_res {
                        Ok(token) => {
                            println!("Parsed token {} as {:?}", tokens_parsed, token);
                            tokens_parsed += 1;
                        },
                        Err(error) => panic!("{} tokens parsed before failure: {:?}", tokens_parsed, error),
                    }
                }
            }
        };
        ($( $test_name:ident { $($(#[$m:meta])* $test:ident $(where $opt:tt = $val:expr)*,)* } )*) => {
            $(
                mod $test_name {
                    use super::*;

                    $(lex_test_files!(@test $test_name $(#[$m])* $test $(where $opt = $val)*);)*
                }
            )*
        };
    }

    lex_test_files! {
        pass {
            hofstadter,
            string,
        }
    }
}
