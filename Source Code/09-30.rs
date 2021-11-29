/* ILLEGAL. The compiler prints the error message:
type `i32` cannot be dereferenced
*/
fn main() {
    fn double(a: &mut [i32; 10]) {
        for n in 0..10 {
            *a[n] *= 2;
        }
    }
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double(&mut arr);
    print!("{:?}", arr);
}
