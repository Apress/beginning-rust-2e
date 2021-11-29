/* It prints:
3.1622776601683795 3.1622777
*/
fn main() {
    fn quartic_root_f64(x: f64) -> f64 { x.sqrt().sqrt() }
    fn quartic_root_f32(x: f32) -> f32 { x.sqrt().sqrt() }
    print!("{} {}",
        quartic_root_f64(100f64),
        quartic_root_f32(100f32));
}
