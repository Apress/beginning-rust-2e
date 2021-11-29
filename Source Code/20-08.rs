/* It prints:
2.3 + 6.5i*/
mod complex {
    pub struct Complex {
        pub re: f64,
        pub im: f64,
    }
    impl std::ops::Add for Complex {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self { re: self.re + rhs.re, im: self.im + rhs.im }
        }
    }
}
fn main () {
    use complex::Complex;
    let z1 = Complex { re: 3.8, im: -2.1 };
    let z2 = Complex { re: -1.5, im: 8.6 };
    let z3 = z1 + z2;
    print!("{} + {}i", z3.re, z3.im);
}
