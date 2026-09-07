#[serial_test::serial]
#[tokio::test]
async fn test_list_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client.v1().account().list().await;
    println!("{res:?}");
    assert!(res.is_ok());
}
