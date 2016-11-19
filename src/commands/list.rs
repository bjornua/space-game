use super::{Argument, Command};
use super::COMMANDS;

fn run(_: &[Argument]) {
    let mut sorted_list: Vec<_> = COMMANDS.into_iter().collect();
    sorted_list.sort_by_key(|x| x.name);

    for command in sorted_list {
        command.print_short();
    }
}
pub const COMMAND: Command = Command {
    name: "list",
    description: "List all available commands",
    args: &[],
    function: run,
};
