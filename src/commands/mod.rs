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
enum CommandErrorKind {
    TooFewArguments,
    TooManyArguments,
    WrongType {
        arg: &'static Argument,
        token: Token,
    },
}

#[derive(Debug)]
pub struct CommandError {
    kind: CommandErrorKind,
    arguments: Vec<Token>,
    command: &'static Command,
}

impl Command {
    fn call(&'static self, args: &[Token]) -> Result<(), CommandError> {
        if self.args.len() > args.len() {
            return Err(CommandError {
                command: self,
                arguments: args.to_vec(),
                kind: CommandErrorKind::TooFewArguments,
            });
        }
        if self.args.len() < args.len() {
            return Err(CommandError {
                command: self,
                arguments: args.to_vec(),
                kind: CommandErrorKind::TooManyArguments,
            });
        }
        for (arg_spec, token) in self.args.into_iter().zip(args) {
            if !token.kind.converts_to(&arg_spec.kind) {
                return Err(CommandError {
                    command: self,
                    arguments: args.to_vec(),
                    kind: CommandErrorKind::WrongType {
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
            print!(" {{{}}}", arg.name);
        }
    }
    // One line description
    fn print_full(&self) {
        if self.args.len() > 0 {
            println!("Usage:");
            print!("    ");
            self.print_usage();
            println!("");
            for arg in self.args {
                println!("        - {{{}}}: {}", arg.name, arg.description);
            }
            println!("");
        }
        println!("Description:");
        println!("    {}", self.description);
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
                Err(e) => println!("{:#?}", e),
            }
        }
        None => println!("Command not found. Run help_all to list available commands."),
    }
}
