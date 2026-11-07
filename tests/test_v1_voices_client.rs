#[serial_test::serial]
#[tokio::test]
async fn test_list_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .voices()
        .list(magic_hour::resources::v1::voices::ListRequest {
            cursor: Some("string".to_string()),
            limit: Some(20),
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_200_success_required_only() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .voices()
        .list(magic_hour::resources::v1::voices::ListRequest {
            limit: Some(20),
            ..Default::default()
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
