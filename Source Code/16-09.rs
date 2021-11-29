/* It prints:
11 21 31 */
fn main() {
    let vec_iterator: std::vec::IntoIter<i32>
        = vec![10, 20, 30].into_iter();
    for item in vec_iterator {
        let j: i32 = item;
        print!("{} ", j + 1);
    }
}
