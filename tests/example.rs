mod helper;

#[test]
fn test_incr() {
    use rust_by_example::testing::incr;

    helper::setup();

    assert_eq!(incr(2), 3);
}
