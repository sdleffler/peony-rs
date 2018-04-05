#![feature(plugin, trace_macros)]
#![plugin(interpolate_idents)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate garden;
extern crate itertools;
extern crate lalrpop_util;
#[macro_use]
extern crate lazy_static;
extern crate num;
#[macro_use]
extern crate regex;
extern crate smallvec;
extern crate string_cache;
extern crate unicode_categories;

#[macro_use]
mod atom {
    include!(concat!(env!("OUT_DIR"), "/atom.rs"));
}

pub mod ast;
pub mod ir;
pub mod parser;
pub mod vm;
