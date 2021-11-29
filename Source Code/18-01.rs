/* It prints:
abcd abcd*/
fn main() {
    print!("{} {}",
        "abcd".to_string(),
        std::string::ToString::to_string("abcd"));
}
