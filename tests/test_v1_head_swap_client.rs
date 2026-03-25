#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .head_swap()
        .create(magic_hour::resources::v1::head_swap::CreateRequest {
            assets: magic_hour::models::V1HeadSwapCreateBodyAssets {
                body_file_path: "api-assets/id/1234.png".to_string(),
                head_file_path: "api-assets/id/5678.png".to_string(),
            },
            max_resolution: Some(1024),
            name: Some("My Head Swap image".to_string()),
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
