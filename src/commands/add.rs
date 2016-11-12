use super::{ArgSpec, Command, Argument, ArgumentType};


fn add(args: &[Argument]) {
    let a = args[0].unwrap_integer();
    let b = args[1].unwrap_integer();
    println!("{{a}} + {{b}} = {} + {} = {}", a, b, a + b);
}

pub const COMMAND: Command = Command {
    name: "add",
    description: "Adds two integers",
    args: &[ArgSpec {
                kind: ArgumentType::Integer,
                name: "a",
                description: "First addend",
            },
            ArgSpec {
                kind: ArgumentType::Integer,
                name: "b",
                description: "Second addend",
            }],
    function: add,
};
