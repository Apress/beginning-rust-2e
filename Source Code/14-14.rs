/* It prints:
Hello, world!*/
fn main() {
    let vs = ["Hello", ", ", "world", "!"];
    let mut result = String::new();
    for s in vs {
        result = format!("{}{}", result, s);
    }
    print!("{}", result);
}
