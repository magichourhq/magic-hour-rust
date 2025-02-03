#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.8.0");
    let res = client
        .v1()
        .files()
        .upload_urls()
        .create(magic_hour::resources::v1::files::upload_urls::CreateRequest {
            items: vec![
                magic_hour::models::PostV1FilesUploadUrlsBodyItemsItem { extension :
                "mp4".to_string(), type_field :
                magic_hour::models::PostV1FilesUploadUrlsBodyItemsItemTypeEnum::Video },
                magic_hour::models::PostV1FilesUploadUrlsBodyItemsItem { extension :
                "mp3".to_string(), type_field :
                magic_hour::models::PostV1FilesUploadUrlsBodyItemsItemTypeEnum::Audio }
            ],
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
