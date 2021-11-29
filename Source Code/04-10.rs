/* It prints:
1 4 9 16 25 36 49 64 81 100 */
fn main() {
    let mut n = 1;
    while n <= 10 {
        print!("{} ", n * n);
        n += 1;
    }
}
