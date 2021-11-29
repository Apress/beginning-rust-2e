/* It may print:
0 0
1 4
2 4
3 4
4 4
5 8
*/
fn main() {
    let mut v = vec![0; 0];
    println!("{} {}", v.len(), v.capacity());
    v.push(11);
    println!("{} {}", v.len(), v.capacity());
    v.push(22);
    println!("{} {}", v.len(), v.capacity());
    v.push(33);
    println!("{} {}", v.len(), v.capacity());
    v.push(44);
    println!("{} {}", v.len(), v.capacity());
    v.push(55);
    println!("{} {}", v.len(), v.capacity());
}
