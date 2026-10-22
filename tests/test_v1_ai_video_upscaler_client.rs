#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_video_upscaler()
        .create(magic_hour::resources::v1::ai_video_upscaler::CreateRequest {
            assets: magic_hour::models::V1AiVideoUpscalerCreateBodyAssets {
                video_file_path: "api-assets/id/1234.mp4".to_string(),
            },
            end_seconds: 5.0,
            name: Some("My Video Upscaler video".to_string()),
            resolution: magic_hour::models::V1AiVideoUpscalerCreateBodyResolutionEnum::Enum1080p,
            start_seconds: Some(0.0),
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
