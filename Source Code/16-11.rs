/* It prints:
11 21 31 */
fn main() {
    let array_iterator: std::array::IntoIter<i32, 3_usize>
        = [10, 20, 30].into_iter();
    for item in array_iterator {
        let j: i32 = item;
        print!("{} ", j + 1);
    }
}
