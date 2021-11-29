/* In a 64-bit system, it will print something
different from run to run, but similar to:
140729279188109 140729279188110 140729279188111
In a 32-bit system, it will print three much smaller numbers.
*/
fn main() {
    let b1 = true;
    let b2 = true;
    let b3 = false;
    print!("{} {} {}",
        &b1 as *const bool as usize,
        &b2 as *const bool as usize,
        &b3 as *const bool as usize);
}
