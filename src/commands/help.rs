use super::{Argument, ArgSpec, ArgumentType, Command};
use super::find_command;

fn help(args: &[Argument]) {
    let command_name = args[0].unwrap_str();

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
    args: &[ArgSpec {
                kind: ArgumentType::String,
                name: "command_name",
                description: "The name of the command",
            }],
    function: help,
};
