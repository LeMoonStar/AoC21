use crate::days::*;

#[test]
fn day01() {
    assert_eq!(d01::Day1::first(&("1".to_string())).unwrap(), 1);
    assert_eq!(d01::Day1::second(&("1".to_string())).unwrap(), 1);
    assert_ne!(d01::Day1::first(&("1".to_string())).unwrap(), 234);
    assert_ne!(d01::Day1::second(&("1".to_string())).unwrap(), 234);
}