#[serial_test::serial]
#[tokio::test]
async fn test_list_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .estimate()
        .video()
        .list(magic_hour::resources::v1::estimate::video::ListRequest {
            end_seconds: 5.0,
            tool: magic_hour::models::V1EstimateVideoListToolEnum::ImageToVideo,
            audio: Some(true),
            has_video_reference: Some(true),
            model: Some("kling-3.0".to_string()),
            resolution: Some("720p".to_string()),
            start_seconds: Some(123.45),
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
        .video()
        .list(magic_hour::resources::v1::estimate::video::ListRequest {
            end_seconds: 5.0,
            tool: magic_hour::models::V1EstimateVideoListToolEnum::ImageToVideo,
            ..Default::default()
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
