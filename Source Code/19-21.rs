/* ILLEGAL. The compiler prints the error message:
only a single inherent implementation marked with `#[lang = "str"]` is allowed for the `str` primitive
*/
fn main() {
    impl str { // ILLEGAL: Inherent implementation for str type
        fn length(&self) -> usize { self.chars().count() }
    }
    let s = "€èe";
    print!("{} {}", s.len(), s.length());
}
