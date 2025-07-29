#[serial_test::serial]
#[tokio::test]
async fn test_get_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .face_detection()
        .get(magic_hour::resources::v1::face_detection::GetRequest {
            id: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .face_detection()
        .create(magic_hour::resources::v1::face_detection::CreateRequest {
            assets: magic_hour::models::V1FaceDetectionCreateBodyAssets {
                target_file_path: "api-assets/id/1234.png".to_string(),
            },
            confidence_score: Some(0.5),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
