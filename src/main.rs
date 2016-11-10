pub mod console;
pub mod parser;

use std::io::{self, Write};

fn main () {
    println!("Hello world");
    let stdin = io::stdin();
    // for command in console::ConsoleInput::new(stdin) {
    //     println!("{:?}", command);
    // }
    for c in console::CharIterator::new(stdin) {
        print!("{:?}, ", c);
        io::stdout().flush().unwrap();
    }


}
