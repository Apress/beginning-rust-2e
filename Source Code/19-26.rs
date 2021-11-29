/* It prints:
18.767569280959872 18.767574*/
fn main() {
    trait HasLnExpMultiply {
        fn ln(self) -> Self;
        fn exp(self) -> Self;
        fn multiply(self, other: Self) -> Self;
    }
    impl HasLnExpMultiply for f64 {
        fn ln(self) -> Self { self.ln() }
        fn exp(self) -> Self { self.exp() }
        fn multiply(self, other: Self) -> Self { self * other }
    }
    impl HasLnExpMultiply for f32 {
        fn ln(self) -> Self { self.ln() }
        fn exp(self) -> Self { self.exp() }
        fn multiply(self, other: Self) -> Self { self * other }
    }
    fn exponentiate<Number>(base: Number, exponent: Number) -> Number
    where Number: HasLnExpMultiply
    {
        (base.ln().multiply(exponent)).exp()
    }
    print!("{} {}", exponentiate(2.5, 3.2), exponentiate(2.5f32, 3.2));
}
