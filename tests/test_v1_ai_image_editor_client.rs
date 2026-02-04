#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_image_editor()
        .create(magic_hour::resources::v1::ai_image_editor::CreateRequest {
            aspect_ratio: Some(
                magic_hour::models::V1AiImageEditorCreateBodyAspectRatioEnum::Enum11,
            ),
            assets: magic_hour::models::V1AiImageEditorCreateBodyAssets {
                image_file_path: Some("string".to_string()),
                image_file_paths: Some(
                    vec![
                        "api-assets/id/1234.png".to_string(), "api-assets/id/1235.png"
                        .to_string()
                    ],
                ),
            },
            image_count: Some(1.0),
            model: Some(magic_hour::models::V1AiImageEditorCreateBodyModelEnum::Default),
            name: Some("My Ai Image Editor image".to_string()),
            style: magic_hour::models::V1AiImageEditorCreateBodyStyle {
                model: Some(
                    magic_hour::models::V1AiImageEditorCreateBodyStyleModelEnum::NanoBanana,
                ),
                prompt: "Give me sunglasses".to_string(),
            },
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
