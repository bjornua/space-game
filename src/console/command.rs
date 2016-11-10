use utils::io::CharIter;
use std::io::Read;


#[derive(Debug)]
pub enum Token {
    Float(Vec<char>),
    Integer(Vec<char>),
    Text(Vec<char>),
}


impl Token {
    pub fn to_string(self) -> String {
        match self {
            Token::Float(chars) | Token::Integer(chars) | Token::Text(chars)
            => {
                chars.into_iter().collect()
            }
        }
    }
}



#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub arguments: Vec<Token>,
}

enum ParserState {
    Integer(Vec<char>),
    Begin,
    Text(Vec<char>),
    TextEscape(Vec<char>),
    Float(Vec<char>),
    End
}


impl ParserState {
    fn step(self, c: char) -> (Self, Option<Token>) {
        match self {
            ParserState::Begin => {
                match c {
                    c @ '-' => (ParserState::Integer(vec![c]), None),
                    ' ' => (ParserState::Begin, None),
                    c => ParserState::Integer(vec![]).step(c),
                }
            }
            ParserState::Integer(mut xs) => {
                match c {
                    c if c.is_digit(10) => {
                        xs.push(c);
                        (ParserState::Integer(xs), None)
                    }
                    c @ '.' => {
                        xs.push(c);
                        (ParserState::Float(xs), None)
                    }
                    ' ' => (ParserState::Begin, Some(Token::Integer(xs))),
                    '\n' => (ParserState::End, Some(Token::Integer(xs))),
                    c => {
                        ParserState::Text(xs).step(c)
                    }
                }
            }
            ParserState::Float(mut xs) => {
                match c {
                    '\n' => (ParserState::End, Some(Token::Float(xs))),
                    ' ' => (ParserState::Begin, Some(Token::Float(xs))),
                    c if c.is_digit(10) => {
                        xs.push(c);
                        (ParserState::Float(xs), None)
                    }
                    c => {
                        ParserState::Text(xs).step(c)
                    }
                }
            }
            ParserState::Text(mut xs) => {
                match c {
                    '\n' => (ParserState::End, Some(Token::Text(xs))),
                    ' ' => (ParserState::Begin, Some(Token::Text(xs))),
                    '\\' => {
                        (ParserState::TextEscape(xs), None)
                    }
                    c => {
                        xs.push(c);
                        (ParserState::Text(xs), None)
                    }
                }
            }
            ParserState::TextEscape(mut xs) => {
                match c {
                    '\n' => {
                        (ParserState::Begin, Some(Token::Text(xs)))
                    }
                    c => {
                        xs.push(c);
                        (ParserState::Text(xs), None)

                    }
                }
            }
            ParserState::End => {
                (ParserState::End, None)
            }
        }
    }
}

struct Tokenizer<T: Iterator<Item = char>> {
    chars: T,
    state: Option<ParserState>,
}
impl<T: Iterator<Item = char>> Tokenizer<T> {
    pub fn new(chars: T) -> Self {
        return Tokenizer {
            chars: chars,
            state: Some(ParserState::Begin)

        }
    }
}
impl<T: Iterator<Item = char>> Iterator for Tokenizer<T> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let mut state = match self.state.take() {
            Some(state) => state,
            None => return None
        };

        if let ParserState::End = state {
            return None
        }

        for c in &mut self.chars {
            let (new_state, token) = state.step(c);

            state = new_state;
            if token.is_some() {
                self.state = Some(state);
                return token;
            }

        }
        return None
    }
}

pub fn get_command<T: Iterator<Item = char>>(iter: &mut T) -> Option<Command> {
    let mut tokens = Tokenizer::new(iter);

    let name = tokens.next().map_or_else(|| "".to_string(), |x| x.to_string());

    Some(Command { name: name, arguments: tokens.collect() })
}


pub struct CommandIterator<R: Read> {
    iterator: CharIter<R>,
}


impl<R: Read> CommandIterator<R> {
    pub fn new(stream: R) -> Self {
        return CommandIterator {
            iterator: CharIter::new(stream)
        }
    }
}


impl<R: Read> Iterator for CommandIterator<R> {
    type Item = Command;
    fn next(&mut self) -> Option<Self::Item> {
        return get_command(&mut self.iterator)
    }
}



// impl<R: Read> CommandIterator<R> {
//     fn close(&mut self) -> Option<R> {
//         self.stream.take()
//     }
// }
