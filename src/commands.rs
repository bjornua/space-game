

// let name = "help";

// let description = "Shows this help"


// let name = "build"
// let description  = "Build a thing from items in your inventory"
// use lib::console::Command;

// pub fn run(Command {name, arguments}: Command) {
// }
// }

pub enum Type {
    Integer,
    Float,
    String
}

pub struct Parameter {
    name: &'static str,
    type_: Type,
    description: &'static str
}

pub struct Command {
    name: &'static str,
    parameters: &'static [Parameter],
    function: FnOnce(u64)
}

fn help (x: u64) {

}

const HELP: Command = Command {
    name: "Help",
    parameters: &[

    ],
    function: |x| {}
};

// const HELP {

// }