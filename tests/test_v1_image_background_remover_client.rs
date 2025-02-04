#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.8.2");
    let res = client
        .v1()
        .image_background_remover()
        .create(magic_hour::resources::v1::image_background_remover::CreateRequest {
            assets: magic_hour::models::PostV1ImageBackgroundRemoverBodyAssets {
                image_file_path: "image/id/1234.png".to_string(),
            },
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
