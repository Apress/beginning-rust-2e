/* It prints:
0.33333334*/
fn main() {
    struct LargeNumber (f64);
    struct SmallNumber (f32);
    impl Into<SmallNumber> for LargeNumber {
        fn into(self) -> SmallNumber {
            SmallNumber (self.0 as f32)
        }
    }
    let ln = LargeNumber (1. / 3.);
    let sn: SmallNumber = ln.into();
    print!("{}", sn.0);
}
