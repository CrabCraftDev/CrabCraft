use crate::network::packet_byte_buf::write_byte_buf::WriteByteBuf;

#[test]
fn test_put_var_int() {
    let mut buf = Vec::new();
    buf.put_var_int(500);
    assert_eq!(buf, vec![0xf4, 0x03]);
}

#[test]
fn test_put_var_long() {
    let mut buf = Vec::new();
    buf.put_var_long(i64::pow(10, 10));
    assert_eq!(buf, vec![0x80, 0xc8, 0xaf, 0xa0, 0x25]);
}

#[test]
fn test_put_string() {
    let mut buf = Vec::new();
    buf.put_string("Hello!".to_string());
    assert_eq!(buf, vec![0x06, b'H', b'e', b'l', b'l', b'o', b'!']);
}

#[test]
fn test_put_boolean() {
    let mut buf = Vec::new();
    buf.put_boolean(true);
    assert_eq!(buf, vec![0x01]);

    buf.clear();
    buf.put_boolean(false);
    assert_eq!(buf, vec![0x00]);
}

#[test]
fn test_put_byte_array() {
    let mut buf = Vec::new();
    buf.put_byte_array(&[0x01, 0x02, 0x03]);
    assert_eq!(buf, vec![0x01, 0x02, 0x03]);
}
