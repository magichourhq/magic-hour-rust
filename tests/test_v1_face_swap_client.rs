#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.8.3");
    let res = client
        .v1()
        .face_swap()
        .create(magic_hour::resources::v1::face_swap::CreateRequest {
            assets: magic_hour::models::PostV1FaceSwapBodyAssets {
                image_file_path: "image/id/1234.png".to_string(),
                video_file_path: Some("video/id/1234.mp4".to_string()),
                video_source: magic_hour::models::PostV1FaceSwapBodyAssetsVideoSourceEnum::File,
                ..Default::default()
            },
            end_seconds: 15.0,
            height: 960,
            start_seconds: 0.0,
            width: 512,
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
