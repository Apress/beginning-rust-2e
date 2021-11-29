/* ILLEGAL. The compiler prints the error message:
borrow of moved value: `a2`
*/
fn main() {
    let mut a1 = vec![4, 56, -2];
    let a2 = vec![7, 81, 12500];
    println!("{:?} {:?}", a1, a2);
    a1 = a2;
    println!("{:?}", a1);
    a1[1] = 10;
    println!("{:?} {:?}", a1, a2);
}
