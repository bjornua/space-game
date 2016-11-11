use std::io::{Read, Write};

use super::parser::{Token, Tokenizer};
use utils::io::CharIter;


#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub arguments: Vec<Token>,
}


pub fn get_command<T: Iterator<Item = char>>(iter: &mut T) -> Option<Command> {
    let mut tokens = Tokenizer::new(iter);

    tokens.next().map(|name| Command{name: name.text, arguments: tokens.collect() })
}


pub struct CommandIterator<R: Read, W: Write> {
    iterator: CharIter<R>,
    stream_out: W,
}


impl<R: Read, W: Write> CommandIterator<R, W> {
    pub fn new(stream_in: R, stream_out: W) -> Self {
        return CommandIterator { iterator: CharIter::new(stream_in), stream_out: stream_out};
    }
}

impl<R: Read, W: Write> Iterator for CommandIterator<R, W> {
    type Item = Command;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            write!(self.stream_out, "[Command]: ").unwrap();
            self.stream_out.flush().unwrap();

            if let Some(c) =  get_command(&mut self.iterator) {
                println!("{:?}", c);
                return Some(c)
            }
        }
    }
}
