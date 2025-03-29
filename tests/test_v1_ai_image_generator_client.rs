#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_image_generator()
        .create(magic_hour::resources::v1::ai_image_generator::CreateRequest {
            image_count: 1,
            orientation: magic_hour::models::V1AiImageGeneratorCreateBodyOrientationEnum::Landscape,
            style: magic_hour::models::V1AiImageGeneratorCreateBodyStyle {
                prompt: "Cool image".to_string(),
            },
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
