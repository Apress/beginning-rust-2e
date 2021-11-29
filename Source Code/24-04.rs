/* It prints nothing.
*/
fn main() {
    trait Tr {
        fn f(x: &u8) -> (&u8, &f64, bool, &Vec<String>);
    }
}
