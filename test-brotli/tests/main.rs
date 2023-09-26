/*
use anyhow::Result;

#[test]
fn brotli_works() -> Result<()> {
    let mut origin = common::Origin::default();
    origin.insert("a".to_string(), b"abcdefg".to_vec());
    origin.insert("b".to_string(), b"abcdefg".to_vec());
    origin.insert("c".to_string(), b"abcdefg".to_vec());

    let archive = test_brotli::compress(&origin)?;
    let extracted = test_brotli::decompress(&archive)?;
    assert_eq!(origin, extracted);

    Ok(())
}
*/
