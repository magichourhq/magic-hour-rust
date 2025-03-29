#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .text_to_video()
        .create(magic_hour::resources::v1::text_to_video::CreateRequest {
            end_seconds: 5.0,
            orientation: magic_hour::models::V1TextToVideoCreateBodyOrientationEnum::Landscape,
            style: magic_hour::models::V1TextToVideoCreateBodyStyle {
                prompt: "string".to_string(),
            },
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
