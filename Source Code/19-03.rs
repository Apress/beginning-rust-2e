/* ILLEGAL. The compiler prints the error message:
no method named `sqrt` found for type parameter `Number` in the current scope
*/
fn main() {
    fn quartic_root<Number>(x: Number) -> Number {
        x.sqrt().sqrt() // ILLEGAL: no sqrt method for Number
    }
    print!("{} {}",
        quartic_root(100f64),
        quartic_root(100f32));
}
