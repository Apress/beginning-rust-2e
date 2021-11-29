/* It prints:
60, 10000000, 183.19, Q*/
fn main() {
    struct SomeData {
        integer: i32,
        fractional: f32,
        character: char,
        five_bytes: [u8; 5],
    }
    let data = SomeData {
        integer: 10_000_000,
        fractional: 183.19,
        character: 'Q',
        five_bytes: [9, 0, 250, 60, 200],
    };
    print!("{}, {}, {}, {}",
        data.five_bytes[3], data.integer,
        data.fractional, data.character);
}
