#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_photo_editor()
        .create(magic_hour::resources::v1::ai_photo_editor::CreateRequest {
            assets: magic_hour::models::V1AiPhotoEditorCreateBodyAssets {
                image_file_path: "api-assets/id/1234.png".to_string(),
            },
            name: Some("My Photo Editor image".to_string()),
            resolution: 768,
            steps: Some(123),
            style: magic_hour::models::V1AiPhotoEditorCreateBodyStyle {
                image_description: "A photo of a person".to_string(),
                likeness_strength: 5.2,
                negative_prompt: Some("painting, cartoon, sketch".to_string()),
                prompt: "A photo portrait of a person wearing a hat".to_string(),
                prompt_strength: 3.75,
                steps: Some(4),
                upscale_factor: Some(2),
                upscale_fidelity: Some(0.5),
            },
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
