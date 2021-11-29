/* It prints:
1 4 9 16 25 36 49 64 81 100 */
fn main() {
    for n in 1..=10 {
        print!("{} ", n * n);
    }
}
