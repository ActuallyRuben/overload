use crate::returns_one;

#[test]
fn returns_one_returns_one() {
    assert_eq!(returns_one(-1, 2), 1);
    assert_eq!(returns_one(1), 1);
    assert_eq!(returns_one(), 1);
}