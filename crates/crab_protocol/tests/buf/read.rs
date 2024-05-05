use bytes::Bytes;

use crab_protocol::buf::ReadByteBuf;

#[test]
fn test_get_var_int() {
    let mut buf = Bytes::from(b"\xf4\x03" as &[u8]);
    let result = buf.get_var_int().unwrap();
    assert_eq!(result, 500);
}

#[test]
fn test_get_var_long() {
    let mut buf = Bytes::from(b"\x80\xc8\xaf\xa0%" as &[u8]);
    let result = buf.get_var_long().unwrap();
    assert_eq!(result, i64::pow(10, 10));
}

#[test]
fn test_get_string() {
    let mut buf: &[u8] = &[0x06, b'H', b'e', b'l', b'l', b'o', b'!'];
    let result = buf.get_string().unwrap();
    assert_eq!(result, "Hello!".to_string());
}

#[test]
fn test_get_boolean() {
    let mut buf: &[u8] = &[0x01];
    let result = buf.get_bool();
    assert!(result);

    let mut buf: &[u8] = &[0x00];
    let result = buf.get_bool();
    assert!(!result);
}
