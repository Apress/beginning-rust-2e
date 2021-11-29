/* It prints:
26 26 26 26 26 26*/
fn main() {
    let factor = 2;
    let multiply = |a| a * factor;
    print!("{}", multiply(13));
    let multiply_ref = &multiply;
    print!(
        " {} {} {} {} {}",
        (*multiply_ref)(13),
        multiply_ref(13),
        (|a| a * factor)(13),
        (|a: i32| a * factor)(13),
        |a| -> i32 { a * factor }(13));
}
