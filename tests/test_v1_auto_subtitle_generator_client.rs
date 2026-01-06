#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .auto_subtitle_generator()
        .create(magic_hour::resources::v1::auto_subtitle_generator::CreateRequest {
            assets: magic_hour::models::V1AutoSubtitleGeneratorCreateBodyAssets {
                video_file_path: "api-assets/id/1234.mp4".to_string(),
            },
            end_seconds: 15.0,
            name: Some("My Auto Subtitle video".to_string()),
            start_seconds: 0.0,
            style: magic_hour::models::V1AutoSubtitleGeneratorCreateBodyStyle {
                custom_config: Some(magic_hour::models::V1AutoSubtitleGeneratorCreateBodyStyleCustomConfig {
                    font: Some("Noto Sans".to_string()),
                    font_size: Some(24.0),
                    font_style: Some("normal".to_string()),
                    highlighted_text_color: Some("#FFD700".to_string()),
                    horizontal_position: Some("center".to_string()),
                    stroke_color: Some("#000000".to_string()),
                    stroke_width: Some(1.0),
                    text_color: Some("#FFFFFF".to_string()),
                    vertical_position: Some("bottom".to_string()),
                }),
                template: Some(
                    magic_hour::models::V1AutoSubtitleGeneratorCreateBodyStyleTemplateEnum::Cinematic,
                ),
            },
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
