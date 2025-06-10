use test_rust::prelude::*;

#[test]
fn it_works() {
    _ = verify_that!(2 + 2, eq(4));
}
