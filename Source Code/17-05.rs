/* It prints many line, each having this pattern:
[NAME]=[Value]
Where NAME is the name of an environment variable, and Value is its current value.
*/
fn main() {
    for var in std::env::vars() {
        println!("[{}]=[{}]", var.0, var.1);
    }
}
