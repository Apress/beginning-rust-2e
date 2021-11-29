/* It prints nothing.
*/
fn main() {
    fn _f1() -> i32 { 4.5; "abc"; 73i32 }
    fn _f2() -> i32 { 4.5; "abc"; 73 }
    fn _f3() -> i32 { 4.5; "abc"; 73 + 100 }
}
