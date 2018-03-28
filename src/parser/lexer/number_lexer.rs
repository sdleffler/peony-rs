use std::{num::ParseIntError, str::CharIndices};

use num::{BigUint, Num};

use parser::lexer::NumberFlags;

pub type Spanned<T> = (usize, T, usize);

#[derive(Debug)]
pub struct Error {
    pub location: usize,
    pub kind: ErrorKind,
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "expected NaN ('nan.0')")]
    ExpectedNan,

    #[fail(display = "expected Inf ('inf.0')")]
    ExpectedInf,

    #[fail(display = "didn't expect to find '{:?}' within a number", _0)]
    UnexpectedCharacter(char),

    #[fail(display = "couldn't parse digits: {}", _0)]
    ParseIntError(ParseIntError),

    #[fail(display = "couldn't parse digits: {}", _0)]
    ParseBigUintError(<BigUint as Num>::FromStrRadixErr),

    #[fail(display = "number too big to be an exponent: {}", _0)]
    ExponentTooBig(BigUint),
}

impl Error {
    pub fn new(kind: ErrorKind, location: usize) -> Self {
        Error { kind, location }
    }
}

#[derive(Debug)]
pub enum Tok<'input> {
    Digits(&'input str),

    Exponent,
    ExponentShort,
    ExponentFloat,
    ExponentDouble,
    ExponentLong,

    MantissaWidth(&'input str),

    Plus,
    Minus,

    Point,

    At,
    Slash,

    Nan,
    Inf,

    I,
}

pub struct Lexer<'input> {
    text: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    shift: usize,
    radix: u32,
}

impl<'input> Lexer<'input> {
    pub fn new(text: &'input str, shift: usize, radix: u32) -> Lexer<'input> {
        let mut t = Self {
            text,
            chars: text.char_indices(),
            lookahead: None,
            shift,
            radix,
        };
        t.bump();
        t
    }

    fn bump(&mut self) -> Option<(usize, char)> {
        self.lookahead = self.chars.next();
        self.lookahead
    }

    fn take_while<F>(&mut self, mut keep_going: F) -> Option<usize>
    where
        F: FnMut(char) -> bool,
    {
        self.take_until(|c| !keep_going(c))
    }

    fn take_until<F>(&mut self, mut terminate: F) -> Option<usize>
    where
        F: FnMut(char) -> bool,
    {
        loop {
            match self.lookahead {
                None => {
                    return None;
                }
                Some((idx1, c)) => {
                    if terminate(c) {
                        return Some(idx1);
                    } else {
                        self.bump();
                    }
                }
            }
        }
    }

    fn take(&mut self, n: usize) -> Option<usize> {
        if n > 0 {
            self.lookahead = self.chars.by_ref().skip(n - 1).last();
        }

        self.lookahead.map(|t| t.0)
    }

    fn next_unshifted(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        fn is_exponent_marker(c: char) -> bool {
            match c {
                'e' | 'E' | 's' | 'S' | 'f' | 'F' | 'l' | 'L' => true,
                _ => false,
            }
        }

        loop {
            return match self.lookahead {
                Some((idx0, '+')) => {
                    self.bump();
                    Some(Ok((idx0, Tok::Plus, idx0 + 1)))
                }
                Some((idx0, '-')) => {
                    self.bump();
                    Some(Ok((idx0, Tok::Minus, idx0 + 1)))
                }
                Some((idx0, '.')) => {
                    self.bump();
                    Some(Ok((idx0, Tok::Point, idx0 + 1)))
                }
                Some((idx0, '@')) => {
                    self.bump();
                    Some(Ok((idx0, Tok::At, idx0 + 1)))
                }
                Some((idx0, '/')) => {
                    self.bump();
                    Some(Ok((idx0, Tok::Slash, idx0 + 1)))
                }
                Some((idx0, 'i')) => {
                    self.bump();
                    Some(Ok((idx0, Tok::I, idx0 + 1)))
                }
                Some((idx0, '|')) => {
                    self.bump();
                    let s = match self.take_while(|c| c.is_digit(10)) {
                        Some(idx1) => {
                            self.bump();
                            &self.text[idx0..idx1]
                        }
                        None => &self.text[idx0..],
                    };

                    Some(Ok((idx0, Tok::MantissaWidth(s), idx0 + s.len())))
                }
                Some((idx0, 'n')) => {
                    let idx1 = self.take(5).unwrap_or(self.text.len());

                    if &self.text[idx0..idx1] == "nan.0" {
                        Some(Ok((idx0, Tok::Nan, idx1)))
                    } else {
                        Some(Err(Error::new(ErrorKind::ExpectedNan, idx0)))
                    }
                }
                Some((idx0, 'i')) => {
                    let idx1 = self.take(5).unwrap_or(self.text.len());

                    if &self.text[idx0..idx1] == "inf.0" {
                        Some(Ok((idx0, Tok::Inf, idx1)))
                    } else {
                        Some(Err(Error::new(ErrorKind::ExpectedInf, idx0)))
                    }
                }
                Some((idx0, c)) if c.is_digit(self.radix) => {
                    let r = self.radix;
                    let s = match self.take_while(|c| c.is_digit(r)) {
                        Some(idx1) => {
                            self.bump();
                            &self.text[idx0..idx1]
                        }
                        None => &self.text[idx0..],
                    };

                    Some(Ok((idx0, Tok::Digits(s), idx0 + s.len())))
                }
                Some((idx0, c)) if self.radix == 10 && is_exponent_marker(c) => {
                    let tok = match c {
                        'e' | 'E' => Tok::Exponent,
                        's' | 'S' => Tok::ExponentShort,
                        'f' | 'F' => Tok::ExponentFloat,
                        'd' | 'D' => Tok::ExponentDouble,
                        'l' | 'L' => Tok::ExponentLong,
                        _ => unreachable!(),
                    };

                    Some(Ok((idx0, tok, idx0 + 1)))
                }
                Some((idx0, c)) => Some(Err(Error::new(ErrorKind::UnexpectedCharacter(c), idx0))),
                None => None,
            };
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Spanned<Tok<'input>>, Error>;

    fn next(&mut self) -> Option<Result<Spanned<Tok<'input>>, Error>> {
        match self.next_unshifted() {
            None => None,
            Some(Ok((l, t, r))) => Some(Ok((l + self.shift, t, r + self.shift))),
            Some(Err(Error { location, kind })) => Some(Err(Error {
                location: location + self.shift,
                kind: kind,
            })),
        }
    }
}
