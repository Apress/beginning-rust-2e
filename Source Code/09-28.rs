/* It prints:
[10, -8, 18, 0, -14, -2, 6, 10, 6, 2]*/
fn main() {
    fn double(mut a: [i32; 10]) -> [i32; 10] {
        for n in 0..10 {
            a[n] *= 2;
        }
        a
    }
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    arr = double(arr);
    print!("{:?}", arr);
}
