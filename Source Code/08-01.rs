/* It prints:
10000000, 183.19, Q*/
fn main() {
    let data = (10000000, 183.19, 'Q');
    let copy_of_data = data;
    print!("{}, {}, {}",
        data.0, copy_of_data.1, data.2);
}
