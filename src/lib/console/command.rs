use std::io::{Read, Write};

use super::parser::{self, Token, Tokenizer};
use utils::io::CharIter;


#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub arguments: Vec<Token>,
}

pub enum Error {
    EOF,
    MissingName,
}

pub fn get_command<T: Iterator<Item = char>>(iter: &mut T) -> Result<Command, Error> {
    let mut tokens = Tokenizer::new(iter);

    let name = match tokens.next() {
        Some(Err(parser::Error::EOF)) => return Err(Error::EOF),
        Some(Ok(n)) => n,
        None => return Err(Error::MissingName),
    };

    let arguments: Result<Vec<_>, parser::Error> = tokens.collect();

    match arguments {
        Err(parser::Error::EOF) => return Err(Error::EOF),
        Ok(arguments) => {
            Ok(Command {
                name: name.text,
                arguments: arguments,
            })
        }
    }
}


pub struct CommandIterator<R: Read, W: Write> {
    iterator: CharIter<R>,
    stream_out: W,
}


impl<R: Read, W: Write> CommandIterator<R, W> {
    pub fn new(stream_in: R, stream_out: W) -> Self {
        return CommandIterator {
            iterator: CharIter::new(stream_in),
            stream_out: stream_out,
        };
    }
}

impl<R: Read, W: Write> Iterator for CommandIterator<R, W> {
    type Item = Command;
    fn next(&mut self) -> Option<Self::Item> {
        write!(self.stream_out, "\n").unwrap();
        loop {
            write!(self.stream_out, "[Command]: ").unwrap();
            self.stream_out.flush().unwrap();

            match get_command(&mut self.iterator) {
                Err(Error::MissingName) => continue,
                Err(Error::EOF) => return None,
                Ok(command) => {
                    writeln!(self.stream_out, "").unwrap();
                    return Some(command);
                }
            }
        }
    }
}
