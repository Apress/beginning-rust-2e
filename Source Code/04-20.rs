/* It prints:
1234567*/
fn main() {
    print!("1");
    {
        print!("2");
        print!("3");
        {
            print!("4");
            {
                print!("5");
                { { } };
                print!("6");
            }
        }
        print!("7");
    }
}
