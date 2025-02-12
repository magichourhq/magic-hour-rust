#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_headshot_generator()
        .create(magic_hour::resources::v1::ai_headshot_generator::CreateRequest {
            assets: magic_hour::models::PostV1AiHeadshotGeneratorBodyAssets {
                image_file_path: "api-assets/id/1234.png".to_string(),
            },
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
