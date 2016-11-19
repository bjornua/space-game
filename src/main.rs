pub mod lib;
pub mod utils;
pub mod commands;
pub mod items;

use lib::console::CommandIterator;
use std::error::Error as StdError;
use std::fmt;
use std::io::{stdin, stdout};


fn main() {
    match main_err() {
        Ok(()) => (),
        Err(e) => {
            utils::error::stack_printer(&e);
        }
    }
}

fn main_err() -> Result<(), Error> {
    let stdin_unlocked = stdin();
    let stdin = stdin_unlocked.lock();
    let stdout_unlocked = stdout();
    let stdout = stdout_unlocked.lock();

    let iter = CommandIterator::new(stdin, stdout);

    let items = try!(items::Items::compile());

    println!("{:#?}", items);

    for c in iter {
        commands::run(&c);
    }
    Ok(())
}


#[derive(Debug)]
pub enum Error {
    ItemsError(items::Error),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ItemsError(_) => write!(f, "{}", self.description()),
        }
    }
}
impl From<items::Error> for Error {
    fn from(e: items::Error) -> Self {
        Error::ItemsError(e)
    }
}


impl StdError for Error {
    fn description(&self) -> &'static str {
        match *self {
            Error::ItemsError(_) => "An error happened while trying to get items",
        }
    }
    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::ItemsError(ref e) => Some(e),
        }
    }
}
