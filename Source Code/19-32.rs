/* It prints:
None Some(4.3)*/
fn main() {
    fn get_third(s: &[f64]) -> Option<f64> {
        if s.len() >= 3 {
            Some(s[2])
        } else {
            None
        }
    }
    print!("{:?}, {:?}",
        get_third(&[3.1, 3.2]),
        get_third(&[4.1, 4.2, 4.3, 4.4]));
}
