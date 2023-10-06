use anyhow::Result;

#[test]
fn gzip_works() -> Result<()> {
    let mut origin = common::Origin::default();
    origin.insert("a".to_string(), b"abcdefg".to_vec());
    origin.insert("b".to_string(), b"abcdefg".to_vec());
    origin.insert("c".to_string(), b"abcdefg".to_vec());

    let compressed = test_gzip::compress(&origin)?;
    let extracted = test_gzip::decompress(&compressed)?;
    assert_eq!(origin, extracted);

    Ok(())
}
