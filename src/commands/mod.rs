mod help;
mod list;
mod add;
use lib::console::command;
use lib::console::parser::{Token, TokenKind};
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum ArgumentError {
    WrongType { token: Token, target: ArgumentType },
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
}
impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl StdError for ArgumentError {
    fn description(&self) -> &'static str {
        match *self {
            ArgumentError::WrongType { .. } => "Wrong type",
            ArgumentError::ParseIntError(_) => "An error happened while parsing integer",
            ArgumentError::ParseFloatError(_) => "An error happened while parsing float",
        }
    }
    fn cause(&self) -> Option<&StdError> {
        match *self {
            ArgumentError::WrongType { .. } => None,
            ArgumentError::ParseIntError(ref e) => Some(e),
            ArgumentError::ParseFloatError(ref e) => Some(e),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ArgumentType {
    String,
    Integer,
    Float,
}

impl fmt::Display for ArgumentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            ArgumentType::Float => "float",
            ArgumentType::Integer => "integer",
            ArgumentType::String => "string",
        };
        s.fmt(f)
    }
}


impl ArgumentType {
    pub fn convert(&self, token: &Token) -> Result<Argument, ArgumentError> {
        match (&token.kind, self) {
            (&TokenKind::Integer, &ArgumentType::Integer) => {
                match token.text.parse::<i64>() {
                    Ok(i) => Ok(Argument::Integer(i)),
                    Err(e) => Err(ArgumentError::ParseIntError(e)),
                }
            }
            (&TokenKind::Integer, &ArgumentType::Float) |
            (&TokenKind::Float, &ArgumentType::Float) => {
                match token.text.parse::<f64>() {
                    Ok(i) => Ok(Argument::Float(i)),
                    Err(e) => Err(ArgumentError::ParseFloatError(e)),
                }
            }
            (&TokenKind::Float, &ArgumentType::String) |
            (&TokenKind::Integer, &ArgumentType::String) |
            (&TokenKind::Text, &ArgumentType::String) => Ok(Argument::String(token.text.clone())),
            (&TokenKind::Text, &ArgumentType::Float) |
            (&TokenKind::Float, &ArgumentType::Integer) |
            (&TokenKind::Text, &ArgumentType::Integer) => {
                Err(ArgumentError::WrongType {
                    token: token.clone(),
                    target: (*self).clone(),
                })
            }
        }
    }
}
pub enum Argument {
    String(String),
    Integer(i64),
    Float(f64),
}

impl Argument {
    pub fn unwrap_integer(&self) -> i64 {
        if let Argument::Integer(i) = *self {
            return i;
        }
        panic!("Not an integer");
    }
    pub fn unwrap_float(&self) -> f64 {
        if let Argument::Float(i) = *self {
            return i;
        }
        panic!("Not a float");
    }
    pub fn unwrap_str<'a>(&'a self) -> &'a str {
        if let Argument::String(ref s) = *self {
            return s;
        }
        panic!("Not an string");
    }
}


#[derive(Debug)]
pub struct ArgSpec {
    name: &'static str,
    kind: ArgumentType,
    description: &'static str,
}
impl fmt::Display for ArgSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{{{}}}", &self.name)
    }
}

pub struct Command {
    name: &'static str,
    description: &'static str,
    args: &'static [ArgSpec],
    function: fn(&[Argument]),
}

use std::fmt;
impl fmt::Debug for Command {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.debug_struct("Foo")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("args", &self.args)
            .field("function", &"...".to_string())
            .finish()
    }
}

#[derive(Debug)]
enum ErrorKind {
    TooFewArguments,
    TooManyArguments,
    ArgumentError {
        arg_spec: &'static ArgSpec,
        error: ArgumentError,
    },
}
use std::error::Error as StdError;

impl StdError for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::TooFewArguments => "Not enough arguments",
            ErrorKind::TooManyArguments => "Too many arguments",
            ErrorKind::ArgumentError { .. } => "Argument error",
        }
    }
    fn cause(&self) -> Option<&StdError> {
        match self.kind {
            ErrorKind::TooFewArguments |
            ErrorKind::TooManyArguments => None,
            ErrorKind::ArgumentError { ref error, .. } => Some(error),
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::TooFewArguments |
            ErrorKind::TooManyArguments => self.description().fmt(f),
            ErrorKind::ArgumentError { arg_spec, .. } => {
                format!("{}: {}", self.description(), arg_spec).fmt(f)
            }
        }
    }
}


#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    arguments: Vec<Token>,
    command: &'static Command,
}

impl Command {
    fn call(&'static self, args: &[Token]) -> Result<(), Error> {
        if self.args.len() > args.len() {
            return Err(Error {
                command: self,
                arguments: args.to_vec(),
                kind: ErrorKind::TooFewArguments,
            });
        }
        if self.args.len() < args.len() {
            return Err(Error {
                command: self,
                arguments: args.to_vec(),
                kind: ErrorKind::TooManyArguments,
            });
        }
        let args: Result<Vec<_>, Error> = self.args
            .into_iter()
            .zip(args)
            .map(|(arg_spec, token)| {
                match arg_spec.kind.convert(token) {
                    Ok(s) => Ok(s),
                    Err(e) => {
                        Err(Error {
                            command: self,
                            arguments: args.to_vec(),
                            kind: ErrorKind::ArgumentError {
                                error: e,
                                arg_spec: arg_spec,
                            },
                        })
                    }
                }
            })
            .collect();
        let args = try!(args);
        (self.function)(&args);
        Ok(())
    }
    fn print_usage(&self) {
        print!("{}", self.name);
        for arg in self.args {
            print!(" {}", arg);
        }
    }
    // One line description
    fn print_full(&self) {
        print!("Usage:     ");
        self.print_usage();
        println!("");

        if self.args.len() > 0 {
            println!("Arguments: {:7} {} - {}",
                     self.args[0].kind,
                     self.args[0],
                     self.args[0].description);
            for arg in &self.args[1..] {
                println!("           {:7} {} - {}", arg.kind, arg, arg.description);
            }
            println!("");
        }
        println!("{}", self.description);
    }
    fn print_short(&self) {
        println!("{: <10} - {}", self.name, self.description);
        // println!(" - {}", self.description);
    }
}

const COMMANDS: &'static [Command] = &[help::COMMAND, list::COMMAND, add::COMMAND];


fn find_command(name: &str) -> Option<&'static Command> {
    for x in COMMANDS {
        if name == x.name {
            return Some(x);
        }
    }
    None
}

pub fn run(c: &command::Command) {
    match find_command(&c.name) {
        Some(spec) => {
            match spec.call(&c.arguments) {
                Ok(()) => (),
                Err(e) => handle_command_error(e),
            }
        }
        None => println!("Command not found. Type \"list\" for available commands."),
    }
}

use utils::error::stack_printer;

pub fn handle_command_error(e: Error) {
    print!("ERROR: ");
    stack_printer(&e);
    println!("");
    e.command.print_full();
}
