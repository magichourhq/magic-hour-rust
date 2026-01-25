#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_image_generator()
        .create(magic_hour::resources::v1::ai_image_generator::CreateRequest {
            aspect_ratio: Some(
                magic_hour::models::V1AiImageGeneratorCreateBodyAspectRatioEnum::Enum11,
            ),
            image_count: 1,
            model: Some(
                magic_hour::models::V1AiImageGeneratorCreateBodyModelEnum::Default,
            ),
            name: Some("My Ai Image image".to_string()),
            orientation: Some(
                magic_hour::models::V1AiImageGeneratorCreateBodyOrientationEnum::Landscape,
            ),
            resolution: Some(
                magic_hour::models::V1AiImageGeneratorCreateBodyResolutionEnum::Auto,
            ),
            style: magic_hour::models::V1AiImageGeneratorCreateBodyStyle {
                prompt: "Cool image".to_string(),
                quality_mode: Some(
                    magic_hour::models::V1AiImageGeneratorCreateBodyStyleQualityModeEnum::Pro,
                ),
                tool: Some(
                    magic_hour::models::V1AiImageGeneratorCreateBodyStyleToolEnum::AiAnimeGenerator,
                ),
            },
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
