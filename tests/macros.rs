use hex_macro::hex;

type Seed = [u8; 32];

#[test]
fn test_type_alias() {
    let data: [u8; 32] = [
        0, 14, 7, 13, 5, 5, 1, 6, 5, 6, 10, 15, 3, 2, 15, 11, 14, 13, 12, 10, 0, 9, 12, 13, 8, 11,
        14, 1, 9, 2, 7, 9,
    ];
    const SEED: Seed = hex!("000e070d0505010605060a0f03020f0b0e0d0c0a00090c0d080b0e0109020709");
    assert_eq!(SEED, data);
}

#[test]
fn test_valid_hex() {
    const DATA: [u8; 11] = hex!("48656c6c6f20776f726c64");
    assert_eq!(DATA, *b"Hello world");
}

#[test]
fn test_valid_hex_with_prefix() {
    const DATA: [u8; 11] = hex!("0x48656c6c6f20776f726c64");
    assert_eq!(DATA, *b"Hello world");
}
