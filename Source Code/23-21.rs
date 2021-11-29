/* It prints nothing.
*/
#[allow(unused_variables)]
#[allow(dead_code)]
fn main() {
    fn f<'a>(n: i32, x: &'a Vec<u8>, y: &Vec<u8>) -> &'a u8 {
        if n == 0 { return &x[0]; }
        if n < 0 { &x[1] } else { &x[2] }
    }
}
