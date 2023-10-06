use anyhow::Result;

#[test]
fn snappy_works() -> Result<()> {
    let mut origin = common::Origin::default();
    origin.insert("a".to_string(), b"abcdefg".to_vec());
    origin.insert("b".to_string(), b"abcdefg".to_vec());
    origin.insert("c".to_string(), b"abcdefg".to_vec());

    let compressed = test_snappy::compress(&origin)?;
    let extracted = test_snappy::decompress(&compressed)?;
    assert_eq!(origin, extracted);

    Ok(())
}
