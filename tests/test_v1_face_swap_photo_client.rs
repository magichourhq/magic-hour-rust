#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .face_swap_photo()
        .create(magic_hour::resources::v1::face_swap_photo::CreateRequest {
            assets: magic_hour::models::V1FaceSwapPhotoCreateBodyAssets {
                source_file_path: "api-assets/id/1234.png".to_string(),
                target_file_path: "api-assets/id/1234.png".to_string(),
            },
            name: Some("Face Swap image".to_string()),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
