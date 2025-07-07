#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_qr_code_generator()
        .create(magic_hour::resources::v1::ai_qr_code_generator::CreateRequest {
            content: "https://magichour.ai".to_string(),
            name: Some("Qr Code image".to_string()),
            style: magic_hour::models::V1AiQrCodeGeneratorCreateBodyStyle {
                art_style: "Watercolor".to_string(),
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
