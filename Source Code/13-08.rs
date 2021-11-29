/* It prints:
[45, 12, 10, 8, 7, 4, 1, 0]*/
fn main() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    arr.sort_by(|a, b| b.cmp(a));
    print!("{:?}", arr);
}
