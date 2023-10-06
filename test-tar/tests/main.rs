use anyhow::Result;

#[tokio::test]
async fn tar_works() -> Result<()> {
    let mut origin = common::Origin::default();
    origin.insert("a".to_string(), b"abcdefg".to_vec());
    origin.insert("b".to_string(), b"abcdefg".to_vec());
    origin.insert("c".to_string(), b"abcdefg".to_vec());

    let archive = test_tar::archive(origin.clone(), "/tmp/test.tar").await?;
    let extracted = test_tar::extract(archive)?;
    assert_eq!(origin, extracted);

    Ok(())
}
