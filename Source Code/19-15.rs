/* It prints:
3.1622776601683795 3.1622777*/
fn main() {
    // Removed the standalone sqrt function.
    trait HasSqrtAndAbs { // Trait renamed.
        fn sqrt(self) -> Self;
        fn abs(self) -> Self; // Added this function signature.
    }
    impl HasSqrtAndAbs for f32 { // Trait renamed.
        fn sqrt(self) -> Self { self.sqrt() }
        fn abs(self) -> Self { self.abs() } // Implemented.
    }
    impl HasSqrtAndAbs for f64 { // Trait renamed.
        fn sqrt(self) -> Self { self.sqrt() }
        fn abs(self) -> Self { self.abs() } // Implemented.
    }
    fn abs_quartic_root<Number>(x: Number) -> Number // Function renamed.
    where Number: HasSqrtAndAbs { // Trait renamed.
        x.abs().sqrt().sqrt() // Added call to abs method.
    }
    print!("{} {}",
        // Function renamed, and argument become negative.
        abs_quartic_root(-100f64),
        // Function renamed, and argument become negative.
        abs_quartic_root(-100f32));
}
