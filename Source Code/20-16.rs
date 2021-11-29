/* It prints:
Converted to 1.5
Conversion error: precision loss
Conversion error: too large number
Conversion error: too small number
*/
fn main() {
    struct LargeNumber (f64);
    struct SmallNumber (f32);
    impl TryFrom<LargeNumber> for SmallNumber {
        type Error = String;
        fn try_from(source: LargeNumber) -> Result<Self, Self::Error> {
            if source.0.abs() > f32::MAX as f64 {
                Err("too large number".to_string())
            }
            else if source.0 != 0.
            && source.0.abs() < f32::MIN_POSITIVE as f64 {
                Err("too small number".to_string())
            }
            else {
                let result = source.0 as f32;
                if result as f64 == source.0 {
                    Ok(Self (result))
                }
                else {
                    Err("precision loss".to_string())
                }
            }
        }
    }
    fn show_result(n: Result<SmallNumber, String>) {
        match n {
            Ok(x) => println!("Converted to {}", x.0),
            Err(msg) => println!("Conversion error: {}", msg),
        }
    }
    let ln = LargeNumber (1.5);
    show_result(SmallNumber::try_from(ln));
    let ln = LargeNumber (1. / 3.);
    show_result(SmallNumber::try_from(ln));
    let ln = LargeNumber (1e50);
    show_result(SmallNumber::try_from(ln));
    let ln = LargeNumber (1e-50);
    show_result(SmallNumber::try_from(ln));
}
