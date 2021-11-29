/* ILLEGAL. The compiler prints three error messages:
missing lifetime specifier
missing lifetime specifier
use of undeclared lifetime name `'a`
*/
fn main() {
    struct _S1 { _f: &i32 } // ILLEGAL
    struct _S2<'a> { _f: &i32 } // ILLEGAL
    struct _S3 { _f: &'a i32 } // ILLEGAL
    struct _S4<'a> { _f: &'static i32 } // ILLEGAL
    struct _S5 { _f: &'static i32 }
    struct _S6<'a> { _f: &'a i32 }
}
