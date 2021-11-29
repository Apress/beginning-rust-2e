/* It prints:
Undefined, This is the value*/
fn main() {
    print!("{}",
        if std::env::var("abcd").is_ok() {
            "Already defined"
        } else {
            "Undefined"
        });
    std::env::set_var("abcd", "This is the value");
    print!(", {}", match std::env::var("abcd") {
        Ok(value) => value,
        Err(err) => format!("Still undefined: {}", err),
    });
}
