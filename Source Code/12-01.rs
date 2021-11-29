/* It prints:
4 4 */
fn main() {
    print!("{} ", std::mem::size_of::<i32>());
    print!("{} ", std::mem::size_of_val(&12));
}
