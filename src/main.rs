extern crate ostrov;

use ostrov::repl::repl;

use std::env;
use std::iter::FromIterator;

fn main() {
    repl(Vec::from_iter(env::args()))
}
