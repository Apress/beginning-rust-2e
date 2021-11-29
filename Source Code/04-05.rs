/* It prints:
small*/
fn main() {
    let n = 4;
    print!("{}",
        if n > 1000 {
            "big"
        }
        else if n > 0 {
            "small"
        }
        else if n < 0 {
            "negative"
        }
        else {
            "neither positive nor negative"
        }
    );
}
