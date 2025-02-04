#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.8.2");
    let res = client
        .v1()
        .text_to_video()
        .create(magic_hour::resources::v1::text_to_video::CreateRequest {
            end_seconds: 5.0,
            orientation: magic_hour::models::PostV1TextToVideoBodyOrientationEnum::Landscape,
            style: magic_hour::models::PostV1TextToVideoBodyStyle {
                prompt: "string".to_string(),
            },
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
