/* It prints:
6 3*/
fn main() {
    fn length(s: &str) -> usize { s.chars().count() }
    let s = "€èe";
    print!("{} {}", s.len(), length(s));
}
