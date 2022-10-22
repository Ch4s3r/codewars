#![feature(int_roundings)]

pub fn century(year: u32) -> u32 {
    (year as f32 / 100.0).ceil() as u32
}

#[test]
fn basic_tests() {
    assert_eq!(century(1705), 18);
    assert_eq!(century(1900), 19);
    assert_eq!(century(1601), 17);
    assert_eq!(century(2000), 20);
    assert_eq!(century(89), 1);
}
