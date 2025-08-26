#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .files()
        .upload_urls()
        .create(magic_hour::resources::v1::files::upload_urls::CreateRequest {
            items: vec![
                magic_hour::models::V1FilesUploadUrlsCreateBodyItemsItem { extension :
                "mp4".to_string(), type_ :
                magic_hour::models::V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Video
                }, magic_hour::models::V1FilesUploadUrlsCreateBodyItemsItem { extension :
                "mp3".to_string(), type_ :
                magic_hour::models::V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Audio }
            ],
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}

#[serial_test::serial]
#[tokio::test]
async fn test_upload_file_file_not_found() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);

    // Test with a non-existent file path
    let res = client
        .v1()
        .files()
        .upload_file("/non/existent/file.jpg")
        .await;

    println!("{res:?}");
    assert!(res.is_err());

    // Should return an error about not being able to read the file
    let error_msg = format!("{}", res.err().unwrap());
    assert!(error_msg.contains("Failed to read file"));
}

#[serial_test::serial]
#[tokio::test]
async fn test_upload_file_unsupported_extension() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);

    // Create a temporary test file with unsupported extension
    let temp_dir = std::env::temp_dir();
    let test_file_path = temp_dir.join("test_file.xyz");
    std::fs::write(&test_file_path, b"fake content").expect("Failed to create test file");

    let res = client
        .v1()
        .files()
        .upload_file(test_file_path.to_str().unwrap())
        .await;

    // Clean up
    let _ = std::fs::remove_file(&test_file_path);

    println!("{res:?}");
    assert!(res.is_err());

    // Should return an error about unsupported file extension
    let error_msg = format!("{}", res.err().unwrap());
    assert!(error_msg.contains("Unsupported file extension"));
}
