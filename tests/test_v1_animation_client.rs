#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.8.1");
    let res = client
        .v1()
        .animation()
        .create(magic_hour::resources::v1::animation::CreateRequest {
            assets: magic_hour::models::PostV1AnimationBodyAssets {
                audio_file_path: Some("api-assets/id/1234.mp3".to_string()),
                audio_source: magic_hour::models::PostV1AnimationBodyAssetsAudioSourceEnum::File,
                image_file_path: Some("api-assets/id/1234.png".to_string()),
                ..Default::default()
            },
            end_seconds: 15,
            fps: 12,
            height: 960,
            style: magic_hour::models::PostV1AnimationBodyStyle {
                art_style: magic_hour::models::PostV1AnimationBodyStyleArtStyleEnum::PainterlyIllustration,
                camera_effect: magic_hour::models::PostV1AnimationBodyStyleCameraEffectEnum::Accelerate,
                prompt: Some("Cyberpunk city".to_string()),
                prompt_type: magic_hour::models::PostV1AnimationBodyStylePromptTypeEnum::AiChoose,
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
