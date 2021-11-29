/* It prints nothing.
*/
fn main() {
    trait Tr {
        fn f(&self, y: &u8)
        -> (&u8, &f64, bool, &Vec<String>);
    }
}
