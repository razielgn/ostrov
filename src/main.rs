use ostrov::repl::repl;
use std::{env, iter::FromIterator};

fn main() {
    let args = Vec::from_iter(env::args());
    repl(&args)
}
