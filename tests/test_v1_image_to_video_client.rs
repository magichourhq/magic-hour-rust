#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .image_to_video()
        .create(magic_hour::resources::v1::image_to_video::CreateRequest {
            assets: magic_hour::models::V1ImageToVideoCreateBodyAssets {
                image_file_path: "api-assets/id/1234.png".to_string(),
            },
            end_seconds: 5.0,
            height: 960,
            style: magic_hour::models::V1ImageToVideoCreateBodyStyle {
                ..Default::default()
            },
            width: 512,
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
