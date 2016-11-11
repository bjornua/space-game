

// let name = "help";

// let description = "Shows this help"


// let name = "build"
// let description  = "Build a thing from items in your inventory"
// use lib::console::Command;

// pub fn run(Command {name, arguments}: Command) {
// }
// }

use lib::console::parser::{Token, TokenClass};

pub struct ArgSpec {
    name: &'static str,
    type_spec: TokenClass,
    description: &'static str
}

pub struct CommandSpec {
    name: &'static str,
    description: &'static str,
    arg_spec: &'static [ArgSpec],
    function: fn(&[Token])
}
impl CommandSpec {
    fn call(&[Token]) {

    }
}

const COMMANDS: &'static [CommandSpec] = &[
    CommandSpec {
        name: "help",
        description: "Help for a specific command",
        arg_spec: &[
            ArgSpec {
                type_spec: TokenClass::String,
                name: "command_name",
                description: "Which command to show help for",
            }
        ],
        function: help
    },
    CommandSpec {
        name: "add",
        description: "Adds two integers",
        arg_spec: &[
            ArgSpec {
                type_spec: TokenClass::Integer,
                name: "a",
                description: "First addend",
            },
            ArgSpec {
                type_spec: TokenClass::Integer,
                name: "b",
                description: "First addend",
            }

        ],
        function: add
    }
];

fn help(args: &[Token]) {
    let command_name = args[0].to_string();

    for x in COMMANDS {
        if x.name == command_name {
            println!("Found the command!");
        }
    }

}

fn add(args: &[Token]) {
    args[0].to_integer().unwrap();
    args[1].to_integer().unwrap();
}


fn find_commandspec(name: &str) -> Option<&CommandSpec> {
    for x in COMMANDS {
        if name == x.name {
            return Some(x);
        }
    }
    None
}


use lib::console::Command;
pub fn run(c: &Command) {
    match find_commandspec(&c.name) {
        Some(spec) => {
            spec.call(&c.arguments);
        }
        None => println!("Command not found. Write help_all to list available commands.");
    }
}
