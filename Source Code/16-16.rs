/* It prints:
11 21 31 ; 11 21 */
fn main() {
    let array_ref_iterator: std::slice::Iter<i32>
        = [10, 20, 30].iter();
    for item in array_ref_iterator {
        print!("{} ", *item + 1);
    }
    print!("; ");
    let slice_ref_iterator: std::slice::Iter<i32>
        = [10, 20, 30][0..2].iter();
    for item in slice_ref_iterator {
        print!("{} ", *item + 1);
    }
}
