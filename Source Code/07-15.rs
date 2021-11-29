/* It prints:
Error: X*/
fn main() {
    #[allow(dead_code)]
    enum Result {
        Success(u8),
        Failure(u16, char),
        Uncertainty,
    }
    // let outcome = Result::Success(1);
    let outcome = Result::Failure(20, 'X');
    match outcome {
        Result::Success(0) => print!("Result: 0"),
        Result::Success(1) => print!("Result: 1"),
        Result::Success(_) => print!("Result: other"),
        Result::Failure(10, 'X') => print!("Error: 10 X"),
        Result::Failure(10, _) => print!("Error: 10"),
        Result::Failure(_, 'X') => print!("Error: X"),
        Result::Failure(_, _) => print!("Error: other"),
        Result::Uncertainty => {},
    }
}
