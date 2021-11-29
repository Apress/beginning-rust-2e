/* It prints:
41 51 */
fn main() {
    let slice_iterator: std::slice::Iter<i32>
        = [40, 50, 60][0..2].into_iter();
    for item in slice_iterator {
        let j: &i32 = item;
        print!("{} ", *j + 1);
    }
}
