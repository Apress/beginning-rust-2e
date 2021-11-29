/* It prints nothing.
*/
fn main() {
    fn f<Param1, Param2>(_a: Param1, _b: Param2) {}
    f('a', true);
    f(12.56, "Hello");
    f((3, 'a'), [5, 6, 7]);
}
