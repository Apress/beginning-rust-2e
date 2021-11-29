/* It prints:
2.3 + 6.5i*/
fn main() {
    struct Complex {
        re: f64,
        im: f64,
    }
    fn add_complex(lhs: Complex, rhs: Complex) -> Complex {
        Complex { re: lhs.re + rhs.re, im: lhs.im + rhs.im }
    }
    let z1 = Complex { re: 3.8, im: -2.1 };
    let z2 = Complex { re: -1.5, im: 8.6 };
    let z3 = add_complex(z1, z2);
    print!("{} + {}i", z3.re, z3.im);
}
