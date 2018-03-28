// auto-generated: "lalrpop 0.15.0"
#![cfg_attr(rustfmt, rustfmt_skip)]
use failure::*;
use lalrpop_util::ParseError;
use num::{BigInt, BigUint, BigRational, FromPrimitive, Num, One, ToPrimitive, Zero, pow::pow};
use ast::{Number, Precision, Sign, NanInf, UReal, Real};
use parser::lexer::NumberFlags;
use parser::lexer::number_lexer::{self, Error, ErrorKind};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Number {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use failure::*;
    use lalrpop_util::ParseError;
    use num::{BigInt, BigUint, BigRational, FromPrimitive, Num, One, ToPrimitive, Zero, pow::pow};
    use ast::{Number, Precision, Sign, NanInf, UReal, Real};
    use parser::lexer::NumberFlags;
    use parser::lexer::number_lexer::{self, Error, ErrorKind};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(number_lexer::Tok<'input>),
        Variant1(&'input str),
        Variant2(::std::option::Option<&'input str>),
        Variant3(BigUint),
        Variant4(::std::option::Option<BigUint>),
        Variant5(usize),
        Variant6((Precision, &'input str, &'input str, isize)),
        Variant7(Precision),
        Variant8(NanInf),
        Variant9(Number),
        Variant10(Real),
        Variant11(Sign),
        Variant12(::std::option::Option<Sign>),
        Variant13((usize, &'input str, usize)),
        Variant14((usize, Number, usize)),
        Variant15((Precision, isize)),
        Variant16(::std::option::Option<(Precision, isize)>),
        Variant17(UReal),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0,
        // State 1
        -48, -48, 0, 0, -48, 0, 0, 0, 0, 0, 0, -48, 0, 12, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        8, 9, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 11, 17, 18, 0, 19,
        // State 5
        -46, -46, 0, 20, -46, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0,
        // State 6
        -31, -31, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, -33, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, 0, -33,
        // State 8
        0, 0, -34, 0, 0, 0, 0, 0, 0, 0, -34, -34, -34, 0, -34,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0,
        // State 10
        -44, -44, 24, -44, -44, 25, 26, 27, 28, 29, 0, -44, 0, 0, 0,
        // State 11
        -47, -47, 0, 0, -47, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0,
        // State 12
        0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 11, 32, 18, 0, 19,
        // State 13
        8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0,
        // State 14
        -32, -32, 0, 0, -32, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0,
        // State 15
        -30, -30, 0, 0, -30, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        -21, -21, 0, 0, -21, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0,
        // State 18
        -20, -20, 0, 0, -20, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0,
        // State 20
        -10, -10, 0, 0, -10, 25, 26, 27, 28, 29, 0, -10, 0, -10, 0,
        // State 21
        8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0,
        // State 22
        -8, -8, 0, 0, -8, 0, 0, 0, 0, 0, 0, -8, 0, -8, 0,
        // State 23
        -12, -12, 0, 0, -12, 25, 26, 27, 28, 29, 44, -12, 0, -12, 0,
        // State 24
        -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0,
        // State 25
        -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0,
        // State 26
        -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 27
        -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0,
        // State 28
        -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 18, 0, 19,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        -45, -45, 0, 0, -45, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0,
        // State 37
        -44, -44, 0, 0, -44, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0,
        // State 38
        -9, -9, 0, 0, -9, 0, 0, 0, 0, 0, 0, -9, 0, -9, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0,
        // State 40
        -41, -41, 0, 0, -41, 0, 0, 0, 0, 0, 0, -41, 0, -41, 0,
        // State 41
        -37, -37, 0, 0, -37, 0, 0, 0, 0, 0, 0, -37, 0, -37, 0,
        // State 42
        -11, -11, 0, 0, -11, 0, 0, 0, 0, 0, 0, -11, 0, -11, 0,
        // State 43
        -14, -14, 0, 0, -14, 25, 26, 27, 28, 29, 0, -14, 0, -14, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        -40, -40, 0, 0, -40, 0, 0, 0, 0, 0, 0, -40, 0, -40, 0,
        // State 49
        -13, -13, 0, 0, -13, 0, 0, 0, 0, 0, 0, -13, 0, -13, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -48,
        // State 2
        -49,
        // State 3
        -22,
        // State 4
        0,
        // State 5
        -46,
        // State 6
        -31,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        -44,
        // State 11
        -47,
        // State 12
        0,
        // State 13
        0,
        // State 14
        -32,
        // State 15
        -30,
        // State 16
        -29,
        // State 17
        -21,
        // State 18
        -20,
        // State 19
        0,
        // State 20
        -10,
        // State 21
        0,
        // State 22
        -8,
        // State 23
        -12,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        -26,
        // State 32
        -23,
        // State 33
        0,
        // State 34
        -28,
        // State 35
        -27,
        // State 36
        -45,
        // State 37
        -44,
        // State 38
        -9,
        // State 39
        0,
        // State 40
        -41,
        // State 41
        -37,
        // State 42
        -11,
        // State 43
        -14,
        // State 44
        -25,
        // State 45
        -24,
        // State 46
        -32,
        // State 47
        -30,
        // State 48
        -40,
        // State 49
        -13,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 2, 0, 0, 3, 4, 5, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 2, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 16, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 2, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 31, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 2, 0, 0, 0, 33, 34, 0, 0, 0, 0, 0, 0, 6, 7, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 2, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 48, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""+""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###""@""###,
            r###""D""###,
            r###""E""###,
            r###""F""###,
            r###""L""###,
            r###""S""###,
            r###""digits""###,
            r###""i""###,
            r###""inf.0""###,
            r###""mantissa width""###,
            r###""nan.0""###,
        ];
        __ACTION[(__state * 15)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct NumberParser {
        _priv: (),
    }

    impl NumberParser {
        pub fn new() -> NumberParser {
            NumberParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=number_lexer::Error>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            flags: &NumberFlags,
            __tokens0: __TOKENS,
        ) -> Result<Number, __lalrpop_util::ParseError<usize, number_lexer::Tok<'input>, number_lexer::Error>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    number_lexer::Tok::Plus if true => 0,
                    number_lexer::Tok::Minus if true => 1,
                    number_lexer::Tok::Point if true => 2,
                    number_lexer::Tok::Slash if true => 3,
                    number_lexer::Tok::At if true => 4,
                    number_lexer::Tok::ExponentDouble if true => 5,
                    number_lexer::Tok::Exponent if true => 6,
                    number_lexer::Tok::ExponentFloat if true => 7,
                    number_lexer::Tok::ExponentLong if true => 8,
                    number_lexer::Tok::ExponentShort if true => 9,
                    number_lexer::Tok::Digits(_) if true => 10,
                    number_lexer::Tok::I if true => 11,
                    number_lexer::Tok::Inf if true => 12,
                    number_lexer::Tok::MantissaWidth(_) if true => 13,
                    number_lexer::Tok::Nan if true => 14,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 15 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Point => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Slash => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::At => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::ExponentDouble => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Exponent => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::ExponentFloat => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::ExponentLong => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::ExponentShort => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                number_lexer::Tok::Digits(__tok0) => __Symbol::Variant1((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::I => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Inf => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                number_lexer::Tok::MantissaWidth(__tok0) => __Symbol::Variant1((__tok0)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Nan => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(flags, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(flags, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        flags: &NumberFlags,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Number,__lalrpop_util::ParseError<usize, number_lexer::Tok<'input>, number_lexer::Error>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                // "mantissa width"? = "mantissa width" => ActionFn(33);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 0)
            }
            2 => {
                // "mantissa width"? =  => ActionFn(34);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action34::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (0, __symbol, 0)
            }
            3 => {
                // ("/" <UInteger>) = "/", UInteger => ActionFn(38);
                let __sym1 = __pop_Variant3(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 1)
            }
            4 => {
                // ("/" <UInteger>)? = "/", UInteger => ActionFn(45);
                let __sym1 = __pop_Variant3(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action45::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (2, __symbol, 2)
            }
            5 => {
                // ("/" <UInteger>)? =  => ActionFn(37);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action37::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (0, __symbol, 2)
            }
            6 => {
                // @L =  => ActionFn(35);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action35::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                (0, __symbol, 3)
            }
            7 => {
                // @R =  => ActionFn(42);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action42::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                (0, __symbol, 4)
            }
            8 => {
                // Decimal = "digits", Suffix => ActionFn(16);
                let __sym1 = __pop_Variant15(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (2, __symbol, 5)
            }
            9 => {
                // Decimal = ".", "digits", Suffix => ActionFn(59);
                let __sym2 = __pop_Variant15(__symbols);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action59::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (3, __symbol, 5)
            }
            10 => {
                // Decimal = ".", "digits" => ActionFn(60);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action60::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (2, __symbol, 5)
            }
            11 => {
                // Decimal = "digits", ".", Suffix => ActionFn(61);
                let __sym2 = __pop_Variant15(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action61::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (3, __symbol, 5)
            }
            12 => {
                // Decimal = "digits", "." => ActionFn(62);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action62::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (2, __symbol, 5)
            }
            13 => {
                // Decimal = "digits", ".", "digits", Suffix => ActionFn(63);
                let __sym3 = __pop_Variant15(__symbols);
                let __sym2 = __pop_Variant1(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action63::<>(flags, __sym0, __sym1, __sym2, __sym3);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (4, __symbol, 5)
            }
            14 => {
                // Decimal = "digits", ".", "digits" => ActionFn(64);
                let __sym2 = __pop_Variant1(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action64::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (3, __symbol, 5)
            }
            15 => {
                // ExponentMarker = "E" => ActionFn(21);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 6)
            }
            16 => {
                // ExponentMarker = "S" => ActionFn(22);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 6)
            }
            17 => {
                // ExponentMarker = "F" => ActionFn(23);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 6)
            }
            18 => {
                // ExponentMarker = "D" => ActionFn(24);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 6)
            }
            19 => {
                // ExponentMarker = "L" => ActionFn(25);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 6)
            }
            20 => {
                // NanInf = "nan.0" => ActionFn(28);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant8(__nt), __end);
                (1, __symbol, 7)
            }
            21 => {
                // NanInf = "inf.0" => ActionFn(29);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant8(__nt), __end);
                (1, __symbol, 7)
            }
            22 => {
                // Number = Real => ActionFn(2);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (1, __symbol, 8)
            }
            23 => {
                // Number = Real, "@", Real => ActionFn(3);
                let __sym2 = __pop_Variant10(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (3, __symbol, 8)
            }
            24 => {
                // Number = Real, Sign, UReal, "i" => ActionFn(4);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant17(__symbols);
                let __sym1 = __pop_Variant11(__symbols);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action4::<>(flags, __sym0, __sym1, __sym2, __sym3);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (4, __symbol, 8)
            }
            25 => {
                // Number = Real, Sign, NanInf, "i" => ActionFn(5);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant8(__symbols);
                let __sym1 = __pop_Variant11(__symbols);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5::<>(flags, __sym0, __sym1, __sym2, __sym3);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (4, __symbol, 8)
            }
            26 => {
                // Number = Real, Sign, "i" => ActionFn(6);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant11(__symbols);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (3, __symbol, 8)
            }
            27 => {
                // Number = Sign, UReal, "i" => ActionFn(7);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant17(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (3, __symbol, 8)
            }
            28 => {
                // Number = Sign, NanInf, "i" => ActionFn(8);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant8(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (3, __symbol, 8)
            }
            29 => {
                // Number = Sign, "i" => ActionFn(9);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (2, __symbol, 8)
            }
            30 => {
                // Real = Sign, UReal => ActionFn(55);
                let __sym1 = __pop_Variant17(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant10(__nt), __end);
                (2, __symbol, 9)
            }
            31 => {
                // Real = UReal => ActionFn(56);
                let __sym0 = __pop_Variant17(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action56::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant10(__nt), __end);
                (1, __symbol, 9)
            }
            32 => {
                // Real = Sign, NanInf => ActionFn(12);
                let __sym1 = __pop_Variant8(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant10(__nt), __end);
                (2, __symbol, 9)
            }
            33 => {
                // Sign = "+" => ActionFn(26);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant11(__nt), __end);
                (1, __symbol, 10)
            }
            34 => {
                // Sign = "-" => ActionFn(27);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant11(__nt), __end);
                (1, __symbol, 10)
            }
            35 => {
                // Sign? = Sign => ActionFn(39);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant12(__nt), __end);
                (1, __symbol, 11)
            }
            36 => {
                // Sign? =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant12(__nt), __end);
                (0, __symbol, 11)
            }
            37 => {
                // Sp<"digits"> = "digits" => ActionFn(53);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant13(__nt), __end);
                (1, __symbol, 12)
            }
            38 => {
                // Sp<Number> = Number => ActionFn(54);
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action54::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant14(__nt), __end);
                (1, __symbol, 13)
            }
            39 => {
                // SpannedNumber = Sp<Number> => ActionFn(10);
                let __sym0 = __pop_Variant14(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant14(__nt), __end);
                (1, __symbol, 14)
            }
            40 => {
                // Suffix = ExponentMarker, Sign, Sp<"digits"> => ActionFn(57);
                let __sym2 = __pop_Variant13(__symbols);
                let __sym1 = __pop_Variant11(__symbols);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action57::<>(flags, __sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant15(__nt), __end);
                (3, __symbol, 15)
            }
            41 => {
                // Suffix = ExponentMarker, Sp<"digits"> => ActionFn(58);
                let __sym1 = __pop_Variant13(__symbols);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action58::<>(flags, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant15(__nt), __end);
                (2, __symbol, 15)
            }
            42 => {
                // Suffix? = Suffix => ActionFn(31);
                let __sym0 = __pop_Variant15(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant16(__nt), __end);
                (1, __symbol, 16)
            }
            43 => {
                // Suffix? =  => ActionFn(32);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action32::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant16(__nt), __end);
                (0, __symbol, 16)
            }
            44 => {
                // UInteger = "digits" => ActionFn(50);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action50::<>(flags, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (1, __symbol, 17)
            }
            45 => {
                // UReal = UInteger, "/", UInteger => ActionFn(46);
                let __sym2 = __pop_Variant3(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action46::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant17(__nt), __end);
                (3, __symbol, 18)
            }
            46 => {
                // UReal = UInteger => ActionFn(47);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant17(__nt), __end);
                (1, __symbol, 18)
            }
            47 => {
                // UReal = Decimal, "mantissa width" => ActionFn(51);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action51::<>(flags, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant17(__nt), __end);
                (2, __symbol, 18)
            }
            48 => {
                // UReal = Decimal => ActionFn(52);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action52::<>(flags, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant17(__nt), __end);
                (1, __symbol, 18)
            }
            49 => {
                // __Number = Number => ActionFn(0);
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(flags, __sym0);
                return Some(Ok(__nt));
            }
            50 => {
                // __SpannedNumber = SpannedNumber => ActionFn(1);
                let __sym0 = __pop_Variant14(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant14(__nt), __end);
                (1, __symbol, 20)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 21 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Precision, isize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Precision, &'input str, &'input str, isize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (usize, Number, usize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (usize, &'input str, usize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BigUint, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NanInf, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Number, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Precision, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Real, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Sign, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UReal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, number_lexer::Tok<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(Precision, isize)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<BigUint>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Sign>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<&'input str>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Number::NumberParser;

mod __parse__SpannedNumber {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use failure::*;
    use lalrpop_util::ParseError;
    use num::{BigInt, BigUint, BigRational, FromPrimitive, Num, One, ToPrimitive, Zero, pow::pow};
    use ast::{Number, Precision, Sign, NanInf, UReal, Real};
    use parser::lexer::NumberFlags;
    use parser::lexer::number_lexer::{self, Error, ErrorKind};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(number_lexer::Tok<'input>),
        Variant1(&'input str),
        Variant2(::std::option::Option<&'input str>),
        Variant3(BigUint),
        Variant4(::std::option::Option<BigUint>),
        Variant5(usize),
        Variant6((Precision, &'input str, &'input str, isize)),
        Variant7(Precision),
        Variant8(NanInf),
        Variant9(Number),
        Variant10(Real),
        Variant11(Sign),
        Variant12(::std::option::Option<Sign>),
        Variant13((usize, &'input str, usize)),
        Variant14((usize, Number, usize)),
        Variant15((Precision, isize)),
        Variant16(::std::option::Option<(Precision, isize)>),
        Variant17(UReal),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0,
        // State 1
        -48, -48, 0, 0, -48, 0, 0, 0, 0, 0, 0, -48, 0, 14, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        10, 11, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 13, 19, 20, 0, 21,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        -46, -46, 0, 22, -46, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0,
        // State 8
        -31, -31, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, -33, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, 0, -33,
        // State 10
        0, 0, -34, 0, 0, 0, 0, 0, 0, 0, -34, -34, -34, 0, -34,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0,
        // State 12
        -44, -44, 26, -44, -44, 27, 28, 29, 30, 31, 0, -44, 0, 0, 0,
        // State 13
        -47, -47, 0, 0, -47, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0,
        // State 14
        0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 13, 34, 20, 0, 21,
        // State 15
        10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0,
        // State 16
        -32, -32, 0, 0, -32, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0,
        // State 17
        -30, -30, 0, 0, -30, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        -21, -21, 0, 0, -21, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0,
        // State 20
        -20, -20, 0, 0, -20, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0,
        // State 22
        -10, -10, 0, 0, -10, 27, 28, 29, 30, 31, 0, -10, 0, -10, 0,
        // State 23
        10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0,
        // State 24
        -8, -8, 0, 0, -8, 0, 0, 0, 0, 0, 0, -8, 0, -8, 0,
        // State 25
        -12, -12, 0, 0, -12, 27, 28, 29, 30, 31, 46, -12, 0, -12, 0,
        // State 26
        -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0,
        // State 27
        -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0,
        // State 28
        -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 29
        -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0,
        // State 30
        -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 13, 0, 20, 0, 21,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        -45, -45, 0, 0, -45, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0,
        // State 39
        -44, -44, 0, 0, -44, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0,
        // State 40
        -9, -9, 0, 0, -9, 0, 0, 0, 0, 0, 0, -9, 0, -9, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0,
        // State 42
        -41, -41, 0, 0, -41, 0, 0, 0, 0, 0, 0, -41, 0, -41, 0,
        // State 43
        -37, -37, 0, 0, -37, 0, 0, 0, 0, 0, 0, -37, 0, -37, 0,
        // State 44
        -11, -11, 0, 0, -11, 0, 0, 0, 0, 0, 0, -11, 0, -11, 0,
        // State 45
        -14, -14, 0, 0, -14, 27, 28, 29, 30, 31, 0, -14, 0, -14, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        -40, -40, 0, 0, -40, 0, 0, 0, 0, 0, 0, -40, 0, -40, 0,
        // State 51
        -13, -13, 0, 0, -13, 0, 0, 0, 0, 0, 0, -13, 0, -13, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -48,
        // State 2
        -38,
        // State 3
        -22,
        // State 4
        0,
        // State 5
        -39,
        // State 6
        -50,
        // State 7
        -46,
        // State 8
        -31,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        -44,
        // State 13
        -47,
        // State 14
        0,
        // State 15
        0,
        // State 16
        -32,
        // State 17
        -30,
        // State 18
        -29,
        // State 19
        -21,
        // State 20
        -20,
        // State 21
        0,
        // State 22
        -10,
        // State 23
        0,
        // State 24
        -8,
        // State 25
        -12,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        -26,
        // State 34
        -23,
        // State 35
        0,
        // State 36
        -28,
        // State 37
        -27,
        // State 38
        -45,
        // State 39
        -44,
        // State 40
        -9,
        // State 41
        0,
        // State 42
        -41,
        // State 43
        -37,
        // State 44
        -11,
        // State 45
        -14,
        // State 46
        -25,
        // State 47
        -24,
        // State 48
        -32,
        // State 49
        -30,
        // State 50
        -40,
        // State 51
        -13,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 2, 0, 0, 3, 4, 5, 0, 0, 6, 7, 0, 0, 8, 9, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 2, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 18, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 2, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 33, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 2, 0, 0, 0, 35, 36, 0, 0, 0, 0, 0, 0, 8, 9, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 2, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 50, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""+""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###""@""###,
            r###""D""###,
            r###""E""###,
            r###""F""###,
            r###""L""###,
            r###""S""###,
            r###""digits""###,
            r###""i""###,
            r###""inf.0""###,
            r###""mantissa width""###,
            r###""nan.0""###,
        ];
        __ACTION[(__state * 15)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct SpannedNumberParser {
        _priv: (),
    }

    impl SpannedNumberParser {
        pub fn new() -> SpannedNumberParser {
            SpannedNumberParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, Error=number_lexer::Error>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            flags: &NumberFlags,
            __tokens0: __TOKENS,
        ) -> Result<(usize, Number, usize), __lalrpop_util::ParseError<usize, number_lexer::Tok<'input>, number_lexer::Error>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    number_lexer::Tok::Plus if true => 0,
                    number_lexer::Tok::Minus if true => 1,
                    number_lexer::Tok::Point if true => 2,
                    number_lexer::Tok::Slash if true => 3,
                    number_lexer::Tok::At if true => 4,
                    number_lexer::Tok::ExponentDouble if true => 5,
                    number_lexer::Tok::Exponent if true => 6,
                    number_lexer::Tok::ExponentFloat if true => 7,
                    number_lexer::Tok::ExponentLong if true => 8,
                    number_lexer::Tok::ExponentShort if true => 9,
                    number_lexer::Tok::Digits(_) if true => 10,
                    number_lexer::Tok::I if true => 11,
                    number_lexer::Tok::Inf if true => 12,
                    number_lexer::Tok::MantissaWidth(_) if true => 13,
                    number_lexer::Tok::Nan if true => 14,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 15 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Point => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Slash => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::At => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::ExponentDouble => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Exponent => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::ExponentFloat => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::ExponentLong => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::ExponentShort => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                number_lexer::Tok::Digits(__tok0) => __Symbol::Variant1((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::I => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Inf => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                number_lexer::Tok::MantissaWidth(__tok0) => __Symbol::Variant1((__tok0)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ number_lexer::Tok::Nan => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(flags, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(flags, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        flags: &NumberFlags,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<(usize, Number, usize),__lalrpop_util::ParseError<usize, number_lexer::Tok<'input>, number_lexer::Error>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                // "mantissa width"? = "mantissa width" => ActionFn(33);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 0)
            }
            2 => {
                // "mantissa width"? =  => ActionFn(34);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action34::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (0, __symbol, 0)
            }
            3 => {
                // ("/" <UInteger>) = "/", UInteger => ActionFn(38);
                let __sym1 = __pop_Variant3(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 1)
            }
            4 => {
                // ("/" <UInteger>)? = "/", UInteger => ActionFn(45);
                let __sym1 = __pop_Variant3(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action45::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (2, __symbol, 2)
            }
            5 => {
                // ("/" <UInteger>)? =  => ActionFn(37);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action37::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (0, __symbol, 2)
            }
            6 => {
                // @L =  => ActionFn(35);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action35::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                (0, __symbol, 3)
            }
            7 => {
                // @R =  => ActionFn(42);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action42::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                (0, __symbol, 4)
            }
            8 => {
                // Decimal = "digits", Suffix => ActionFn(16);
                let __sym1 = __pop_Variant15(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (2, __symbol, 5)
            }
            9 => {
                // Decimal = ".", "digits", Suffix => ActionFn(59);
                let __sym2 = __pop_Variant15(__symbols);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action59::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (3, __symbol, 5)
            }
            10 => {
                // Decimal = ".", "digits" => ActionFn(60);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action60::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (2, __symbol, 5)
            }
            11 => {
                // Decimal = "digits", ".", Suffix => ActionFn(61);
                let __sym2 = __pop_Variant15(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action61::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (3, __symbol, 5)
            }
            12 => {
                // Decimal = "digits", "." => ActionFn(62);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action62::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (2, __symbol, 5)
            }
            13 => {
                // Decimal = "digits", ".", "digits", Suffix => ActionFn(63);
                let __sym3 = __pop_Variant15(__symbols);
                let __sym2 = __pop_Variant1(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action63::<>(flags, __sym0, __sym1, __sym2, __sym3);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (4, __symbol, 5)
            }
            14 => {
                // Decimal = "digits", ".", "digits" => ActionFn(64);
                let __sym2 = __pop_Variant1(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action64::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (3, __symbol, 5)
            }
            15 => {
                // ExponentMarker = "E" => ActionFn(21);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 6)
            }
            16 => {
                // ExponentMarker = "S" => ActionFn(22);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 6)
            }
            17 => {
                // ExponentMarker = "F" => ActionFn(23);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 6)
            }
            18 => {
                // ExponentMarker = "D" => ActionFn(24);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 6)
            }
            19 => {
                // ExponentMarker = "L" => ActionFn(25);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 6)
            }
            20 => {
                // NanInf = "nan.0" => ActionFn(28);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant8(__nt), __end);
                (1, __symbol, 7)
            }
            21 => {
                // NanInf = "inf.0" => ActionFn(29);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant8(__nt), __end);
                (1, __symbol, 7)
            }
            22 => {
                // Number = Real => ActionFn(2);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (1, __symbol, 8)
            }
            23 => {
                // Number = Real, "@", Real => ActionFn(3);
                let __sym2 = __pop_Variant10(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (3, __symbol, 8)
            }
            24 => {
                // Number = Real, Sign, UReal, "i" => ActionFn(4);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant17(__symbols);
                let __sym1 = __pop_Variant11(__symbols);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action4::<>(flags, __sym0, __sym1, __sym2, __sym3);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (4, __symbol, 8)
            }
            25 => {
                // Number = Real, Sign, NanInf, "i" => ActionFn(5);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant8(__symbols);
                let __sym1 = __pop_Variant11(__symbols);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5::<>(flags, __sym0, __sym1, __sym2, __sym3);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (4, __symbol, 8)
            }
            26 => {
                // Number = Real, Sign, "i" => ActionFn(6);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant11(__symbols);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (3, __symbol, 8)
            }
            27 => {
                // Number = Sign, UReal, "i" => ActionFn(7);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant17(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (3, __symbol, 8)
            }
            28 => {
                // Number = Sign, NanInf, "i" => ActionFn(8);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant8(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (3, __symbol, 8)
            }
            29 => {
                // Number = Sign, "i" => ActionFn(9);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (2, __symbol, 8)
            }
            30 => {
                // Real = Sign, UReal => ActionFn(55);
                let __sym1 = __pop_Variant17(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant10(__nt), __end);
                (2, __symbol, 9)
            }
            31 => {
                // Real = UReal => ActionFn(56);
                let __sym0 = __pop_Variant17(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action56::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant10(__nt), __end);
                (1, __symbol, 9)
            }
            32 => {
                // Real = Sign, NanInf => ActionFn(12);
                let __sym1 = __pop_Variant8(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant10(__nt), __end);
                (2, __symbol, 9)
            }
            33 => {
                // Sign = "+" => ActionFn(26);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant11(__nt), __end);
                (1, __symbol, 10)
            }
            34 => {
                // Sign = "-" => ActionFn(27);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant11(__nt), __end);
                (1, __symbol, 10)
            }
            35 => {
                // Sign? = Sign => ActionFn(39);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant12(__nt), __end);
                (1, __symbol, 11)
            }
            36 => {
                // Sign? =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant12(__nt), __end);
                (0, __symbol, 11)
            }
            37 => {
                // Sp<"digits"> = "digits" => ActionFn(53);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant13(__nt), __end);
                (1, __symbol, 12)
            }
            38 => {
                // Sp<Number> = Number => ActionFn(54);
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action54::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant14(__nt), __end);
                (1, __symbol, 13)
            }
            39 => {
                // SpannedNumber = Sp<Number> => ActionFn(10);
                let __sym0 = __pop_Variant14(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant14(__nt), __end);
                (1, __symbol, 14)
            }
            40 => {
                // Suffix = ExponentMarker, Sign, Sp<"digits"> => ActionFn(57);
                let __sym2 = __pop_Variant13(__symbols);
                let __sym1 = __pop_Variant11(__symbols);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action57::<>(flags, __sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant15(__nt), __end);
                (3, __symbol, 15)
            }
            41 => {
                // Suffix = ExponentMarker, Sp<"digits"> => ActionFn(58);
                let __sym1 = __pop_Variant13(__symbols);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action58::<>(flags, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant15(__nt), __end);
                (2, __symbol, 15)
            }
            42 => {
                // Suffix? = Suffix => ActionFn(31);
                let __sym0 = __pop_Variant15(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant16(__nt), __end);
                (1, __symbol, 16)
            }
            43 => {
                // Suffix? =  => ActionFn(32);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action32::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant16(__nt), __end);
                (0, __symbol, 16)
            }
            44 => {
                // UInteger = "digits" => ActionFn(50);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action50::<>(flags, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (1, __symbol, 17)
            }
            45 => {
                // UReal = UInteger, "/", UInteger => ActionFn(46);
                let __sym2 = __pop_Variant3(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action46::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant17(__nt), __end);
                (3, __symbol, 18)
            }
            46 => {
                // UReal = UInteger => ActionFn(47);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action47::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant17(__nt), __end);
                (1, __symbol, 18)
            }
            47 => {
                // UReal = Decimal, "mantissa width" => ActionFn(51);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action51::<>(flags, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant17(__nt), __end);
                (2, __symbol, 18)
            }
            48 => {
                // UReal = Decimal => ActionFn(52);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action52::<>(flags, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant17(__nt), __end);
                (1, __symbol, 18)
            }
            49 => {
                // __Number = Number => ActionFn(0);
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (1, __symbol, 19)
            }
            50 => {
                // __SpannedNumber = SpannedNumber => ActionFn(1);
                let __sym0 = __pop_Variant14(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(flags, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 21 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Precision, isize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Precision, &'input str, &'input str, isize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (usize, Number, usize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (usize, &'input str, usize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BigUint, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, NanInf, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Number, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Precision, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Real, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Sign, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant17<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UReal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, number_lexer::Tok<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(Precision, isize)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<BigUint>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Sign>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<&'input str>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__SpannedNumber::SpannedNumberParser;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, Number, usize),
) -> Number
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, (usize, Number, usize), usize),
) -> (usize, Number, usize)
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, Real, usize),
) -> Number
{
    Number::Real(__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, Real, usize),
    (_, _, _): (usize, number_lexer::Tok<'input>, usize),
    (_, __1, _): (usize, Real, usize),
) -> Number
{
    Number::Complex(__0, __1)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    flags: &NumberFlags,
    (_, a, _): (usize, Real, usize),
    (_, s, _): (usize, Sign, usize),
    (_, b, _): (usize, UReal, usize),
    (_, _, _): (usize, number_lexer::Tok<'input>, usize),
) -> Number
{
    Number::Complex(a, Real::Signed(s, b))
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    flags: &NumberFlags,
    (_, a, _): (usize, Real, usize),
    (_, s, _): (usize, Sign, usize),
    (_, b, _): (usize, NanInf, usize),
    (_, _, _): (usize, number_lexer::Tok<'input>, usize),
) -> Number
{
    Number::Complex(a, Real::NanInf(s, b))
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    flags: &NumberFlags,
    (_, a, _): (usize, Real, usize),
    (_, s, _): (usize, Sign, usize),
    (_, _, _): (usize, number_lexer::Tok<'input>, usize),
) -> Number
{
    Number::Complex(a, Real::Signed(s, UReal::Integer(BigUint::one())))
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    flags: &NumberFlags,
    (_, s, _): (usize, Sign, usize),
    (_, b, _): (usize, UReal, usize),
    (_, _, _): (usize, number_lexer::Tok<'input>, usize),
) -> Number
{
    Number::Complex(Real::Signed(Sign::Pos, UReal::Integer(BigUint::zero())), Real::Signed(s, b))
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    flags: &NumberFlags,
    (_, s, _): (usize, Sign, usize),
    (_, b, _): (usize, NanInf, usize),
    (_, _, _): (usize, number_lexer::Tok<'input>, usize),
) -> Number
{
    Number::Complex(Real::Signed(Sign::Pos, UReal::Integer(BigUint::zero())), Real::NanInf(s, b))
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    flags: &NumberFlags,
    (_, s, _): (usize, Sign, usize),
    (_, _, _): (usize, number_lexer::Tok<'input>, usize),
) -> Number
{
    Number::Complex(Real::Signed(Sign::Pos, UReal::Integer(BigUint::zero())), Real::Signed(s, UReal::Integer(BigUint::one())))
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, (usize, Number, usize), usize),
) -> (usize, Number, usize)
{
    __0
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    flags: &NumberFlags,
    (_, s, _): (usize, ::std::option::Option<Sign>, usize),
    (_, u, _): (usize, UReal, usize),
) -> Real
{
    Real::Signed(s.unwrap_or_default(), u)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    flags: &NumberFlags,
    (_, s, _): (usize, Sign, usize),
    (_, n, _): (usize, NanInf, usize),
) -> Real
{
    Real::NanInf(s, n)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    flags: &NumberFlags,
    (_, n, _): (usize, BigUint, usize),
    (_, d, _): (usize, ::std::option::Option<BigUint>, usize),
) -> UReal
{
    match d {
        Some(d) => UReal::Rational(n, d),
        None => UReal::Integer(n),
    }
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    flags: &NumberFlags,
    (_, loc, _): (usize, usize, usize),
    (_, d, _): (usize, (Precision, &'input str, &'input str, isize), usize),
    (_, m, _): (usize, ::std::option::Option<&'input str>, usize),
) -> Result<UReal,__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    {
        if let Some(_) = m {
            // TODO warn that mantissa width is ignored
        }

        let (p, i, m, e) = d;
        let n = BigUint::from_str_radix(&(i.to_owned() + m), 10)
            .map_err(|e| ParseError::User { error: Error::new(ErrorKind::ParseBigUintError(e), loc) })?;
        let e_offset = m.trim_right_matches('0').len() as isize;

        Ok(UReal::Decimal(p, n, e - e_offset))
    }
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    flags: &NumberFlags,
    (_, loc, _): (usize, usize, usize),
    (_, d, _): (usize, &'input str, usize),
) -> Result<BigUint,__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    BigUint::from_str_radix(d, flags.to_radix())
    .map_err(|e| ParseError::User { error: Error::new(ErrorKind::ParseBigUintError(e), loc) })
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    flags: &NumberFlags,
    (_, d, _): (usize, &'input str, usize),
    (_, s, _): (usize, (Precision, isize), usize),
) -> (Precision, &'input str, &'input str, isize)
{
    (s.0, d, "", s.1)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    flags: &NumberFlags,
    (_, _, _): (usize, number_lexer::Tok<'input>, usize),
    (_, d, _): (usize, &'input str, usize),
    (_, s, _): (usize, ::std::option::Option<(Precision, isize)>, usize),
) -> (Precision, &'input str, &'input str, isize)
{
    { let s = s.unwrap_or_default(); (s.0, "", d, s.1) }
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    flags: &NumberFlags,
    (_, d, _): (usize, &'input str, usize),
    (_, _, _): (usize, number_lexer::Tok<'input>, usize),
    (_, s, _): (usize, ::std::option::Option<(Precision, isize)>, usize),
) -> (Precision, &'input str, &'input str, isize)
{
    { let s = s.unwrap_or_default(); (s.0, d, "", s.1) }
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    flags: &NumberFlags,
    (_, d, _): (usize, &'input str, usize),
    (_, _, _): (usize, number_lexer::Tok<'input>, usize),
    (_, p, _): (usize, &'input str, usize),
    (_, s, _): (usize, ::std::option::Option<(Precision, isize)>, usize),
) -> (Precision, &'input str, &'input str, isize)
{
    { let s = s.unwrap_or_default(); (s.0, d, p, s.1) }
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    flags: &NumberFlags,
    (_, marker, _): (usize, Precision, usize),
    (_, sign, _): (usize, ::std::option::Option<Sign>, usize),
    (_, spanned_exp, _): (usize, (usize, &'input str, usize), usize),
) -> Result<(Precision, isize),__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    {
        let (idx0, exp, _) = spanned_exp;
        let e = exp.parse::<isize>()
            .map_err(|err| ParseError::User {
                error: Error::new(ErrorKind::ParseIntError(err), idx0),
            })?;
        let signed_e = match sign {
            Some(Sign::Neg) => -e,
            _ => e,
        };

        Ok((marker, signed_e))
    }
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Precision
{
    Precision::Double
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Precision
{
    Precision::Single
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Precision
{
    Precision::Single
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Precision
{
    Precision::Double
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Precision
{
    Precision::Double
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Sign
{
    Sign::Pos
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Sign
{
    Sign::Neg
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> NanInf
{
    NanInf::Nan
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> NanInf
{
    NanInf::Inf
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> (usize, &'input str, usize)
{
    (__0, __1, __2)
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, (Precision, isize), usize),
) -> ::std::option::Option<(Precision, isize)>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    flags: &NumberFlags,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<(Precision, isize)>
{
    None
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::option::Option<&'input str>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    flags: &NumberFlags,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<&'input str>
{
    None
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    flags: &NumberFlags,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, BigUint, usize),
) -> ::std::option::Option<BigUint>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    flags: &NumberFlags,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<BigUint>
{
    None
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    flags: &NumberFlags,
    (_, _, _): (usize, number_lexer::Tok<'input>, usize),
    (_, __0, _): (usize, BigUint, usize),
) -> BigUint
{
    (__0)
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, Sign, usize),
) -> ::std::option::Option<Sign>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    flags: &NumberFlags,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Sign>
{
    None
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, Number, usize),
    (_, __2, _): (usize, usize, usize),
) -> (usize, Number, usize)
{
    (__0, __1, __2)
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    flags: &NumberFlags,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, usize, usize),
    __1: (usize, (Precision, &'input str, &'input str, isize), usize),
    __2: (usize, &'input str, usize),
) -> Result<UReal,__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action33(
        flags,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        flags,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, usize, usize),
    __1: (usize, (Precision, &'input str, &'input str, isize), usize),
) -> Result<UReal,__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action34(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        flags,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, number_lexer::Tok<'input>, usize),
    __1: (usize, BigUint, usize),
) -> ::std::option::Option<BigUint>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        flags,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        flags,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, BigUint, usize),
    __1: (usize, number_lexer::Tok<'input>, usize),
    __2: (usize, BigUint, usize),
) -> UReal
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action45(
        flags,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        flags,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, BigUint, usize),
) -> UReal
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action37(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        flags,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, &'input str, usize),
    __1: (usize, usize, usize),
) -> (usize, &'input str, usize)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action35(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        flags,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, Number, usize),
    __1: (usize, usize, usize),
) -> (usize, Number, usize)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action35(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        flags,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, &'input str, usize),
) -> Result<BigUint,__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action35(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        flags,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, (Precision, &'input str, &'input str, isize), usize),
    __1: (usize, &'input str, usize),
) -> Result<UReal,__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action35(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        flags,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, (Precision, &'input str, &'input str, isize), usize),
) -> Result<UReal,__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action35(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        flags,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, &'input str, usize),
) -> (usize, &'input str, usize)
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action42(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        flags,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, Number, usize),
) -> (usize, Number, usize)
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action42(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        flags,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action55<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, Sign, usize),
    __1: (usize, UReal, usize),
) -> Real
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action39(
        flags,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        flags,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action56<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, UReal, usize),
) -> Real
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action40(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        flags,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action57<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, Precision, usize),
    __1: (usize, Sign, usize),
    __2: (usize, (usize, &'input str, usize), usize),
) -> Result<(Precision, isize),__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action39(
        flags,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        flags,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action58<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, Precision, usize),
    __1: (usize, (usize, &'input str, usize), usize),
) -> Result<(Precision, isize),__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action40(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        flags,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action59<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, number_lexer::Tok<'input>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, (Precision, isize), usize),
) -> (Precision, &'input str, &'input str, isize)
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action31(
        flags,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        flags,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action60<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, number_lexer::Tok<'input>, usize),
    __1: (usize, &'input str, usize),
) -> (Precision, &'input str, &'input str, isize)
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action32(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        flags,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action61<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, &'input str, usize),
    __1: (usize, number_lexer::Tok<'input>, usize),
    __2: (usize, (Precision, isize), usize),
) -> (Precision, &'input str, &'input str, isize)
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action31(
        flags,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        flags,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action62<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, &'input str, usize),
    __1: (usize, number_lexer::Tok<'input>, usize),
) -> (Precision, &'input str, &'input str, isize)
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action32(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        flags,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action63<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, &'input str, usize),
    __1: (usize, number_lexer::Tok<'input>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, (Precision, isize), usize),
) -> (Precision, &'input str, &'input str, isize)
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action31(
        flags,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        flags,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action64<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, &'input str, usize),
    __1: (usize, number_lexer::Tok<'input>, usize),
    __2: (usize, &'input str, usize),
) -> (Precision, &'input str, &'input str, isize)
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action32(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        flags,
        __0,
        __1,
        __2,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,number_lexer::Tok<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, number_lexer::Tok<'input>, usize) {
    type Error = number_lexer::Error;
    fn to_triple(value: Self) -> Result<(usize,number_lexer::Tok<'input>,usize),number_lexer::Error> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, number_lexer::Tok<'input>, usize),number_lexer::Error> {
    type Error = number_lexer::Error;
    fn to_triple(value: Self) -> Result<(usize,number_lexer::Tok<'input>,usize),number_lexer::Error> {
        value
    }
}
