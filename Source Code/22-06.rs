/* It prints:
3 3*/
fn main() {
    let v1 = vec![11, 22, 33];
    let v2 = v1.clone();
    let v3 = v1;
    // ILLEGAL: print!("{} ", v1.len());
    print!("{} {}", v2.len(), v3.len());
}
