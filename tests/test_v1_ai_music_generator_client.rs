#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_music_generator()
        .create(magic_hour::resources::v1::ai_music_generator::CreateRequest {
            duration: 120,
            name: Some("My Music Generator audio".to_string()),
            style: magic_hour::models::V1AiMusicGeneratorCreateBodyStyle {
                instrumental: Some(false),
                lyrics: Some("City lights are fading...".to_string()),
                prompt: "Dreamy synthwave with warm bass and soft vocals".to_string(),
                ..Default::default()
            },
            ..Default::default()
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
