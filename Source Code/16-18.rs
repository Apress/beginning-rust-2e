/* ILLEGAL. The compiler prints the error message:
cannot assign twice to immutable variable `iterator`
*/
fn main() {
    let slice1 = &[3, 4, 5];
    let slice2 = &[7, 8];
    let iterator = slice1.iter();
    for item_ref in iterator {
        print!("{} ", *item_ref);
    }
    print!("; ");
    iterator = slice2.iter();
    for item_ref in iterator {
        print!("{} ", *item_ref);
    }
}
