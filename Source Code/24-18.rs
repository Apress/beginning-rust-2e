/* It prints nothing.
*/
fn main() {
    struct TS<'a>(&'a u8);
    enum E<'a, 'b> {
        _A(&'a u8),
        _B,
        _C(bool, &'b f64, char),
        _D(&'static str),
    }
    let byte = 34;
    let _ts = TS(&byte);
    let _e = E::_A(&byte);
}
