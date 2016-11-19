pub enum Error {
    EOF,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Float,
    Integer,
    Text,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub kind: TokenKind,
}

enum ParserState {
    Integer(String),
    Begin,
    Text(String),
    TextEscape(String),
    Float(String),
    End,
}

fn emptystr() -> String {
    String::with_capacity(20)
}
fn charstr(i: char) -> String {
    let mut s = emptystr();
    s.push(i);
    s
}

impl ParserState {
    fn step(self, c: char) -> (Self, Option<Token>) {
        match self {
            ParserState::Begin => {
                match c {
                    ' ' => (ParserState::Begin, None),
                    '\n' => (ParserState::End, None),
                    c @ '-' => (ParserState::Integer(charstr(c)), None),
                    c => ParserState::Integer(emptystr()).step(c),
                }
            }
            ParserState::Integer(mut xs) => {
                match c {
                    ' ' => {
                        (ParserState::Begin,
                         Some(Token {
                            text: xs,
                            kind: TokenKind::Integer,
                        }))
                    }
                    '\n' => {
                        (ParserState::End,
                         Some(Token {
                            text: xs,
                            kind: TokenKind::Integer,
                        }))
                    }
                    c if c.is_digit(10) => {
                        xs.push(c);
                        (ParserState::Integer(xs), None)
                    }
                    c @ '.' => {
                        xs.push(c);
                        (ParserState::Float(xs), None)
                    }
                    c => ParserState::Text(xs).step(c),
                }
            }
            ParserState::Float(mut xs) => {
                match c {
                    '\n' => {
                        (ParserState::End,
                         Some(Token {
                            text: xs,
                            kind: TokenKind::Float,
                        }))
                    }
                    ' ' => {
                        (ParserState::Begin,
                         Some(Token {
                            text: xs,
                            kind: TokenKind::Float,
                        }))
                    }
                    c if c.is_digit(10) => {
                        xs.push(c);
                        (ParserState::Float(xs), None)
                    }
                    c => ParserState::Text(xs).step(c),
                }
            }
            ParserState::Text(mut xs) => {
                match c {
                    '\n' => {
                        (ParserState::End,
                         Some(Token {
                            text: xs,
                            kind: TokenKind::Text,
                        }))
                    }
                    ' ' => {
                        (ParserState::Begin,
                         Some(Token {
                            text: xs,
                            kind: TokenKind::Text,
                        }))
                    }
                    '\\' => (ParserState::TextEscape(xs), None),
                    c => {
                        xs.push(c);
                        (ParserState::Text(xs), None)
                    }
                }
            }
            ParserState::TextEscape(mut xs) => {
                match c {
                    // No you cannot escape newlines.
                    // If needed consider introducing ['\\', 'n'] instead
                    '\n' => {
                        (ParserState::End,
                         Some(Token {
                            text: xs,
                            kind: TokenKind::Text,
                        }))
                    }
                    c => {
                        xs.push(c);
                        (ParserState::Text(xs), None)

                    }
                }
            }
            ParserState::End => (ParserState::End, None),
        }
    }
}

pub struct Tokenizer<T: Iterator<Item = char>> {
    chars: T,
    state: Option<ParserState>,
}
impl<T: Iterator<Item = char>> Tokenizer<T> {
    pub fn new(chars: T) -> Self {
        return Tokenizer {
            chars: chars,
            state: Some(ParserState::Begin),
        };
    }
}
impl<T: Iterator<Item = char>> Iterator for Tokenizer<T> {
    type Item = Result<Token, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut state = match self.state.take() {
            Some(state) => state,
            None => return None,
        };
        loop {
            if let ParserState::End = state {
                self.state = Some(state);
                return None;
            }
            match self.chars.next() {
                Some(c) => {
                    let (new_state, token) = state.step(c);
                    state = new_state;
                    if token.is_some() {
                        self.state = Some(state);
                        return token.map(|x| Ok(x));
                    }
                }
                None => return Some(Err(Error::EOF)),
            }
        }
    }
}
