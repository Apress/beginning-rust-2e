/* ILLEGAL. The compiler prints twice the error message:
mismatched types
both one with the explanation:
expected `i16`, found floating-point number
*/
fn main() {
    // Library code
    fn f(ch: char, num1: i16, num2: i16) -> i16 {
        if ch == 'a' { num1 }
        else { num2 }
    }
    // Application code
    print!("{}", f('a', 37.2, 41.));
}
