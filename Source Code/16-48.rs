/* It prints:
[36, 1, 15, 9, 4]*/
fn main() {
    let arr = [36, 1, 15, 9, 4];
    let v = arr.into_iter().collect::<Vec<i32>>();
    print!("{:?}", v);
}
