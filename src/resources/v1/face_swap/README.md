
### create <a name="create"></a>
Create Face Swap video

Create a Face Swap video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
  
Get more information about this mode at our [product page](/products/face-swap).
  

**API Endpoint**: `POST /v1/face-swap`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .face_swap()
    .create(magic_hour::resources::v1::face_swap::CreateRequest {
        data: magic_hour::models::PostV1FaceSwapBody {
            assets: magic_hour::models::PostV1FaceSwapBodyAssets {
                image_file_path: "image/id/1234.png".to_string(),
                video_file_path: Some("video/id/1234.mp4".to_string()),
                video_source: magic_hour::models::PostV1FaceSwapBodyAssetsVideoSourceEnum::File,
                youtube_url: Some("http://www.example.com".to_string()),
            },
            end_seconds: 15,
            height: 960,
            name: Some("Face Swap video".to_string()),
            start_seconds: 0,
            width: 512,
        },
    })
    .await;
```

**Upgrade to see all examples**