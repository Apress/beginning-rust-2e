/* It prints:
3 4 5 6 */
fn main() {
    for i in 3.. {
        if i * i > 40 { break; }
        print!("{} ", i);
    }
}
