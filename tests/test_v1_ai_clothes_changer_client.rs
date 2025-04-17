#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_clothes_changer()
        .create(magic_hour::resources::v1::ai_clothes_changer::CreateRequest {
            assets: magic_hour::models::V1AiClothesChangerCreateBodyAssets {
                garment_file_path: "api-assets/id/outfit.png".to_string(),
                garment_type: magic_hour::models::V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::Dresses,
                person_file_path: "api-assets/id/model.png".to_string(),
            },
            name: Some("Clothes Changer image".to_string()),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
