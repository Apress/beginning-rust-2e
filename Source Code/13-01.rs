/* It prints:
[0, 1, 4, 7, 8, 10, 12, 45]*/
fn main() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    arr.sort();
    print!("{:?}", arr);
}
