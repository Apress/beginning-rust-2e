/* It prints nothing,
but it tries to copy the file whose name is the first command-line argument
to the file whose name is the second command-line argument
*/
fn main() {
    use std::io::Read;
    use std::io::Write;
    let mut command_line: std::env::Args = std::env::args();
    command_line.next().unwrap();
    let source = command_line.next().unwrap();
    let destination = command_line.next().unwrap();
    let mut file_in = std::fs::File::open(source).unwrap();
    let mut file_out = std::fs::File::create(destination).unwrap();
    let mut buffer = [0u8; 4096];loop {
        let nbytes = file_in.read(&mut buffer).unwrap();
        file_out.write_all(&buffer[..nbytes]).unwrap();
        if nbytes < buffer.len() { break; }
    }
}
