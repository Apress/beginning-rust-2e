/* It prints:
1 4 16 25 49 64 100 121 169 196 256 289 361 400 */
fn main() {
    let mut n = 0;
    while n < 50 {
        n += 1;
        if n % 3 == 0 { continue; }
        if n * n > 400 { break; }
        print!("{} ", n * n);
    }
}
