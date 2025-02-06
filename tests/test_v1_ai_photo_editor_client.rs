#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_base_url("https://api.sideko.dev/v1/mock/magichour/magic-hour/0.8.4");
    let res = client
        .v1()
        .ai_photo_editor()
        .create(magic_hour::resources::v1::ai_photo_editor::CreateRequest {
            assets: magic_hour::models::PostV1AiPhotoEditorBodyAssets {
                image_file_path: "api-assets/id/1234.png".to_string(),
            },
            resolution: 768,
            style: magic_hour::models::PostV1AiPhotoEditorBodyStyle {
                image_description: "A photo of a person".to_string(),
                likeness_strength: 5.2,
                negative_prompt: Some("painting, cartoon, sketch".to_string()),
                prompt: "A photo portrait of a person wearing a hat".to_string(),
                prompt_strength: 3.75,
                steps: Some(4),
            },
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
