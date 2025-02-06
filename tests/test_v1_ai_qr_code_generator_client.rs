#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.8.4");
    let res = client
        .v1()
        .ai_qr_code_generator()
        .create(magic_hour::resources::v1::ai_qr_code_generator::CreateRequest {
            content: "https://magichour.ai".to_string(),
            style: magic_hour::models::PostV1AiQrCodeGeneratorBodyStyle {
                art_style: "Watercolor".to_string(),
            },
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
