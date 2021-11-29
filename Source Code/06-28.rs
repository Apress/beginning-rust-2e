/* It prints many lines. The first two lines are:
32: [ ]
33: [!]
in the middle there are the four lines:
125: [}]
126: [~]
160: [ ]
161: [¡]
and the last two lines are:
254: [þ]
255: [ÿ]
*/
fn main() {
    for n in 32..127 {
        println!("{}: [{}]", n, n as u8 as char);
    }
    for n in 160..256 {
        println!("{}: [{}]", n, n as u8 as char);
    }
}
