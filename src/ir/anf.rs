use std::rc::Rc;

use super::{Prim, Var};

#[derive(Debug, Clone)]
pub enum AnfAtomic {
    Lam {
        args: Vec<Var>,
        body: Rc<AnfComplex>,
    },
    Var(Var),
    Bool(bool),
    Int(i64),
    Unit,
}

#[derive(Debug, Clone)]
pub enum AnfComplex {
    Apply(Rc<AnfAtomic>, Vec<Rc<AnfAtomic>>),
    If {
        cond: Rc<AnfAtomic>,
        only_then: Rc<AnfComplex>,
        else_then: Rc<AnfComplex>,
    },
    Set {
        var: Var,
        to: AnfExpr,
    },
}

#[derive(Debug, Clone)]
pub enum AnfExpr {
    LetRec {
        vars: Vec<(Var, Rc<AnfAtomic>)>,
        then: Rc<AnfComplex>,
    },
    Atomic(Rc<AnfAtomic>),
    Complex(Rc<AnfComplex>),
}

