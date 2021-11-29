/* It prints:
226 130 172 195 168 101 */
fn main() {
    let string: &str = "€èe";
    let string_it: std::str::Bytes = string.bytes();
    for byte in string_it {
        print!("{} ", byte);
    }
}
