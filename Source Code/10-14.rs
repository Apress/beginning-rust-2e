/* It prints:
(4.5, 'A') (-6.0, 'g')*/
fn main() {
    fn swap_char_f64(a: char, b: f64) -> (f64, char) { (b, a) }
    let x = swap_char_f64('A', 4.5);
    let y = swap_char_f64('g', -6.);
    print!("{:?} {:?}", x, y);
}
