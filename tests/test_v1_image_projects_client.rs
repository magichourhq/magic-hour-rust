#[serial_test::serial]
#[tokio::test]
async fn test_delete_204_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .image_projects()
        .delete(magic_hour::resources::v1::image_projects::DeleteRequest {
            id: "cm6pvghix03bvyz0zwash6noj".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .image_projects()
        .get(magic_hour::resources::v1::image_projects::GetRequest {
            id: "cm6pvghix03bvyz0zwash6noj".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
