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

mod eval_application;
mod eval_basic_types;
mod eval_boolean_procedures;
mod eval_conditionals;
mod eval_definitions;
mod eval_integer_procedures;
mod eval_quotation;

mod ast_fmt;
