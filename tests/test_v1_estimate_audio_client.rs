#[serial_test::serial]
#[tokio::test]
async fn test_list_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .estimate()
        .audio()
        .list(magic_hour::resources::v1::estimate::audio::ListRequest {
            tool: magic_hour::models::V1EstimateAudioListToolEnum::AiVoiceGenerator,
            character_count: Some(120),
            duration: Some(123),
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_200_success_required_only() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .estimate()
        .audio()
        .list(magic_hour::resources::v1::estimate::audio::ListRequest {
            tool: magic_hour::models::V1EstimateAudioListToolEnum::AiVoiceGenerator,
            ..Default::default()
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
