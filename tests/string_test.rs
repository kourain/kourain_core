use kourain_core::*;
#[test]
fn to_slug_test() {
    assert_eq!("  Hello World!  ".to_slug(), "hello-world");
    assert_eq!("Đây là một chuỗi có dấu".to_slug(), "day-la-mot-chuoi-co-dau");
    assert_eq!("Café au lait".to_slug(), "cafe-au-lait");
    assert_eq!("Năm 2021".to_slug(), "nam-2021");
    assert_eq!("   Multiple   spaces   ".to_slug(), "multiple-spaces");
}