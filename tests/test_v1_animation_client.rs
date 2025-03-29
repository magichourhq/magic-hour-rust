#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .animation()
        .create(magic_hour::resources::v1::animation::CreateRequest {
            assets: magic_hour::models::V1AnimationCreateBodyAssets {
                audio_file_path: Some("api-assets/id/1234.mp3".to_string()),
                audio_source: magic_hour::models::V1AnimationCreateBodyAssetsAudioSourceEnum::File,
                image_file_path: Some("api-assets/id/1234.png".to_string()),
                ..Default::default()
            },
            end_seconds: 15.0,
            fps: 12.0,
            height: 960,
            style: magic_hour::models::V1AnimationCreateBodyStyle {
                art_style: magic_hour::models::V1AnimationCreateBodyStyleArtStyleEnum::PainterlyIllustration,
                camera_effect: magic_hour::models::V1AnimationCreateBodyStyleCameraEffectEnum::Accelerate,
                prompt: Some("Cyberpunk city".to_string()),
                prompt_type: magic_hour::models::V1AnimationCreateBodyStylePromptTypeEnum::AiChoose,
                transition_speed: 5,
                ..Default::default()
            },
            width: 512,
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
