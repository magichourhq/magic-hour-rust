#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .face_swap()
        .create(magic_hour::resources::v1::face_swap::CreateRequest {
            assets: magic_hour::models::V1FaceSwapCreateBodyAssets {
                image_file_path: "image/id/1234.png".to_string(),
                video_file_path: Some("api-assets/id/1234.mp4".to_string()),
                video_source: magic_hour::models::V1FaceSwapCreateBodyAssetsVideoSourceEnum::File,
                ..Default::default()
            },
            end_seconds: 15.0,
            height: 960,
            name: Some("Face Swap video".to_string()),
            start_seconds: 0.0,
            width: 512,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
