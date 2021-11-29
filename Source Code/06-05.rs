/* It prints:
1000, 1000000, 13500000000, 0.000012*/
fn main() {
    let one_thousand = 1e3;
    let one_million = 1e6;
    let thirteen_billions_and_half = 13.5e9;
    let twelve_millionths = 12e-6;
    print!(
        "{}, {}, {}, {}",
        one_thousand, one_million, thirteen_billions_and_half, twelve_millionths
    );
}
