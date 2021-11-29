/* It prints:
 Ok(300) Ok(210) Err("Negative argument")*/
fn main() {
    fn f1(n: i32) -> Result<i64, String> {
        Ok(f2(n * 2)? as i64 * 3)
    }
    fn f2(n: i32) -> Result<i32, String> {
        if n >= 0 {
            Ok(n * 5)
        } else {
            Err("Negative argument".to_string())
        }
    }
    print!("{:?} ", f1(10));
    print!("{:?} ", f1(7));
    print!("{:?} ", f1(-1));
}
