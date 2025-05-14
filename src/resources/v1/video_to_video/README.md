
### Video-to-Video <a name="create"></a>

Create a Video To Video video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
  
Get more information about this mode at our [product page](/products/video-to-video).
  

**API Endpoint**: `POST /v1/video-to-video`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .video_to_video()
    .create(magic_hour::resources::v1::video_to_video::CreateRequest {
        assets: magic_hour::models::V1VideoToVideoCreateBodyAssets {
            video_file_path: Some("api-assets/id/1234.mp4".to_string()),
            video_source: magic_hour::models::V1VideoToVideoCreateBodyAssetsVideoSourceEnum::File,
            ..Default::default()
        },
        end_seconds: 15.0,
        fps_resolution: Some(
            magic_hour::models::V1VideoToVideoCreateBodyFpsResolutionEnum::Half,
        ),
        height: Some(960),
        name: Some("Video To Video video".to_string()),
        start_seconds: 0.0,
        style: magic_hour::models::V1VideoToVideoCreateBodyStyle {
            art_style: magic_hour::models::V1VideoToVideoCreateBodyStyleArtStyleEnum::Enum3dRender,
            model: magic_hour::models::V1VideoToVideoCreateBodyStyleModelEnum::AbsoluteReality,
            prompt: Some("string".to_string()),
            prompt_type: magic_hour::models::V1VideoToVideoCreateBodyStylePromptTypeEnum::AppendDefault,
            version: magic_hour::models::V1VideoToVideoCreateBodyStyleVersionEnum::Default,
        },
        width: Some(512),
    })
    .await;
```
