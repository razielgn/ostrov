#![feature(globs)]

extern crate ostrov;

mod helpers;

mod parser;
mod parser_atoms;
mod parser_booleans;
mod parser_dotted_lists;
mod parser_integers;
mod parser_lists;
mod parser_quoted;

mod eval_primitives;
mod eval_special_forms;

mod ast_fmt;
