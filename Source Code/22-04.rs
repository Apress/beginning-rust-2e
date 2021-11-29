/* ILLEGAL. The compiler prints the error message:
borrow of moved value: `v1`
*/
fn main() {
    let v1 = vec![11, 22, 33];
    #[allow(unused_variables)]
    let v2 = v1;
    print!("{}", v1.len());
}
