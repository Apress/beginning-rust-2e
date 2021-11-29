/* It prints:
2 3 4 That is a sentence*/
fn main() {
    let mut x = vec!["This", "is"]; print!("{}", x.len());
    x.push("a"); print!(" {}", x.len());
    x.push("sentence"); print!(" {}", x.len());
    x[0] = "That";
    for i in 0..x.len() { print!(" {}", x[i]); }
}
