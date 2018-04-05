pub mod anf;
pub mod cps;

use std::{cell::Cell, collections::HashMap, rc::Rc};

use atom::Atom;

#[derive(Debug)]
pub struct Ctx {
    next_sym: Cell<u64>,
}

impl Ctx {
    pub fn gensym(&self) -> Var {
        let id = self.next_sym.get();
        self.next_sym.set(id + 1);
        Var {
            id,
            atom: atom!("gensym"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Prim {
    Add,
    Sub,
    Mul,
    Div,

    Eq,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var {
    id: u64,
    atom: Atom,
}
