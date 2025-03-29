#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .video_to_video()
        .create(magic_hour::resources::v1::video_to_video::CreateRequest {
            assets: magic_hour::models::V1VideoToVideoCreateBodyAssets {
                video_file_path: Some("api-assets/id/1234.mp4".to_string()),
                video_source: magic_hour::models::V1VideoToVideoCreateBodyAssetsVideoSourceEnum::File,
                ..Default::default()
            },
            end_seconds: 15.0,
            height: 960,
            start_seconds: 0.0,
            style: magic_hour::models::V1VideoToVideoCreateBodyStyle {
                art_style: magic_hour::models::V1VideoToVideoCreateBodyStyleArtStyleEnum::Enum3dRender,
                model: magic_hour::models::V1VideoToVideoCreateBodyStyleModelEnum::AbsoluteReality,
                prompt_type: magic_hour::models::V1VideoToVideoCreateBodyStylePromptTypeEnum::AppendDefault,
                version: magic_hour::models::V1VideoToVideoCreateBodyStyleVersionEnum::Default,
                ..Default::default()
            },
            width: 512,
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
