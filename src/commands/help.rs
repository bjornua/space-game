use super::{Argument, Command, Token, TokenKind};
use super::find_command;

fn help(args: &[Token]) {
    let command_name = &args[0].text;

    let command = match find_command(command_name) {
        Some(c) => c,
        None => {
            println!("Could not find help for command {:?}", command_name);
            return;
        }
    };

    command.print_full();

}
pub const COMMAND: Command = Command {
    name: "help",
    description: "Shows the full description of {command_name}",
    args: &[Argument {
                kind: TokenKind::Text,
                name: "command_name",
                description: "The name of the command",
            }],
    function: help,
};
