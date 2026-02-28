use kourain_core::Bitwise;
#[test]
fn bitwise_test() {
    let mut num: u8 = 0b0000_0001; // bit 0 is active
    assert!(num.is_bit_active(0));
    assert!(!num.is_bit_active(1));
    assert!(num.active_bit(1)); // activate bit 1
    assert!(num.is_bit_active(1));
    assert!(!num.active_bit(1)); // bit 1 is already active, should return false
}