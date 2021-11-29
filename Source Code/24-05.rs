/* It prints nothing.
*/
fn main() {
    trait Tr {
        fn f<'a>(x: &'a u8) -> (&u8, &'a f64, bool, &'static Vec<String>);
    }
}
