/* It prints:
€: 8364
è: 232
e: 101
*/
fn main() {
    fn print_codes(s: &str) {
        for c in s.chars() {
            println!("{}: {}", c, c as u32);
        }
    }
    print_codes("€èe");
}
