/* If launched with the name of an existing text file
as command-line argument, it prints:
file: filename
n. of lines: N1
n. of empty lines: N2

where filename is the specified file name
N1 is the number of lines contained in the file
N1 is the number of empty lines contained in the file
*/
fn main() {
    let mut command_line = std::env::args();
    command_line.next();
    let pathname = command_line.next().unwrap();
    let counts = count_lines(&pathname).unwrap();
    println!("file: {}", pathname);
    println!("n. of lines: {}", counts.0);
    println!("n. of empty lines: {}", counts.1);

    fn count_lines(pathname: &str)
    -> Result<(u32, u32), std::io::Error> {
        use std::io::BufRead;
        let f = std::fs::File::open(pathname)?;
        let f = std::io::BufReader::new(f);
        let mut n_lines = 0;
        let mut n_empty_lines = 0;
        for line in f.lines() {
            n_lines += 1;
            if line?.trim().len() == 0 {
                n_empty_lines += 1;
            }
        }
        Ok((n_lines, n_empty_lines))
    }
}
