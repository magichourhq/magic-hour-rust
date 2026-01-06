#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_gif_generator()
        .create(magic_hour::resources::v1::ai_gif_generator::CreateRequest {
            name: Some("My Ai Gif gif".to_string()),
            output_format: Some(
                magic_hour::models::V1AiGifGeneratorCreateBodyOutputFormatEnum::Gif,
            ),
            style: magic_hour::models::V1AiGifGeneratorCreateBodyStyle {
                prompt: "Cute dancing cat, pixel art".to_string(),
            },
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
