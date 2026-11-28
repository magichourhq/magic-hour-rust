#[serial_test::serial]
#[tokio::test]
async fn test_list_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .estimate()
        .image()
        .list(magic_hour::resources::v1::estimate::image::ListRequest {
            tool: magic_hour::models::V1EstimateImageListToolEnum::AiImageEditor,
            image_count: Some(4),
            mode: Some("string".to_string()),
            model: Some("nano-banana-2.1".to_string()),
            resolution: Some("2k".to_string()),
            scale_factor: Some(123.45),
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
        .estimate()
        .image()
        .list(magic_hour::resources::v1::estimate::image::ListRequest {
            tool: magic_hour::models::V1EstimateImageListToolEnum::AiImageEditor,
            ..Default::default()
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
