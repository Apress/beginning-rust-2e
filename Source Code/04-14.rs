/* It prints:
1 4 9 16 25 36 49 64 81 100 121 144 169 196 */
fn main() {
    let mut n = 1;
    loop {
        let nn = n * n;
        if nn >= 200 { break; }
        print!("{} ", nn);
        n += 1;
    }
}
