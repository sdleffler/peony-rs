// auto-generated: "lalrpop 0.15.0"
#![cfg_attr(rustfmt, rustfmt_skip)]
use failure::*;
use lalrpop_util::ParseError;
use num::{BigInt, BigUint, BigRational, FromPrimitive, Num, One, ToPrimitive, Zero, pow::pow};
use parser::lexer::number_lexer::{self, Error, ErrorKind};
use parser::lexer::{Number, Precision, Sign, NumberFlags, NanInf, UReal, Real};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Number {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use failure::*;
    use lalrpop_util::ParseError;
    use num::{BigInt, BigUint, BigRational, FromPrimitive, Num, One, ToPrimitive, Zero, pow::pow};
    use parser::lexer::number_lexer::{self, Error, ErrorKind};
    use parser::lexer::{Number, Precision, Sign, NumberFlags, NanInf, UReal, Real};
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
        Variant13((Precision, isize)),
        Variant14(::std::option::Option<(Precision, isize)>),
        Variant15(UReal),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0,
        // State 1
        -44, -44, 0, 0, -44, 0, 0, 0, 0, 0, 0, -44, 0, 12, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        8, 9, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 11, 17, 18, 0, 19,
        // State 5
        -42, -42, 0, 20, -42, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0,
        // State 6
        -30, -30, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, -32, 0, 0, 0, 0, 0, 0, 0, -32, -32, -32, 0, -32,
        // State 8
        0, 0, -33, 0, 0, 0, 0, 0, 0, 0, -33, -33, -33, 0, -33,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0,
        // State 10
        -40, -40, 24, -40, -40, 25, 26, 27, 28, 29, 0, -40, 0, 0, 0,
        // State 11
        -43, -43, 0, 0, -43, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0,
        // State 12
        0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 11, 32, 18, 0, 19,
        // State 13
        8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0,
        // State 14
        -31, -31, 0, 0, -31, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0,
        // State 15
        -29, -29, 0, 0, -29, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        -20, -20, 0, 0, -20, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0,
        // State 18
        -19, -19, 0, 0, -19, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0,
        // State 20
        -9, -9, 0, 0, -9, 25, 26, 27, 28, 29, 0, -9, 0, -9, 0,
        // State 21
        8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0,
        // State 22
        -7, -7, 0, 0, -7, 0, 0, 0, 0, 0, 0, -7, 0, -7, 0,
        // State 23
        -11, -11, 0, 0, -11, 25, 26, 27, 28, 29, 43, -11, 0, -11, 0,
        // State 24
        -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 25
        -14, -14, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0,
        // State 26
        -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0,
        // State 27
        -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0,
        // State 28
        -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0,
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
        -41, -41, 0, 0, -41, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0,
        // State 37
        -40, -40, 0, 0, -40, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0,
        // State 38
        -8, -8, 0, 0, -8, 0, 0, 0, 0, 0, 0, -8, 0, -8, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0,
        // State 40
        -37, -37, 0, 0, -37, 0, 0, 0, 0, 0, 0, -37, 0, -37, 0,
        // State 41
        -10, -10, 0, 0, -10, 0, 0, 0, 0, 0, 0, -10, 0, -10, 0,
        // State 42
        -13, -13, 0, 0, -13, 25, 26, 27, 28, 29, 0, -13, 0, -13, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        -36, -36, 0, 0, -36, 0, 0, 0, 0, 0, 0, -36, 0, -36, 0,
        // State 48
        -12, -12, 0, 0, -12, 0, 0, 0, 0, 0, 0, -12, 0, -12, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -44,
        // State 2
        -45,
        // State 3
        -21,
        // State 4
        0,
        // State 5
        -42,
        // State 6
        -30,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        -40,
        // State 11
        -43,
        // State 12
        0,
        // State 13
        0,
        // State 14
        -31,
        // State 15
        -29,
        // State 16
        -28,
        // State 17
        -20,
        // State 18
        -19,
        // State 19
        0,
        // State 20
        -9,
        // State 21
        0,
        // State 22
        -7,
        // State 23
        -11,
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
        -25,
        // State 32
        -22,
        // State 33
        0,
        // State 34
        -27,
        // State 35
        -26,
        // State 36
        -41,
        // State 37
        -40,
        // State 38
        -8,
        // State 39
        0,
        // State 40
        -37,
        // State 41
        -10,
        // State 42
        -13,
        // State 43
        -24,
        // State 44
        -23,
        // State 45
        -31,
        // State 46
        -29,
        // State 47
        -36,
        // State 48
        -12,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 2, 0, 0, 3, 4, 5, 0, 0, 0, 6, 7, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 2, 0, 15, 0, 0, 0, 0, 0, 0, 6, 16, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 2, 0, 30, 0, 0, 0, 0, 0, 0, 6, 31, 0,
        // State 13
        0, 0, 0, 0, 2, 0, 0, 0, 33, 34, 0, 0, 0, 6, 7, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 2, 0, 46, 0, 0, 0, 0, 0, 0, 6, 47, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
                // "mantissa width"? = "mantissa width" => ActionFn(30);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (1, __symbol, 0)
            }
            2 => {
                // "mantissa width"? =  => ActionFn(31);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action31::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                (0, __symbol, 0)
            }
            3 => {
                // ("/" <UInteger>) = "/", UInteger => ActionFn(35);
                let __sym1 = __pop_Variant3(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (2, __symbol, 1)
            }
            4 => {
                // ("/" <UInteger>)? = "/", UInteger => ActionFn(40);
                let __sym1 = __pop_Variant3(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action40::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (2, __symbol, 2)
            }
            5 => {
                // ("/" <UInteger>)? =  => ActionFn(34);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action34::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                (0, __symbol, 2)
            }
            6 => {
                // @L =  => ActionFn(32);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action32::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                (0, __symbol, 3)
            }
            7 => {
                // Decimal = "digits", Suffix => ActionFn(14);
                let __sym1 = __pop_Variant13(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (2, __symbol, 4)
            }
            8 => {
                // Decimal = ".", "digits", Suffix => ActionFn(51);
                let __sym2 = __pop_Variant13(__symbols);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action51::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (3, __symbol, 4)
            }
            9 => {
                // Decimal = ".", "digits" => ActionFn(52);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action52::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (2, __symbol, 4)
            }
            10 => {
                // Decimal = "digits", ".", Suffix => ActionFn(53);
                let __sym2 = __pop_Variant13(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action53::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (3, __symbol, 4)
            }
            11 => {
                // Decimal = "digits", "." => ActionFn(54);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (2, __symbol, 4)
            }
            12 => {
                // Decimal = "digits", ".", "digits", Suffix => ActionFn(55);
                let __sym3 = __pop_Variant13(__symbols);
                let __sym2 = __pop_Variant1(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action55::<>(flags, __sym0, __sym1, __sym2, __sym3);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (4, __symbol, 4)
            }
            13 => {
                // Decimal = "digits", ".", "digits" => ActionFn(56);
                let __sym2 = __pop_Variant1(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                (3, __symbol, 4)
            }
            14 => {
                // ExponentMarker = "E" => ActionFn(19);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 5)
            }
            15 => {
                // ExponentMarker = "S" => ActionFn(20);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 5)
            }
            16 => {
                // ExponentMarker = "F" => ActionFn(21);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 5)
            }
            17 => {
                // ExponentMarker = "D" => ActionFn(22);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 5)
            }
            18 => {
                // ExponentMarker = "L" => ActionFn(23);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant7(__nt), __end);
                (1, __symbol, 5)
            }
            19 => {
                // NanInf = "nan.0" => ActionFn(26);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant8(__nt), __end);
                (1, __symbol, 6)
            }
            20 => {
                // NanInf = "inf.0" => ActionFn(27);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant8(__nt), __end);
                (1, __symbol, 6)
            }
            21 => {
                // Number = Real => ActionFn(1);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (1, __symbol, 7)
            }
            22 => {
                // Number = Real, "@", Real => ActionFn(2);
                let __sym2 = __pop_Variant10(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action2::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (3, __symbol, 7)
            }
            23 => {
                // Number = Real, Sign, UReal, "i" => ActionFn(3);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant15(__symbols);
                let __sym1 = __pop_Variant11(__symbols);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action3::<>(flags, __sym0, __sym1, __sym2, __sym3);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (4, __symbol, 7)
            }
            24 => {
                // Number = Real, Sign, NanInf, "i" => ActionFn(4);
                let __sym3 = __pop_Variant0(__symbols);
                let __sym2 = __pop_Variant8(__symbols);
                let __sym1 = __pop_Variant11(__symbols);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action4::<>(flags, __sym0, __sym1, __sym2, __sym3);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (4, __symbol, 7)
            }
            25 => {
                // Number = Real, Sign, "i" => ActionFn(5);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant11(__symbols);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (3, __symbol, 7)
            }
            26 => {
                // Number = Sign, UReal, "i" => ActionFn(6);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant15(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (3, __symbol, 7)
            }
            27 => {
                // Number = Sign, NanInf, "i" => ActionFn(7);
                let __sym2 = __pop_Variant0(__symbols);
                let __sym1 = __pop_Variant8(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (3, __symbol, 7)
            }
            28 => {
                // Number = Sign, "i" => ActionFn(8);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant9(__nt), __end);
                (2, __symbol, 7)
            }
            29 => {
                // Real = Sign, UReal => ActionFn(47);
                let __sym1 = __pop_Variant15(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action47::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant10(__nt), __end);
                (2, __symbol, 8)
            }
            30 => {
                // Real = UReal => ActionFn(48);
                let __sym0 = __pop_Variant15(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action48::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant10(__nt), __end);
                (1, __symbol, 8)
            }
            31 => {
                // Real = Sign, NanInf => ActionFn(10);
                let __sym1 = __pop_Variant8(__symbols);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(flags, __sym0, __sym1);
                let __symbol = (__start, __Symbol::Variant10(__nt), __end);
                (2, __symbol, 8)
            }
            32 => {
                // Sign = "+" => ActionFn(24);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant11(__nt), __end);
                (1, __symbol, 9)
            }
            33 => {
                // Sign = "-" => ActionFn(25);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant11(__nt), __end);
                (1, __symbol, 9)
            }
            34 => {
                // Sign? = Sign => ActionFn(36);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant12(__nt), __end);
                (1, __symbol, 10)
            }
            35 => {
                // Sign? =  => ActionFn(37);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action37::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant12(__nt), __end);
                (0, __symbol, 10)
            }
            36 => {
                // Suffix = ExponentMarker, Sign, "digits" => ActionFn(49);
                let __sym2 = __pop_Variant1(__symbols);
                let __sym1 = __pop_Variant11(__symbols);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = match super::__action49::<>(flags, __sym0, __sym1, __sym2) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant13(__nt), __end);
                (3, __symbol, 11)
            }
            37 => {
                // Suffix = ExponentMarker, "digits" => ActionFn(50);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action50::<>(flags, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant13(__nt), __end);
                (2, __symbol, 11)
            }
            38 => {
                // Suffix? = Suffix => ActionFn(28);
                let __sym0 = __pop_Variant13(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant14(__nt), __end);
                (1, __symbol, 12)
            }
            39 => {
                // Suffix? =  => ActionFn(29);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action29::<>(flags, &__start, &__end);
                let __symbol = (__start, __Symbol::Variant14(__nt), __end);
                (0, __symbol, 12)
            }
            40 => {
                // UInteger = "digits" => ActionFn(44);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action44::<>(flags, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                (1, __symbol, 13)
            }
            41 => {
                // UReal = UInteger, "/", UInteger => ActionFn(41);
                let __sym2 = __pop_Variant3(__symbols);
                let __sym1 = __pop_Variant0(__symbols);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action41::<>(flags, __sym0, __sym1, __sym2);
                let __symbol = (__start, __Symbol::Variant15(__nt), __end);
                (3, __symbol, 14)
            }
            42 => {
                // UReal = UInteger => ActionFn(42);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(flags, __sym0);
                let __symbol = (__start, __Symbol::Variant15(__nt), __end);
                (1, __symbol, 14)
            }
            43 => {
                // UReal = Decimal, "mantissa width" => ActionFn(45);
                let __sym1 = __pop_Variant1(__symbols);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action45::<>(flags, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant15(__nt), __end);
                (2, __symbol, 14)
            }
            44 => {
                // UReal = Decimal => ActionFn(46);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action46::<>(flags, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                let __symbol = (__start, __Symbol::Variant15(__nt), __end);
                (1, __symbol, 14)
            }
            45 => {
                // __Number = Number => ActionFn(0);
                let __sym0 = __pop_Variant9(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(flags, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (Precision, isize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant15<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UReal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant14<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<(Precision, isize)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
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
    (_, __0, _): (usize, Real, usize),
) -> Number
{
    Number::Real(__0)
}

#[allow(unused_variables)]
fn __action2<
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
fn __action3<
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
fn __action4<
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
fn __action5<
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
fn __action6<
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
fn __action7<
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
fn __action8<
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
fn __action9<
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
fn __action10<
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
fn __action11<
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
fn __action12<
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
fn __action13<
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
fn __action14<
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
fn __action15<
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
fn __action16<
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
fn __action17<
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
fn __action18<
    'input,
>(
    flags: &NumberFlags,
    (_, marker, _): (usize, Precision, usize),
    (_, sign, _): (usize, ::std::option::Option<Sign>, usize),
    (_, loc, _): (usize, usize, usize),
    (_, exp, _): (usize, &'input str, usize),
) -> Result<(Precision, isize),__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    {
        let e = exp.parse::<isize>()
            .map_err(|err| ParseError::User {
                error: Error::new(ErrorKind::ParseIntError(err), loc),
            })?;
        let signed_e = match sign {
            Some(Sign::Neg) => -e,
            _ => e,
        };

        Ok((marker, signed_e))
    }
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Precision
{
    Precision::Double
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Precision
{
    Precision::Single
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Precision
{
    Precision::Single
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Precision
{
    Precision::Double
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Precision
{
    Precision::Double
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Sign
{
    Sign::Pos
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> Sign
{
    Sign::Neg
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> NanInf
{
    NanInf::Nan
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, number_lexer::Tok<'input>, usize),
) -> NanInf
{
    NanInf::Inf
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, (Precision, isize), usize),
) -> ::std::option::Option<(Precision, isize)>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action29<
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
fn __action30<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::option::Option<&'input str>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action31<
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
fn __action32<
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
fn __action33<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, BigUint, usize),
) -> ::std::option::Option<BigUint>
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
) -> ::std::option::Option<BigUint>
{
    None
}

#[allow(unused_variables)]
fn __action35<
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
fn __action36<
    'input,
>(
    flags: &NumberFlags,
    (_, __0, _): (usize, Sign, usize),
) -> ::std::option::Option<Sign>
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
) -> ::std::option::Option<Sign>
{
    None
}

#[allow(unused_variables)]
fn __action38<
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
    let __temp0 = __action30(
        flags,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        flags,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, usize, usize),
    __1: (usize, (Precision, &'input str, &'input str, isize), usize),
) -> Result<UReal,__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action31(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        flags,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, number_lexer::Tok<'input>, usize),
    __1: (usize, BigUint, usize),
) -> ::std::option::Option<BigUint>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action35(
        flags,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        flags,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action41<
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
    let __temp0 = __action40(
        flags,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        flags,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, BigUint, usize),
) -> UReal
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action34(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        flags,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, Precision, usize),
    __1: (usize, ::std::option::Option<Sign>, usize),
    __2: (usize, &'input str, usize),
) -> Result<(Precision, isize),__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
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
        __2,
    )
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, &'input str, usize),
) -> Result<BigUint,__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action32(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        flags,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, (Precision, &'input str, &'input str, isize), usize),
    __1: (usize, &'input str, usize),
) -> Result<UReal,__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action32(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        flags,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, (Precision, &'input str, &'input str, isize), usize),
) -> Result<UReal,__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action32(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action39(
        flags,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, Sign, usize),
    __1: (usize, UReal, usize),
) -> Real
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action36(
        flags,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        flags,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, UReal, usize),
) -> Real
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action37(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action9(
        flags,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, Precision, usize),
    __1: (usize, Sign, usize),
    __2: (usize, &'input str, usize),
) -> Result<(Precision, isize),__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action36(
        flags,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        flags,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, Precision, usize),
    __1: (usize, &'input str, usize),
) -> Result<(Precision, isize),__lalrpop_util::ParseError<usize,number_lexer::Tok<'input>,number_lexer::Error>>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action37(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        flags,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action51<
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
    let __temp0 = __action28(
        flags,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        flags,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, number_lexer::Tok<'input>, usize),
    __1: (usize, &'input str, usize),
) -> (Precision, &'input str, &'input str, isize)
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action29(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        flags,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action53<
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
    let __temp0 = __action28(
        flags,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        flags,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    flags: &NumberFlags,
    __0: (usize, &'input str, usize),
    __1: (usize, number_lexer::Tok<'input>, usize),
) -> (Precision, &'input str, &'input str, isize)
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action29(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        flags,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action55<
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
    let __temp0 = __action28(
        flags,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        flags,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action56<
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
    let __temp0 = __action29(
        flags,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
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
