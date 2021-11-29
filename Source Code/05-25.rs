/* It prints:
This is a sentence
This line is a sentence
This line contains is a sentence
This line contains a sentence
This line contains a sentence about Rust
This line contains a sentence
*/
fn main() {
    let mut x = vec!["This", "is", "a", "sentence"];
    for i in 0..x.len() { print!("{} ", x[i]); } println!();
    x.insert(1, "line");
    for i in 0..x.len() { print!("{} ", x[i]); } println!();
    x.insert(2, "contains");
    for i in 0..x.len() { print!("{} ", x[i]); } println!();
    x.remove(3);
    for i in 0..x.len() { print!("{} ", x[i]); } println!();
    x.push("about Rust");
    for i in 0..x.len() { print!("{} ", x[i]); } println!();
    x.pop();
    for i in 0..x.len() { print!("{} ", x[i]); } println!();
}
