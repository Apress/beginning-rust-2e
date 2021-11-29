/* It prints:
Ok(true)
Ok(12300000.0)
Err(ParseFloatError { kind: Invalid })
*/
fn main() {
    println!("{:?}", "true".parse::<bool>());
    println!("{:?}", "1.23e7".parse::<f32>());
    println!("{:?}", "1.23y7".parse::<f32>());
}
