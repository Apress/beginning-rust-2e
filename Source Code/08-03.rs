/* It prints:
-5, 183.19, x*/
fn main() {
    let mut data = (10000000, 183.19, 'Q');
    data.0 = -5;
    data.2 = 'x';
    print!("{}, {}, {}", data.0, data.1, data.2);
}
