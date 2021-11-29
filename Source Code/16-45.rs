/* ILLEGAL. The compiler prints the error message:
the trait bound `bool: Sum` is not satisfied
*/
fn main() {
    [3.4].into_iter().sum::<f64>();
    [true].into_iter().sum::<bool>();
}
