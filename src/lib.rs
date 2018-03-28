#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate itertools;
extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;
extern crate num;
#[macro_use]
extern crate peony;
extern crate regex;
extern crate string_cache;
extern crate unicode_categories;

#[macro_use]
mod atom {
    include!(concat!(env!("OUT_DIR"), "/atom.rs"));
}

pub mod ast;
pub mod parser;
