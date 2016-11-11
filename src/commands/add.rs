use super::{Argument, Command, Token, TokenKind};


fn add(args: &[Token]) {
    let a = args[0].to_integer().unwrap();
    let b = args[1].to_integer().unwrap();
    println!("{{a}} + {{b}} = {} + {} = {}", a, b, a + b);
}
pub const COMMAND: Command = Command {
    name: "add",
    description: "Adds two integers",
    args: &[Argument {
                kind: TokenKind::Integer,
                name: "a",
                description: "First addend",
            },
            Argument {
                kind: TokenKind::Integer,
                name: "b",
                description: "Second addend",
            }],
    function: add,
};
