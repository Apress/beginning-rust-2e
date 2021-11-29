/* It prints:
16775075 1234567 134023794 27121*/
fn main() {
    let hexadecimal = 0x_00FF_F7A3;
    let decimal = 1_234_567;
    let octal = 0o_777_205_162;
    let binary = 0b_0110_1001_1111_0001;
    print!("{} {} {} {}", hexadecimal, decimal, octal, binary);
}
