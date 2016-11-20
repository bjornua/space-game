type Time = u64;
type PlayerId = u64;
type ObjectId = u64;

struct Location {
    x: u64,
    y: u64,
}

struct Item;


enum MissionType {
    GoTo {
        location: Location
    }
}

struct Mission {
    begin: Time,
    type: MissionType
}

struct WorldObject {
    id: ObjectId,
    owner: Option<PlayerId>,
    location: Location,
    mission: Path
}

struct Player {
    id: PlayerId,
    name: String,
}


struct Game {
    player: Player,
    objects: Objects,
}
