/* It prints:
€e*/
fn main() {
    fn print_nth_char(s: &str, mut n: u32) {
        let mut iter: std::str::Chars = s.chars();
        loop {
            let item: Option<char> = iter.next();
            match item {
                Some(c) => if n == 0 { print!("{}", c); break; },
                None => { break; },
            }
            n -= 1;
        }
    }
    print_nth_char("€èe", 0); // It prints: €
    print_nth_char("€èe", 2); // It prints: e
}
