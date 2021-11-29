/* This program waits for the user to input a line. If you type:
Hello
and then you press the Enter key, it prints:
Ok(6)
[Hello
]
*/
fn main() {
    let mut line = String::new();
    println!("{:?}", std::io::stdin().read_line(&mut line));
    println!("[{}]", line);
}
