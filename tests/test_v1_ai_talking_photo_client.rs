#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_talking_photo()
        .create(magic_hour::resources::v1::ai_talking_photo::CreateRequest {
            assets: magic_hour::models::V1AiTalkingPhotoCreateBodyAssets {
                audio_file_path: "api-assets/id/1234.mp3".to_string(),
                image_file_path: "api-assets/id/1234.png".to_string(),
            },
            end_seconds: 15.0,
            max_resolution: Some(1024),
            name: Some("My Talking Photo image".to_string()),
            start_seconds: 0.0,
            style: Some(magic_hour::models::V1AiTalkingPhotoCreateBodyStyle {
                generation_mode: Some(
                    magic_hour::models::V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Realistic,
                ),
                intensity: Some(123.45),
                prompt: Some("string".to_string()),
            }),
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
