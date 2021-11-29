/* In a 64-bit system, it will print something
different from run to run, but similar to:
0x7ffe16b20c8d 0x7ffe16b20c8e 0x7ffe16b20c8f
In a 32-bit system, it will print three much smaller numbers.
*/
fn main() {
    let b1 = true;
    let b2 = true;
    let b3 = false;
    print!("{:p} {:p} {:p}", &b1, &b2, &b3);
}
