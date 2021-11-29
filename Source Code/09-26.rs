/* It prints:
[10, -8, 18, 0, -14, -2, 6, 10, 6, 2]*/
fn main() {
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    for n in 0..10 {
        arr[n] *= 2;
    }
    print!("{:?}", arr);
}
