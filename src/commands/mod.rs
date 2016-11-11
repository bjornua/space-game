mod help;
mod list;
mod add;
use lib::console::command;
pub use lib::console::parser::{Token, TokenKind};



#[derive(Debug)]
pub struct Argument {
    name: &'static str,
    kind: TokenKind,
    description: &'static str,
}
impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{{{}}}", &self.name)
    }
}

pub struct Command {
    name: &'static str,
    description: &'static str,
    args: &'static [Argument],
    function: fn(&[Token]),
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
    WrongType {
        arg: &'static Argument,
        token: Token,
    },
}
use std::error::Error as StdError;

impl StdError for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::TooFewArguments => "Not enough arguments",
            ErrorKind::TooManyArguments => "Too many arguments",
            ErrorKind::WrongType { .. } => "Argument was wrong type",
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::TooFewArguments |
            ErrorKind::TooManyArguments |
            ErrorKind::WrongType { .. } => write!(f, "{}", self.description()),
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
        for (arg_spec, token) in self.args.into_iter().zip(args) {
            if !token.kind.converts_to(&arg_spec.kind) {
                return Err(Error {
                    command: self,
                    arguments: args.to_vec(),
                    kind: ErrorKind::WrongType {
                        arg: arg_spec,
                        token: token.clone(),
                    },
                });
            }
        }
        (self.function)(args);
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
            println!("Arguments: {:>4} {} {}", self.args[0].kind, self.args[0], self.args[0].description);
            for arg in &self.args[1..] {
                println!("           {}:{}: {}", arg.name, arg.kind, arg.description);
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

pub fn handle_command_error(e: Error) {
    println!("Error: {}", e);
    e.command.print_full();
}