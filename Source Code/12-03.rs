/* It prints:
4 4 */
fn main() {
    use std::mem::size_of;
    use std::mem::size_of_val;
    print!("{} ", size_of::<i32>());
    print!("{} ", size_of_val(&12));
}
