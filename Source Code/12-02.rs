/* It prints:
4 4 */
fn main() {
    use std::mem;
    print!("{} ", mem::size_of::<i32>());
    print!("{} ", mem::size_of_val(&12));
}
