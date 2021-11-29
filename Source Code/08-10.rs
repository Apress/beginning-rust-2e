/* It prints:
8.2, 10*/
fn main() {
    struct SomeData {
        integer: i32,
        fractional: f32,
    }
    let mut data = SomeData {
        integer: 10,
        fractional: 183.19,
    };
    data.fractional = 8.2;
    print!("{}, {}", data.fractional, data.integer);
}
