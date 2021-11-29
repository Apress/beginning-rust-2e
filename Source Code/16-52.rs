/* It prints:
[72, 101, 108, 108, 111]
[72, 101, 108, 108, 111]
*/
fn main() {
    let s = "Hello";
    println!("{:?}", s.bytes().collect::<Vec<u8>>());
    println!("{:?}", s.as_bytes().iter().collect::<Vec<&u8>>());
}
