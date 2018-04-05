extern crate lalrpop;
extern crate string_cache_codegen;

use std::{env, fs::File, io::{Read, Write}, path::Path};

const ATOMS: &'static [&'static str] = &[
    "+", "-", "...", "head", "tail", "cons", "lambda", "let", "gensym"
];

fn main() {
    lalrpop::process_root().unwrap();
    string_cache_codegen::AtomType::new("atom::Atom", "atom!")
        .atoms(ATOMS)
        .write_to_file(&Path::new(&env::var("OUT_DIR").unwrap()).join("atom.rs"))
        .unwrap();
}
