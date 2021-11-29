/* If run with this command line in a Unix-like shell:
./main   first    second
it prints:
[./main][first][second]

If run with this command line in a Windows CMD console:
main   first    second
it prints:
[main][first][second]
*/
fn main() {
    for a in std::env::args() {
        print!("[{}]", a);
    }
}
