#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.7.2");
    let res = client
        .v1()
        .ai_image_generator()
        .create(magic_hour::resources::v1::ai_image_generator::CreateRequest {
            image_count: 1,
            orientation: magic_hour::models::PostV1AiImageGeneratorBodyOrientationEnum::Landscape,
            style: magic_hour::models::PostV1AiImageGeneratorBodyStyle {
                prompt: "Cool image".to_string(),
            },
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
