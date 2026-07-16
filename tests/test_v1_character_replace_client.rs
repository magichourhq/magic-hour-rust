#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .character_replace()
        .create(magic_hour::resources::v1::character_replace::CreateRequest {
            assets: magic_hour::models::V1CharacterReplaceCreateBodyAssets {
                image_file_path: "api-assets/id/5678.png".to_string(),
                video_file_path: "api-assets/id/1234.mp4".to_string(),
            },
            end_seconds: 15.0,
            name: Some("My Character Replace video".to_string()),
            resolution: Some(
                magic_hour::models::V1CharacterReplaceCreateBodyResolutionEnum::Enum720p,
            ),
            start_seconds: Some(0.0),
            style: Some(magic_hour::models::V1CharacterReplaceCreateBodyStyle {
                mode: Some(
                    magic_hour::models::V1CharacterReplaceCreateBodyStyleModeEnum::Replace,
                ),
                points: Some(
                    vec![
                        magic_hour::models::V1CharacterReplaceCreateBodyStylePointsItem {
                        position_x : 320, position_y : 180, time_seconds : 2.5 }
                    ],
                ),
                selection_mode: Some(
                    magic_hour::models::V1CharacterReplaceCreateBodyStyleSelectionModeEnum::Auto,
                ),
            }),
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
