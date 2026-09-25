#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_video_translator()
        .create(magic_hour::resources::v1::ai_video_translator::CreateRequest {
            assets: magic_hour::models::V1AiVideoTranslatorCreateBodyAssets {
                video_file_path: "api-assets/id/1234.mp4".to_string(),
            },
            end_seconds: 15.0,
            name: Some("My Video Translator video".to_string()),
            resolution: Some(
                magic_hour::models::V1AiVideoTranslatorCreateBodyResolutionEnum::Enum720p,
            ),
            start_seconds: Some(0.0),
            target_language: magic_hour::models::V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Spanish,
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
