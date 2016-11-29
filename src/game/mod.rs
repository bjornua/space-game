use items::Item;
pub struct GameState {
    pub objects: Vec<Item>
}

pub fn generate_gamestate() -> GameState {
    GameState {
        objects: vec![

        ]
    }
}


// type Time = u64;
// type PlayerId = u64;
// type ObjectId = u64;

// struct Vector2(x, y);

// struct Point(Vector2);
// struct Velocity(Vector2);


// struct Location {
//     x: u64,
//     y: u64,
// }

// struct Item;

// enum MissionType {
//     Goto {
//         location: Point
//     }
// }

// struct Mission {
//     begin: Time,
//     type: MissionType
// }

// struct WorldObject {
//     id: ObjectId,
//     // owner: Option<PlayerId>,
//     // location: Location,
//     // mission: Path
// }

// struct Player {
//     id: PlayerId,
//     name: String,
// }


// struct Game {
//     player: Player,
//     objects: Objects,
// }
