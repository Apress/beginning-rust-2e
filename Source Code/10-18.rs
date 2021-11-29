/* It prints:
Ok(4.0), Err("Divide by zero")*/
fn main() {
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0. {
            Err(format!("Divide by zero"))
        } else {
            Ok(numerator / denominator)
        }
    }
    print!("{:?}, {:?}", divide(8., 2.), divide(8., 0.));
}
