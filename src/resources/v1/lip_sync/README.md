
### create <a name="create"></a>
Create Lip Sync video

Create a Lip Sync video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
  
Get more information about this mode at our [product page](/products/lip-sync).
  

**API Endpoint**: `POST /v1/lip-sync`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .lip_sync()
    .create(magic_hour::resources::v1::lip_sync::CreateRequest {
        assets: magic_hour::models::PostV1LipSyncBodyAssets {
            audio_file_path: "audio/id/1234.mp3".to_string(),
            video_file_path: Some("video/id/1234.mp4".to_string()),
            video_source: magic_hour::models::PostV1LipSyncBodyAssetsVideoSourceEnum::File,
            ..Default::default()
        },
        end_seconds: 15,
        height: 960,
        start_seconds: 0,
        width: 512,
        ..Default::default()
    })
    .await;
```
