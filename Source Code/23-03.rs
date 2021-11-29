/* ILLEGAL. The compiler prints the error message:
cannot borrow `v` as mutable because it is also borrowed as immutable
*/
fn main() {
    let mut v = vec![12]; // A vector is allocated and initialized
    let ref_to_first = &v[0]; // A reference to it is taken
    v.push(13); // The vector is mutated
    print!("{}", *ref_to_first); // The reference accesses the vector
    // The vector is implicitly deallocated
}
