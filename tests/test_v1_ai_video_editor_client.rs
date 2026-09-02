#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_video_editor()
        .create(magic_hour::resources::v1::ai_video_editor::CreateRequest {
            assets: magic_hour::models::V1AiVideoEditorCreateBodyAssets {
                video_file_path: "api-assets/id/1234.mp4".to_string(),
            },
            end_seconds: 5.0,
            model: Some(
                magic_hour::models::V1AiVideoEditorCreateBodyModelEnum::GeminiOmni11,
            ),
            name: Some("My Video Editor video".to_string()),
            resolution: Some(
                magic_hour::models::V1AiVideoEditorCreateBodyResolutionEnum::Enum720p,
            ),
            start_seconds: Some(0.0),
            style: magic_hour::models::V1AiVideoEditorCreateBodyStyle {
                prompt: "Change the car color to blue".to_string(),
            },
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
