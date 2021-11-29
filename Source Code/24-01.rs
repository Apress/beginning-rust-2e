/* It prints nothing.
*/
fn main() {
    trait Tr {
        fn f<'a>(x: &'a u8, b: bool) -> &'a u8;
    }
}
