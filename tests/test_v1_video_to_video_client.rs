#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
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
                youtube_url: Some("http://www.example.com".to_string()),
            },
            end_seconds: 15.0,
            fps_resolution: Some(
                magic_hour::models::V1VideoToVideoCreateBodyFpsResolutionEnum::Half,
            ),
            height: magic_hour::Patch::new(123),
            name: Some("My Video To Video video".to_string()),
            start_seconds: 0.0,
            style: magic_hour::models::V1VideoToVideoCreateBodyStyle {
                art_style: magic_hour::models::V1VideoToVideoCreateBodyStyleArtStyleEnum::Enum3dRender,
                model: Some(
                    magic_hour::models::V1VideoToVideoCreateBodyStyleModelEnum::Default,
                ),
                prompt: magic_hour::Patch::new("string".to_string()),
                prompt_type: Some(
                    magic_hour::models::V1VideoToVideoCreateBodyStylePromptTypeEnum::Default,
                ),
                version: Some(
                    magic_hour::models::V1VideoToVideoCreateBodyStyleVersionEnum::Default,
                ),
            },
            width: magic_hour::Patch::new(123),
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
