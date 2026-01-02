#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_voice_cloner()
        .create(magic_hour::resources::v1::ai_voice_cloner::CreateRequest {
            assets: magic_hour::models::V1AiVoiceClonerCreateBodyAssets {
                audio_file_path: "api-assets/id/1234.mp3".to_string(),
            },
            name: Some("Voice Cloner audio".to_string()),
            style: magic_hour::models::V1AiVoiceClonerCreateBodyStyle {
                prompt: "Hello, this is my cloned voice.".to_string(),
            },
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
