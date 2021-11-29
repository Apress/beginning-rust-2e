/* It prints:
10 14*/
fn main() {
    {
        let n = 10;
        {
            let m = 4;
            {
                print!("{} ", n);
            }
            print!("{}", n + m);
        } // End of the scope of `m`
    } // End of the scope of `n`
}
