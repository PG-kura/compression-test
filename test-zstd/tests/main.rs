use anyhow::Result;

#[test]
fn zstd_works() -> Result<()> {
    let mut origin = common::Origin::default();
    origin.insert("a".to_string(), b"abcdefg".to_vec());
    origin.insert("b".to_string(), b"abcdefg".to_vec());
    origin.insert("c".to_string(), b"abcdefg".to_vec());

    let archive = test_zstd::compress(&origin)?;
    let extracted = test_zstd::decompress(&archive)?;
    assert_eq!(origin, extracted);

    Ok(())
}
