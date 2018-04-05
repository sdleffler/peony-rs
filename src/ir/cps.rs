use std::{iter::once, rc::Rc};

use super::{Ctx, Prim, Var};

use ir::anf::{AnfAtomic, AnfComplex, AnfExpr};

#[derive(Debug, Clone)]
pub enum CpsAtomic {
    Lam {
        args: Vec<Var>,
        body: Rc<CpsComplex>,
    },
    Var(Var),
    Bool(bool),
    Int(i64),
    Unit,
}

#[derive(Debug, Clone)]
pub enum CpsComplex {
    If {
        cond: Rc<CpsAtomic>,
        only_then: Rc<CpsComplex>,
        else_then: Rc<CpsComplex>,
    },
    Set {
        var: Var,
        to: Rc<CpsAtomic>,
        then: Rc<CpsComplex>,
    },
    LetRec {
        vars: Vec<(Var, Rc<CpsAtomic>)>,
        then: Rc<CpsComplex>,
    },
    Prim(Prim, Vec<Rc<CpsAtomic>>),
    Apply(Rc<CpsAtomic>, Vec<Rc<CpsAtomic>>),
}

impl Ctx {
    fn cps_tc_apply<I>(
        &self,
        f: Rc<CpsAtomic>,
        mut iter: I,
        mut es: Vec<Rc<CpsAtomic>>,
        c: Rc<CpsAtomic>,
    ) -> Rc<CpsComplex>
    where
        I: Iterator<Item = Rc<AnfAtomic>>,
    {
        match iter.next() {
            Some(anf_e) => self.cps_tk(AnfExpr::Atomic(anf_e), |cps_e| {
                es.push(cps_e);
                self.cps_tc_apply(f, iter, es, c)
            }),
            None => {
                es.push(c);
                Rc::new(CpsComplex::Apply(f, es))
            }
        }
    }

    pub fn cps_t_complex(&self, complex: Rc<AnfComplex>, c: Rc<CpsAtomic>) -> Rc<CpsComplex> {
        match &*complex {
            AnfComplex::Apply(f, es) => self.cps_tk(AnfExpr::Atomic(f.clone()), |f| {
                self.cps_tc_apply(f, es.iter().cloned(), Vec::new(), c)
            }),

            AnfComplex::If {
                cond,
                only_then,
                else_then,
            } => {
                let block = {
                    let k = self.gensym();
                    let k_var = Rc::new(CpsAtomic::Var(k.clone()));

                    let only_then = self.cps_tc(AnfExpr::Complex(only_then.clone()), k_var.clone());
                    let else_then = self.cps_tc(AnfExpr::Complex(else_then.clone()), k_var.clone());

                    let cond = self.cps_tk(AnfExpr::Atomic(cond.clone()), |cond| {
                        Rc::new(CpsComplex::If {
                            cond,
                            only_then,
                            else_then,
                        })
                    });

                    Rc::new(CpsAtomic::Lam {
                        args: vec![k],
                        body: cond,
                    })
                };
                Rc::new(CpsComplex::Apply(block, vec![c]))
            }

            AnfComplex::Set { var, to } => {
                let var = var.clone();
                let to = to.clone();

                let then = Rc::new(CpsComplex::Apply(c, vec![Rc::new(CpsAtomic::Unit)]));
                self.cps_tk(to, |to| Rc::new(CpsComplex::Set { var, to, then }))
            }
        }
    }

    pub fn cps_tc(&self, expr: AnfExpr, c: Rc<CpsAtomic>) -> Rc<CpsComplex> {
        match expr {
            AnfExpr::LetRec { .. } => unimplemented!(),
            AnfExpr::Atomic(atomic) => Rc::new(CpsComplex::Apply(c, vec![self.cps_m(atomic)])),
            AnfExpr::Complex(complex) => self.cps_t_complex(complex, c),
        }
    }

    pub fn cps_tk<F: FnOnce(Rc<CpsAtomic>) -> Rc<CpsComplex>>(
        &self,
        expr: AnfExpr,
        k: F,
    ) -> Rc<CpsComplex> {
        match expr {
            AnfExpr::LetRec { .. } => unimplemented!(),
            AnfExpr::Atomic(atomic) => k(self.cps_m(atomic)),
            AnfExpr::Complex(complex) => {
                let cont = {
                    let rv = self.gensym();
                    let rv_var = Rc::new(CpsAtomic::Var(rv.clone()));
                    Rc::new(CpsAtomic::Lam {
                        args: vec![rv],
                        body: k(rv_var),
                    })
                };

                self.cps_t_complex(complex, cont)
            }
        }
    }

    pub fn cps_m(&self, expr: Rc<AnfAtomic>) -> Rc<CpsAtomic> {
        let atomic = match &*expr {
            AnfAtomic::Lam { args, body } => {
                let k = self.gensym();
                let args = args.into_iter().chain(once(&k)).cloned().collect();
                let body = self.cps_tc(AnfExpr::Complex(body.clone()), Rc::new(CpsAtomic::Var(k)));
                CpsAtomic::Lam { args, body }
            }
            AnfAtomic::Var(v) => CpsAtomic::Var(v.clone()),
            AnfAtomic::Bool(b) => CpsAtomic::Bool(*b),
            AnfAtomic::Int(i) => CpsAtomic::Int(*i),
            AnfAtomic::Unit => CpsAtomic::Unit,
        };

        Rc::new(atomic)
    }
}
