#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_ugc_ad()
        .create(magic_hour::resources::v1::ai_ugc_ad::CreateRequest {
            assets: magic_hour::models::V1AiUgcAdCreateBodyAssets {
                image_file_path: "api-assets/id/image.jpg".to_string(),
                ..Default::default()
            },
            end_seconds: 4.0,
            model: Some(magic_hour::models::V1AiUgcAdCreateBodyModelEnum::Kling30),
            resolution: Some(
                magic_hour::models::V1AiUgcAdCreateBodyResolutionEnum::Enum1080p,
            ),
            script: "Meet your new everyday essential.".to_string(),
            ..Default::default()
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
