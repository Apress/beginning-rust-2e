/* It prints nothing.
*/
fn main() {
    let mut _n = 12;
    {
        let _ref_n = &_n; // starts an immutable borrow of _n
        let _m = _n; // temporary immutable borrow of _n
        let _k = *_ref_n; // ends the first immutable borrow of _n
    }
    _n = 13; // temporary mutable borrow of _n
    _n += 1; // temporary mutable borrow of _n
}
