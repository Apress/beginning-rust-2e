/* It prints:
-8 -31 */
fn main() {
    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.into_iter().filter(|x_ref| *x_ref < 0) {
        print!("{} ", n);
    }
}
