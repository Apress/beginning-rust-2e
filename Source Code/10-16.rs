/* ILLEGAL. The compiler prints the error message:
mismatched types
with the explanation:
expected `u16`, found `u32`
*/
fn main() {
    enum Result1<SuccessCode, FailureCode> {
        Success(SuccessCode),
        Failure(FailureCode, char),
        Uncertainty,
    }
    let mut _res = Result1::Success::<u32, u16>(12u32);
    _res = Result1::Uncertainty;
    _res = Result1::Failure(0u32, 'd');
}
