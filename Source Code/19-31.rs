/* It prints:
None, Some(22)*/
fn main() {
    fn get_third(r: std::ops::Range<u32>) -> Option<u32> {
        if r.len() >= 3 {
            Some(r.start + 2)
        } else {
            None
        }
    }
    print!("{:?}, {:?}", get_third(10..12), get_third(20..29));
}
