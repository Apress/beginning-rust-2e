/* The compiler prints the warning message:
denote infinite loops with `loop { ... }`
Then, the program prints:
1 4 9 16 25 36 49 64 81 100 121 144 169 196 */
fn main() {
    let mut n = 1;
    while true {
        let n2 = n * n;
        if n2 >= 200 { break; }
        print!("{} ", n2);
        n += 1;
    }
}
