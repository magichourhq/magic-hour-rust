#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_meme_generator()
        .create(magic_hour::resources::v1::ai_meme_generator::CreateRequest {
            name: Some("My Funny Meme".to_string()),
            style: magic_hour::models::V1AiMemeGeneratorCreateBodyStyle {
                search_web: Some(false),
                template: magic_hour::models::V1AiMemeGeneratorCreateBodyStyleTemplateEnum::DrakeHotlineBling,
                topic: "When the code finally works".to_string(),
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
