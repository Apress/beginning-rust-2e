/* It prints:
[36, 1, 15, 9, 4]*/
fn main() {
    let arr = [36, 1, 15, 9, 4];
    let v: Vec<_> = arr.into_iter().collect();
    print!("{:?}", v);
}
