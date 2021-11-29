/* It prints:
"Hello, world!" does not contain 'R'.*/
fn main() {
    let s = "Hello, world!";
    let ch = 'R';
    print!("\"{}\" {} '{}'.",
        s,
        if s.chars().any(|c| c == ch) {
            "contains"
        } else {
            "does not contain"
        },
        ch);
}
