/* It prints:
2.3 + 6.5i*/
mod complex {
    pub struct Complex<Num> {
        re: Num,
        im: Num,
    }
    impl<Num> Complex<Num> {
        pub fn from_re_im(re: Num, im: Num) -> Self {
            Self { re, im }
        }
        pub fn re(&self) -> &Num { &self.re }
        pub fn im(&self) -> &Num { &self.im }
    }
    impl<Num> std::ops::Add for Complex<Num>
    where Num: std::ops::Add<Output = Num> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self { re: self.re + rhs.re, im: self.im + rhs.im }
        }
    }
}
fn main() {
    use complex::Complex;
    let z1 = Complex::from_re_im(3.8, -2.1);
    let z2 = Complex::from_re_im(-1.5, 8.6);
    let z3 = z1 + z2;
    print!("{} + {}i", z3.re(), z3.im());
}
