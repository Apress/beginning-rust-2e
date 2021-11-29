/* ILLEGAL. The compiler prints the error message:
no method named `abs` found for type parameter `Number` in the current scope
*/
fn main() {
    trait HasSquareRoot { // trait declaration
        fn sq_root(self) -> Self;
    }
    impl HasSquareRoot for f32 { // an implementation of the trait
        fn sq_root(self) -> Self { self.sqrt() }
    }
    impl HasSquareRoot for f64 { // another implementation
        fn sq_root(self) -> Self { self.sqrt() }
    }
    // function that depends on the Number parameter,
    // that must implement the HasSquareRoot trait
    fn quartic_root<Number>(x: Number) -> Number
    where Number: HasSquareRoot {
        x.abs()
    }
    // Here that function if instantiated twice, in both cases
    // using types that implement the HasSquareRoot trait
    print!("{} {}",
        quartic_root(100f64),
        quartic_root(100f32));
}
