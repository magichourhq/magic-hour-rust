#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .image_to_video()
        .create(magic_hour::resources::v1::image_to_video::CreateRequest {
            assets: magic_hour::models::V1ImageToVideoCreateBodyAssets {
                image_file_path: "api-assets/id/1234.png".to_string(),
            },
            end_seconds: 5.0,
            height: magic_hour::Patch::new(123),
            name: Some("My Image To Video video".to_string()),
            resolution: Some(
                magic_hour::models::V1ImageToVideoCreateBodyResolutionEnum::Enum720p,
            ),
            style: Some(magic_hour::models::V1ImageToVideoCreateBodyStyle {
                high_quality: Some(true),
                prompt: Some("a dog running".to_string()),
                quality_mode: Some(
                    magic_hour::models::V1ImageToVideoCreateBodyStyleQualityModeEnum::Quick,
                ),
            }),
            width: magic_hour::Patch::new(123),
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
