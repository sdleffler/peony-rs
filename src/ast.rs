use std::sync::Arc;

use num::BigUint;

use atom::Atom;

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

#[derive(Debug, Clone)]
pub enum Datum {
    Boolean(bool),
    Number(Number),
    String(Atom),
    Symbol(Atom),

    Cons(Arc<Datum>, Arc<Datum>),

    Vector(Vec<Arc<Datum>>),
    ByteVector(Vec<u8>),
}
