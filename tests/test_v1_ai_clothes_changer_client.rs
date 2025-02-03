#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.8.1");
    let res = client
        .v1()
        .ai_clothes_changer()
        .create(magic_hour::resources::v1::ai_clothes_changer::CreateRequest {
            assets: magic_hour::models::PostV1AiClothesChangerBodyAssets {
                garment_file_path: "api-assets/id/outfit.png".to_string(),
                garment_type: magic_hour::models::PostV1AiClothesChangerBodyAssetsGarmentTypeEnum::Dresses,
                person_file_path: "api-assets/id/model.png".to_string(),
            },
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
