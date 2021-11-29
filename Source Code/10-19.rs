/* It prints:
8 / 2 = 4
Cannot divide 8 by 0: Divide by zero
*/
fn main() {
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0. {
            Err(format!("Divide by zero"))
        } else {
            Ok(numerator / denominator)
        }
    }
    fn show_divide(num: f64, den: f64) {
        match divide(num, den) {
            Ok(val) => println!("{} / {} = {}", num, den, val),
            Err(msg) => println!("Cannot divide {} by {}: {}",
                num, den, msg),
        }
    }
    show_divide(8., 2.);
    show_divide(8., 0.);
}
