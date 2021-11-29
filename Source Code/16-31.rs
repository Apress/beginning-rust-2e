/* It prints:
0 a, 1 b, 2 c, */
fn main() {
    let arr = ['a', 'b', 'c'];
    for index in 0..arr.len() {
        print!("{} {}, ", index, arr[index]);
    }
}
