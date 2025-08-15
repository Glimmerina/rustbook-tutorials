
//This is the code for the integration test file from the Rustbook.
// It has an error as adder is undefined. Hope the tutorial explains how to fix it.
// Fixed it. It's because it wanted the 
use test_organisation::add_two;
mod common;

#[test]
fn it_adds_two() {
    // Call the setup function from common.rs
    common::setup();
    let result = add_two(2);
    assert_eq!(result, 4);
}