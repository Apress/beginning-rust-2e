/* It prints:
11 21 31 */
fn main() {
    for item in &vec![10, 20, 30] {
        print!("{} ", *item + 1);
    }
}
