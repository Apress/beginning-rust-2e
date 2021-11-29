/* It prints:
Error: Err. 2
Error: Err. 4
5
*/
fn main() {
    fn f1(x: i32) -> Result<i32, String> {
        if x == 1 {
            Err(format!("Err. 1"))
        } else {
            Ok(x)
        }
    }
    fn f2(x: i32) -> Result<i32, String> {
        if x == 2 {
            Err(format!("Err. 2"))
        } else {
            Ok(x)
        }
    }
    fn f3(x: i32) -> Result<i32, String> {
        if x == 3 {
            Err(format!("Err. 3"))
        } else {
            Ok(x)
        }
    }
    fn f4(x: i32) -> Result<i32, String> {
        if x == 4 {
            Err(format!("Err. 4"))
        } else {
            Ok(x)
        }
    }
    fn f(x: i32) -> Result<i32, String> {
        let result1 = f1(x);
        if result1.is_err() { return result1; }
        let result2 = f2(result1.unwrap());
        if result2.is_err() { return result2; }
        let result3 = f3(result2.unwrap());
        if result3.is_err() { return result3; }
        f4(result3.unwrap())
    }
    match f(2) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
    match f(4) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
    match f(5) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
}
