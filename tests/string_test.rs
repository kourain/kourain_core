#[test]
fn to_slug_test() {
    assert_eq!(kourain_core::to_slug("  Hello World!  "), "hello-world");
    assert_eq!(kourain_core::to_slug("Đây là một chuỗi có dấu"), "day-la-mot-chuoi-co-dau");
    assert_eq!(kourain_core::to_slug("Café au lait"), "cafe-au-lait");
    assert_eq!(kourain_core::to_slug("Năm 2021"), "nam-2021");
    assert_eq!(kourain_core::to_slug("   Multiple   spaces   "), "multiple-spaces");
}