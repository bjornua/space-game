pub mod lib;
pub mod utils;
pub mod commands;
pub mod objects;

use lib::console::CommandIterator;
use std::io::{stdin, stdout};

fn main() {
    let stdin_unlocked = stdin();
    let stdin = stdin_unlocked.lock();
    let stdout_unlocked = stdout();
    let stdout = stdout_unlocked.lock();

    let iter = CommandIterator::new(stdin, stdout);

    for c in iter {
        commands::run(&c);
    }


}
