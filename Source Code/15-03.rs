/* It prints:
3..8, 3, 8, 5
3, 4, 5, 6, 7,
*/
fn main() {
    let range: std::ops::Range<usize> = 3..8;
    println!("{:?}, {}, {}, {}",
        range, range.start, range.end, range.len());
    for i in range { print!("{}, ", i); }
}
