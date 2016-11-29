use super::{Argument, Command};

fn scan(game: GameState, _: &[Argument]) {
    let me = ("jd90j", 0, "Spaceship - Merchant");
    println!("{:?}", me);

    gamestate.get_scan();

}

pub const COMMAND: Command = Command {
    name: "scan",
    description: "List all objects within your radar",
    args: &[],
    function: scan,
};



/*

    println!("Objects within scanner range:");
    println!("Stationary:");
    println!("    c508z |    3km | Weapons factory");
    println!("    4fnk9 |    4km | Fuel station");
    println!("    e4py9 |    4km | Energy farm");
    println!("    6ee2w |    7km | Solar array factory");
    println!("    f8hn5 |    7km | Electrical factory");
    println!("    cxntt |   19km | Vegestables farm");
    println!("    takua |   22km | Computer factory");
    println!("    ra0fz |   51km | Silicon factory");
    println!("    4ys3l |   78km | Iron foundry");
    println!("    6ee2w |  100km | Fish farm");
    println!("    8kms1 |  133km | Algea farm");
    println!("    0b1s5 |  170km | Rocket fuel refinery");
    println!("    mpaui | 1451km | Star");
    println!("Moving:");
    println!("    g8493 |    4km | Spaceship - Passenger transport");
    println!("    i034j |    7km | Spaceship - Scout");
    println!("    3d8bg |    7km | Spaceship - Freighter");
    println!("    l0sxh |   19km | Spaceship - Tug");
    println!("    lf9kk |   22km | Computer factory");


*/
