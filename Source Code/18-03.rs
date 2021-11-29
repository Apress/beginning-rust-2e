/* It prints:
[7] [7]*/
fn main() {
    let mut v1 = vec![0u8; 0];
    let mut v2 = vec![0u8; 0];
    v1.push(7);
    Vec::push(&mut v2, 7);
    print!("{:?} {:?}", v1, v2);
}
