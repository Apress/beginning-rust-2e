/* It prints:
None, Some(22), None, Some(4.3)*/
fn main() {
    fn get_third<Iter>(mut iterator: Iter) -> Option<Iter::Item>
    where
        Iter: Iterator,
    {
        iterator.next();
        iterator.next();
        iterator.next()
    }
    print!(
        "{:?}, {:?}, {:?}, {:?}",
        get_third(10..12),
        get_third(20..29),
        get_third([3.1, 3.2].iter()),
        get_third([4.1, 4.2, 4.3, 4.4].iter()));
}
