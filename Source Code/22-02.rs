/* It prints:
3*/
fn main() {
    let a = 3;
    {
        let _a_ref = &a;
    }
    print!("{}", a);
}
