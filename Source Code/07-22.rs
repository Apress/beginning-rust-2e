/* ILLEGAL. The compiler prints the error message:
`match` arms have incompatible types
with the explanation:
expected `char`, found `()`
*/
fn main() {
    #[allow(dead_code)]
    enum CardinalPoint { North, South, West, East }
    let direction = CardinalPoint::South;
    print!("{}", match direction {
        CardinalPoint::North => 'N',
        CardinalPoint::South => 'S',
        _ => {},
    });
}
