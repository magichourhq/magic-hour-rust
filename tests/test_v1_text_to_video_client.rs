#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .text_to_video()
        .create(magic_hour::resources::v1::text_to_video::CreateRequest {
            end_seconds: 5.0,
            name: Some("Text To Video video".to_string()),
            orientation: magic_hour::models::V1TextToVideoCreateBodyOrientationEnum::Landscape,
            resolution: Some(
                magic_hour::models::V1TextToVideoCreateBodyResolutionEnum::Enum1080p,
            ),
            style: magic_hour::models::V1TextToVideoCreateBodyStyle {
                prompt: "a dog running".to_string(),
                quality_mode: Some(
                    magic_hour::models::V1TextToVideoCreateBodyStyleQualityModeEnum::Quick,
                ),
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
