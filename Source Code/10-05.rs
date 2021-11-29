/* ILLEGAL. The compiler prints four times the error message:
mismatched types
*/
fn main() {
    // Library code
    fn f<T>(ch: char, num1: T, num2: T) -> T {
        if ch == 'a' { num1 }
        else { num2 }
    }
    // Application code
    let a: i16 = f::<i16>('a', 37.2, 41.1);
    let b: f64 = f::<f64>('b', 37, 41);
    print!("{} {}", a, b);
}
