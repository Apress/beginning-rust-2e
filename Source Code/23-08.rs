/* It prints nothing.
*/
#[allow(unused_variables)]
fn main() {
    let mut n = 12;
    let ref1_to_n = &mut n;
    let ref2_to_n = &n;
}
