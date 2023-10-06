use anyhow::Result;

#[tokio::test]
async fn zip_works() -> Result<()> {
    let mut origin = common::Origin::default();
    origin.insert("a".to_string(), b"abcdefg".to_vec());
    origin.insert("b".to_string(), b"abcdefg".to_vec());
    origin.insert("c".to_string(), b"abcdefg".to_vec());

    let archive = test_zip_br::archive(origin.clone(), "/tmp/test.tar").await?;
    let extracted = test_zip_br::extract(archive)?;
    assert_eq!(origin, extracted);

    Ok(())
}
