pub mod lib;
pub mod utils;
pub mod commands;

use std::io::stdin;
use lib::console::CommandIterator;

fn main() {
    println!("Hello world");
    let stdin = stdin();

    let iter = CommandIterator::new(stdin.lock());

    for c in iter {
        commands::run(c);
        // println!("{:?}", c);
    }


}
