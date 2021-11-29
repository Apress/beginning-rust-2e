/* ILLEGAL. The compiler prints the error message:
non-exhaustive patterns: `West` and `East` not covered
*/
fn main() {
    #[allow(dead_code)]
    enum CardinalPoint { North, South, West, East }
    let direction = CardinalPoint::South;
    match direction {
        CardinalPoint::North => print!("NORTH"),
        CardinalPoint::South => print!("SOUTH"),
    }
}
