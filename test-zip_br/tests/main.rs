use anyhow::Result;
use test_zip_br::*;

#[test]
fn zip_works() -> Result<()> {
    let mut origin = common::Origin::default();
    origin.insert("a".to_string(), b"abcdefg".to_vec());
    origin.insert("b".to_string(), b"abcdefg".to_vec());
    origin.insert("c".to_string(), b"abcdefg".to_vec());

    let archive = archive(origin.clone(), "/tmp/test.tar", 3)?;
    let extracted = extract(archive)?;
    assert_eq!(origin, extracted);

    Ok(())
}
