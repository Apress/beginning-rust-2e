/* It prints:
aab*/
fn main() {
    {
        fn f() { print!("a"); }
        f(); f();
    }
    {
        fn f() { print!("b"); }
        f();
    }
}
