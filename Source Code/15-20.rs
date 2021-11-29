/* It prints:
[22, 33] [22, 0] [11, 22, 0, 44]*/
fn main() {
    let mut arr = [11, 22, 33, 44];
    {
        let sl_ref = &mut arr[1..3];
        print!("{:?}", sl_ref);
        sl_ref[1] = 0;
        print!(" {:?}", sl_ref);
    }
    print!(" {:?}", arr);
}
