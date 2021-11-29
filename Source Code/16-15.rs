/* It prints:
11 21 31 */
fn main() {
    let v = vec![10, 20, 30];
    let vec_ref_iterator: std::slice::Iter<i32> = v.iter();
    for item_ref in vec_ref_iterator {
        print!("{} ", *item_ref + 1);
    }
}
