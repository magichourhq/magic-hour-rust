#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .lip_sync()
        .create(magic_hour::resources::v1::lip_sync::CreateRequest {
            assets: magic_hour::models::V1LipSyncCreateBodyAssets {
                audio_file_path: "api-assets/id/1234.mp3".to_string(),
                video_file_path: Some("api-assets/id/1234.mp4".to_string()),
                video_source: magic_hour::models::V1LipSyncCreateBodyAssetsVideoSourceEnum::File,
                ..Default::default()
            },
            end_seconds: 15.0,
            height: 960,
            max_fps_limit: Some(12.0),
            name: Some("Lip Sync video".to_string()),
            start_seconds: 0.0,
            width: 512,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
