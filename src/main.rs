pub mod lib;
pub mod utils;


use std::io::stdin;

use lib::console::CommandIterator;

fn main() {
    println!("Hello world");
    let stdin = stdin();

    let iter = CommandIterator::new(stdin.lock());

    for c in iter {
        println!("{:?}", c);
    }


}
