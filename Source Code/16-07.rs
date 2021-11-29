/* It prints:
226 130 172 195 168 101 */
fn main() {
    for byte in "€èe".bytes() {
        print!("{} ", byte);
    }
}
