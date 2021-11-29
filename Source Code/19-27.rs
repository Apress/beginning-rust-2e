/* It prints:
15.625001
15.625000000000002
15.625001
15.625000000000002
15.625000000000002
*/
fn main() {
    trait HasLnExp {
        fn ln(self) -> Self;
        fn exp(self) -> Self;
    }
    impl HasLnExp for f64 {
        fn ln(self) -> Self { self.ln() }
        fn exp(self) -> Self { self.exp() }
    }
    impl HasLnExp for f32 {
        fn ln(self) -> Self { self.ln() }
        fn exp(self) -> Self { self.exp() }
    }
    trait HasMultiply<Rhs> {
        fn multiply(self, rhs: Rhs) -> Self;
    }
    impl<Rhs> HasMultiply<Rhs> for f64 where Rhs: Into<Self> {
        fn multiply(self, rhs: Rhs) -> Self { self * rhs.into() }
    }
    impl<Rhs> HasMultiply<Rhs> for f32 where Rhs: Into<Self> {
        fn multiply(self, rhs: Rhs) -> Self { self * rhs.into() }
    }
    fn exponentiate<Base, Exponent>(
        base: Base, exponent: Exponent) -> Base
    where Base: HasLnExp + HasMultiply<Exponent>
    {
        (base.ln().multiply(exponent)).exp()
    }
    println!("{}", exponentiate(2.5f32, 3i16));
    println!("{}", exponentiate(2.5f64, 3i16));
    println!("{}", exponentiate(2.5f32, 3f32));
    println!("{}", exponentiate(2.5f64, 3f32));
    println!("{}", exponentiate(2.5f64, 3f64));
}
