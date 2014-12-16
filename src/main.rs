extern crate ostrov;

use ostrov::repl::repl;

use std::os;

fn main() {
    repl(os::args())
}
