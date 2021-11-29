/* It prints nothing.
*/
fn main() {
    trait Tr {
        fn f<'a>(&self, y: &'a u8)
        -> (&u8, &'a f64, bool, &Vec<String>);
    }
}
