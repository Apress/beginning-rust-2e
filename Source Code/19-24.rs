/* It prints:
-2.3 + 0i, -2.1 - 5.2i, -2.2 + 5.2i*/
fn main() {
    struct Complex {
        re: f64,
        im: f64,
    }
    impl std::fmt::Display for Complex {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{} {} {}i",
                self.re,
                if self.im >= 0. { '+' } else { '-' },
                self.im.abs()
            )
        }
    }
    let c1 = Complex { re: -2.3, im: 0. };
    let c2 = Complex { re: -2.1, im: -5.2 };
    let c3 = Complex { re: -2.2, im: 5.2 };
    print!("{}, {}, {}", c1, c2, c3);
}
