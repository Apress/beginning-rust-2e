/* ILLEGAL. The compiler prints the error message:
unterminated block comment
*/
fn main() {}
/* This /* instead is not allowed in Rust,
while in C is tolerated (but it may generate a warning).*/
