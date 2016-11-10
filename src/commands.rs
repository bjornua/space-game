

// let name = "help";

// let description = "Shows this help"


// let name = "build"
// let description  = "Build a thing from items in your inventory"
// use lib::console::Command;

// pub fn run(Command {name, arguments}: Command) {
// }
// }

use lib::console::Token;

pub enum TypeSpec {
    Integer,
    Float,
    String
}

pub struct ArgSpec {
    name: &'static str,
    type_spec: TypeSpec,
    description: &'static str
}

pub struct Command {
    name: &'static str,
    description: &'static str,
    arg_spec: &'static [ArgSpec],
    function: Box<Fn(&[Token])>
}

const HELP: Command = Command {
    name: "help",
    description: "Help for ",
    arg_spec: &[
        ArgSpec {
            type_spec: TypeSpec::String,

            name: "",
            description: "Which command to show help for",

        }

    ],
    function: Box::new(|params| {
        params[0].unwrap_text();

    })
};

// const HELP {

// }
