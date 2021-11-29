/* It prints:
6 3*/
fn main() {
    trait HasLength {
        fn length(&self) -> usize;
    }
    impl HasLength for str {
        fn length(&self) -> usize { self.chars().count() }
    }
    let s = "€èe";
    print!("{} {}", s.len(), s.length());
}
