#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_all_params() {
    let mut client = magic_hour::Client::default()
        .with_bearer_auth("API_TOKEN")
        .with_environment(magic_hour::Environment::MockServer);
    let res = client
        .v1()
        .face_swap()
        .create(magic_hour::resources::v1::face_swap::CreateRequest {
            assets: magic_hour::models::V1FaceSwapCreateBodyAssets {
                face_mappings: Some(
                    vec![
                        magic_hour::models::V1FaceSwapCreateBodyAssetsFaceMappingsItem {
                        new_face : "api-assets/id/1234.png".to_string(), original_face :
                        "api-assets/id/0-0.png".to_string() }
                    ],
                ),
                face_swap_mode: Some(
                    magic_hour::models::V1FaceSwapCreateBodyAssetsFaceSwapModeEnum::AllFaces,
                ),
                image_file_path: Some("image/id/1234.png".to_string()),
                video_file_path: Some("api-assets/id/1234.mp4".to_string()),
                video_source: magic_hour::models::V1FaceSwapCreateBodyAssetsVideoSourceEnum::File,
                youtube_url: Some("http://www.example.com".to_string()),
            },
            end_seconds: 15.0,
            height: Some(960),
            name: Some("Face Swap video".to_string()),
            start_seconds: 0.0,
            width: Some(512),
        })
        .await;
    println!("{res:?}");
    assert!(res.is_ok());
}
