#[derive(Debug)]


pub enum TokenClass {
    Float,
    Integer,
    Text,
}

pub struct Token {
    text: String,
    class: TokenClass
}

impl Token {
    pub fn to_integer(&self) -> Option<i64> {
        match self.class {
            TokenClass::Float | TokenClass::Text => {
                return None
            }
            TokenClass::Integer => {
                self.text.parse().ok()
            }
        }
    }
    pub fn to_float(&self) -> Option<f64> {
        match self.class {
            TokenClass::Float | TokenClass::Integer => {
                self.text.parse::<f64>().ok()
            }
            TokenClass::Text => {
                return None
            }
        }
    }
}



enum ParserState {
    Integer(String),
    Begin,
    Text(String),
    TextEscape(String),
    Float(String),
    End,
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
                    ' ' => (ParserState::Begin, Some(Token { text: xs, class: TokenClass::Integer })),
                    '\n' => (ParserState::End, Some(Token { text: xs, class: TokenClass::Integer })),
                    c => ParserState::Text(xs).step(c),
                }
            }
            ParserState::Float(mut xs) => {
                match c {
                    '\n' => (ParserState::End, Some(Token { text: xs, class: TokenClass::Float})),
                    ' ' => (ParserState::Begin, Some(Token { text: xs, class: TokenClass::Float})),
                    c if c.is_digit(10) => {
                        xs.push(c);
                        (ParserState::Float(xs), None)
                    }
                    c => ParserState::Text(xs).step(c),
                }
            }
            ParserState::Text(mut xs) => {
                match c {
                    '\n' => (ParserState::End, Some(Token { text: xs, class: TokenClass::Text})),
                    ' ' => (ParserState::Begin, Some(Token{text:xs,class:TokenClass::Text})),
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
                    '\n' => (ParserState::End, Some(Token{text:xs, class: TokenClass::Text})),
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
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let mut state = match self.state.take() {
            Some(state) => state,
            None => return None,
        };

        if let ParserState::End = state {
            return None;
        }

        for c in &mut self.chars {
            let (new_state, token) = state.step(c);

            state = new_state;
            if token.is_some() {
                self.state = Some(state);
                return token;
            }

        }
        return None;
    }
}
