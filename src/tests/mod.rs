use crate::days::*;

#[test]
fn day01() {
    let d = d01::Day1::new(&("123".to_string()));
    assert_eq!(d.first().unwrap(), 123);
    assert_eq!(d.second().unwrap(), 123);
    assert_ne!(d.first().unwrap(), 1);
    assert_ne!(d.second().unwrap(), 1);
}
