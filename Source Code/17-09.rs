/* This program waits for the user to input a line. If you type:
eè€
and then you press the Enter key, and then you type:
Hello
and then you press the Enter key again, it prints:
First: eè€
Second: Hello
: 28 bytes
*/
fn main() {
    let mut text = format!("First: ");
    let inp = std::io::stdin();
    inp.read_line(&mut text).unwrap();
    text.push_str("Second: ");
    inp.read_line(&mut text).unwrap();
    println!("{}: {} bytes", text, text.len());
}
