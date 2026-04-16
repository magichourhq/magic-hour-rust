#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .body_swap()
        .create(magic_hour::resources::v1::body_swap::CreateRequest {
            assets: magic_hour::models::V1BodySwapCreateBodyAssets {
                person_file_path: "api-assets/id/1234.png".to_string(),
                scene_file_path: "api-assets/id/5678.png".to_string(),
            },
            name: Some("My Body Swap image".to_string()),
            resolution: magic_hour::models::V1BodySwapCreateBodyResolutionEnum::Enum1k,
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
