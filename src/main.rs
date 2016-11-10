pub mod console;
pub mod parser;
pub mod utils;

use std::io::{stdin};


fn main () {
    println!("Hello world");
    let stdin = stdin();
    // for command in console::ConsoleInput::new(stdin) {
    //     println!("{:?}", command);
    // }

    let iter = console::CommandIterator::new(stdin.lock());

    for c in iter {
        println!("{:?}", c);
        // io::stdout().flush().unwrap();
    }


}
