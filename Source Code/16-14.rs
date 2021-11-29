/* It prints:
11 21 31 */
fn main() {
    let v = vec![10, 20, 30];
    for mut item in v.into_iter() {
        item += 1;
        print!("{} ", item);
    }
}
