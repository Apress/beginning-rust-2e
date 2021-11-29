/* It prints:
true
12300000
invalid float literal
*/
fn main() {
    println!("{}", "true".parse::<bool>().unwrap());
    println!("{}", "1.23e7".parse::<f32>().unwrap());
    println!("{}", "1.23y7".parse::<f32>().unwrap_err());
}
