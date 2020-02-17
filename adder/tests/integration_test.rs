extern crate adder;

#[test]
fn in_adds_two(){
    assert_eq!(4, adder::add_two(2));
}