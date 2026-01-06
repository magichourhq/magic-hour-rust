#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_image_upscaler()
        .create(magic_hour::resources::v1::ai_image_upscaler::CreateRequest {
            assets: magic_hour::models::V1AiImageUpscalerCreateBodyAssets {
                image_file_path: "api-assets/id/1234.png".to_string(),
            },
            name: Some("My Image Upscaler image".to_string()),
            scale_factor: 2.0,
            style: magic_hour::models::V1AiImageUpscalerCreateBodyStyle {
                enhancement: magic_hour::models::V1AiImageUpscalerCreateBodyStyleEnhancementEnum::Balanced,
                prompt: Some("string".to_string()),
            },
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
