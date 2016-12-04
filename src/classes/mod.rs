struct Vector2D(u64, u64);

struct Time(u64);
struct Velocity(Vector2D);
struct Point(Vector2D);

trait Flying {
    fn get_location(&self) -> Point;
}

