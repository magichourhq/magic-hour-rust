#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.8.5");
    let res = client
        .v1()
        .ai_image_upscaler()
        .create(magic_hour::resources::v1::ai_image_upscaler::CreateRequest {
            assets: magic_hour::models::PostV1AiImageUpscalerBodyAssets {
                image_file_path: "api-assets/id/1234.png".to_string(),
            },
            scale_factor: 123.45,
            style: magic_hour::models::PostV1AiImageUpscalerBodyStyle {
                enhancement: magic_hour::models::PostV1AiImageUpscalerBodyStyleEnhancementEnum::Balanced,
                ..Default::default()
            },
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
