/* It prints nothing.
*/
fn main() {
    trait Tr {
        fn f<'a>(&'a self, y: &u8)
        -> (&'a u8, &'a f64, bool, &'a Vec<String>);
    }
}
