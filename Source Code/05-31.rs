/* ILLEGAL. The compiler prints the 2 error messages:
`[{integer}; 3]` doesn't implement `std::fmt::Display`
`Vec<{integer}>` doesn't implement `std::fmt::Display`
*/
fn main() {
    print!("{} {}", [1, 2, 3], vec![4, 5]);
}
