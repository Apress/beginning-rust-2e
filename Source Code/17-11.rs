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
        match f1(x) {
            Ok(result) => {
                match f2(result) {
                    Ok(result) => {
                        match f3(result) {
                            Ok(result) => f4(result),
                            Err(err_msg) => Err(err_msg),
                        }
                    }
                    Err(err_msg) => Err(err_msg),
                }
            }
            Err(err_msg) => Err(err_msg),
        }
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
