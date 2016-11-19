type PlayerId = u64;
type FloatingId = u64;

struct Location {
    x: u64,
    y: u64,
}

struct Item {

}

enum ObjectClass {
    Debris()
}

struct  {
    id: u64,
    owner: Option<PlayerId>,
    location: Location,
}

struct Player {
    id: PlayerId,
    name: String,
}


struct Game {
    player: Player,
    objects: Objects,
}
