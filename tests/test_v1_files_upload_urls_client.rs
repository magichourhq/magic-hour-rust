#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
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
    println!("{:?}", res);
    assert!(res.is_ok());
}
