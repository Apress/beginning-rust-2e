/* It prints:
4 10*/
fn main() {
    {
        let n = 10;
        {
            let n = 4;
            print!("{} ", n);
        } // End of the scope of the second `n`
        print!("{}", n);
    } // End of the scope of the first `n`
}
