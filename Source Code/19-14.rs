/* It prints:
NaN Nan*/
fn main() {
    fn sqrt() {}
    trait HasSquareRoot {
        fn sqrt(self) -> Self;
    }
    impl HasSquareRoot for f32 {
        fn sqrt(self) -> Self { self.sqrt() }
    }
    impl HasSquareRoot for f64 {
        fn sqrt(self) -> Self { self.sqrt() }
    }
    fn quartic_root<Number>(x: Number) -> Number
    where Number: HasSquareRoot {
        x.sqrt().sqrt()
    }
    sqrt();
    print!("{} {}",
        quartic_root(-100f64),
        quartic_root(-100f32));
}
