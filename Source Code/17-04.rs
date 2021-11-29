/* It prints nothing, but it returns 107 to the launching process.
In a Unix-line shell, if you run this program and then type the command:
echo $?
it prints:
107
In a Windows CMD console, if you run this program and then type the command:
echo %ErrorLevel%
it prints:
107
*/
fn main() {
    std::process::exit(107);
}
