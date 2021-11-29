/* ILLEGAL. The compiler prints the warning message:
function is never used: `f`
Then, the program prints:
232*/
fn f() { print!("1"); }
fn main() {
    f(); // Prints 2
    {
        f(); // Prints 3
        fn f() { print!("3"); }
    }
    f(); // Prints 2
    fn f() { print!("2"); }
}
