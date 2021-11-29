/* ILLEGAL. The compiler prints the error message:
binary operation `<` cannot be applied to type `CardinalPoint`
*/
fn main() {
    enum CardinalPoint { North, South, West, East }
    if CardinalPoint::South < CardinalPoint::North { }
}
