#[serial_test::serial]
#[tokio::test]
async fn test_delete_204_generated_success() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.8.1");
    let res = client
        .v1()
        .video_projects()
        .delete(magic_hour::resources::v1::video_projects::DeleteRequest {
            id: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_200_generated_success() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.8.1");
    let res = client
        .v1()
        .video_projects()
        .get(magic_hour::resources::v1::video_projects::GetRequest {
            id: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
