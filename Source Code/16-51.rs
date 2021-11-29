/* It prints:
"Hello"
['H', 'e', 'l', 'l', 'o']
*/
fn main() {
    let s = "Hello";
    println!("{:?}", s.chars().collect::<String>());
    println!("{:?}", s.chars().collect::<Vec<char>>());
}
