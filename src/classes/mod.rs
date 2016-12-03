enum Location {
    Point(u64, u64)
    Container()
}
struct Point(u64, u64);



trait Location {
    fn get_location(&self) -> Point();
}
