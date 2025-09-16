#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_voice_generator()
        .create(magic_hour::resources::v1::ai_voice_generator::CreateRequest {
            name: Some("Voice Generator audio".to_string()),
            style: magic_hour::models::V1AiVoiceGeneratorCreateBodyStyle {
                prompt: "Hello, how are you?".to_string(),
                voice_name: magic_hour::models::V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElonMusk,
            },
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
