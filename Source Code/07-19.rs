/* It prints:
Error: n.20 X*/
fn main() {
    #[allow(dead_code)]
    enum Result {
        Success(u8),
        Failure(u16, char),
        Uncertainty,
    }
    // let outcome = Result::Success(13);
    let outcome = Result::Failure(20, 'X');
    match outcome {
        Result::Success(0) => print!("Result: 0"),
        Result::Success(1) => print!("Result: 1"),
        Result::Success(n) => print!("Result: {}", n),
        Result::Failure(10, 'X') => print!("Error: 10 X"),
        Result::Failure(10, m) => print!("Error: 10 in module {}", m),
        Result::Failure(code, 'X') => print!("Error: n.{} X", code),
        Result::Failure(code, module) =>
            print!("Error: n.{} in module {}", code, module),
        Result::Uncertainty => {},
    }
}
