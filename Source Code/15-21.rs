/* It prints:
[22, 33] [11] [11, 22, 33, 44]*/
fn main() {
    let arr = [11, 22, 33, 44];
    {
        let mut sl_ref = &arr[1..3];
        print!("{:?}", sl_ref);
        sl_ref = &arr[0..1];
        print!(" {:?}", sl_ref);
    }
    print!(" {:?}", arr);
}
