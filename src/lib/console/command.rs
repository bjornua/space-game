use utils::io::CharIter;
use std::io::Read;

use super::parser::{Token, Tokenizer};


#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub arguments: Vec<Token>,
}


pub fn get_command<T: Iterator<Item = char>>(iter: &mut T) -> Option<Command> {
    let mut tokens = Tokenizer::new(iter);

    let name = tokens.next().map_or_else(|| "".to_string(), |x| x.to_string());

    Some(Command {
        name: name,
        arguments: tokens.collect(),
    })
}


pub struct CommandIterator<R: Read> {
    iterator: CharIter<R>,
}


impl<R: Read> CommandIterator<R> {
    pub fn new(stream: R) -> Self {
        return CommandIterator { iterator: CharIter::new(stream) };
    }
}


impl<R: Read> Iterator for CommandIterator<R> {
    type Item = Command;
    fn next(&mut self) -> Option<Self::Item> {
        return get_command(&mut self.iterator);
    }
}
