#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.8.5");
    let res = client
        .v1()
        .image_to_video()
        .create(magic_hour::resources::v1::image_to_video::CreateRequest {
            assets: magic_hour::models::PostV1ImageToVideoBodyAssets {
                image_file_path: "api-assets/id/1234.png".to_string(),
            },
            end_seconds: 5.0,
            height: 960,
            style: magic_hour::models::PostV1ImageToVideoBodyStyle {
                ..Default::default()
            },
            width: 512,
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
