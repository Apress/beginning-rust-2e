/* It prints:
other three point */
fn main() {
    match "value" {
        "val" => print!("value "),
        _ => print!("other "),
    }
    match 3 {
        3 => print!("three "),
        4 => print!("four "),
        5 => print!("five "),
        _ => print!("other "),
    }
    match '.' {
        ':' => print!("colon "),
        '.' => print!("point "),
        _ => print!("other "),
    }
}
