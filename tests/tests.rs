#![feature(globs)]

extern crate ostrov;

mod helpers;

mod parser_atoms;
mod parser_booleans;
mod parser_integers;
mod parser_lists;
mod parser_quoted;

mod eval_primitives;
mod eval_quoted;
