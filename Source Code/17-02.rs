/* If run with this command line in a Unix-like shell:
./main " first argument" "second argument "
it prints:
[./main][ first argument][second argument ]

If run with this command line in a Windows CMD console:
main " first argument" "second argument "
it prints:
[main][ first argument][second argument ]
*/
fn main() {
    let command_line: std::env::Args = std::env::args();
    for argument in command_line {
        print!("[{}]", argument);
    }
}
