
### create <a name="create"></a>
Create Video-to-Video

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
        assets: magic_hour::models::PostV1VideoToVideoBodyAssets {
            video_file_path: Some("video/id/1234.mp4".to_string()),
            video_source: magic_hour::models::PostV1VideoToVideoBodyAssetsVideoSourceEnum::File,
            ..Default::default()
        },
        end_seconds: 15,
        height: 960,
        start_seconds: 0,
        style: magic_hour::models::PostV1VideoToVideoBodyStyle {
            art_style: magic_hour::models::PostV1VideoToVideoBodyStyleArtStyleEnum::_3dRender,
            model: magic_hour::models::PostV1VideoToVideoBodyStyleModelEnum::AbsoluteReality,
            prompt_type: magic_hour::models::PostV1VideoToVideoBodyStylePromptTypeEnum::AppendDefault,
            version: magic_hour::models::PostV1VideoToVideoBodyStyleVersionEnum::Default,
            ..Default::default()
        },
        width: 512,
        ..Default::default()
    })
    .await;
```
