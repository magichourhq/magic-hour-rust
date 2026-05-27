#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .audio_to_video()
        .create(magic_hour::resources::v1::audio_to_video::CreateRequest {
            assets: magic_hour::models::V1AudioToVideoCreateBodyAssets {
                audio_file_path: "api-assets/id/1234.mp3".to_string(),
                image_file_path: Some("api-assets/id/1234.png".to_string()),
            },
            end_seconds: 15.0,
            name: Some("My Audio To Video video".to_string()),
            resolution: Some(
                magic_hour::models::V1AudioToVideoCreateBodyResolutionEnum::Enum720p,
            ),
            start_seconds: Some(0.0),
            style: Some(magic_hour::models::V1AudioToVideoCreateBodyStyle {
                prompt: Some("Car driving through a city".to_string()),
            }),
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
