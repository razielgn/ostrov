extern crate ostrov;

use ostrov::repl::repl;

use std::env;
use std::iter::FromIterator;

fn main() {
    let args = Vec::from_iter(env::args());
    repl(&args)
}
