#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .ai_face_editor()
        .create(magic_hour::resources::v1::ai_face_editor::CreateRequest {
            assets: magic_hour::models::V1AiFaceEditorCreateBodyAssets {
                image_file_path: "api-assets/id/1234.png".to_string(),
            },
            name: Some("Face Editor image".to_string()),
            style: magic_hour::models::V1AiFaceEditorCreateBodyStyle {
                enhance_face: false,
                eye_gaze_horizontal: 0.0,
                eye_gaze_vertical: 0.0,
                eye_open_ratio: 0.0,
                eyebrow_direction: 0.0,
                head_pitch: 0.0,
                head_roll: 0.0,
                head_yaw: 0.0,
                lip_open_ratio: 0.0,
                mouth_grim: 0.0,
                mouth_position_horizontal: 0.0,
                mouth_position_vertical: 0.0,
                mouth_pout: 0.0,
                mouth_purse: 0.0,
                mouth_smile: 0.0,
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
