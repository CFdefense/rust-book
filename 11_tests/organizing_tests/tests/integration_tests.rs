// Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope.
use adder::add_two;

// no need to annotate this with cfg as rust treats the tests directory special
#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}

use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}
