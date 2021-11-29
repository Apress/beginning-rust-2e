/* It prints:
0 a, 1 b, 2 c, */
fn main() {
    let arr = ['a', 'b', 'c'];
    for (index, ch) in arr.into_iter().enumerate() {
        print!("{} {}, ", index, ch);
    }
}
