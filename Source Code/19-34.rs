/* It prints:
None, Some(22), None, Some(4.3)*/
fn main() {
    print!(
        "{:?}, {:?}, {:?}, {:?}",
        (10..12).nth(2),
        (20..29).nth(2),
        ([3.1, 3.2].iter()).nth(2),
        ([4.1, 4.2, 4.3, 4.4].iter()).nth(2));
}
