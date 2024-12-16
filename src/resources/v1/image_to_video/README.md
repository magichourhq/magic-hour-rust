
### create <a name="create"></a>
Create Image-to-Video

Create a Image To Video video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
  
Get more information about this mode at our [product page](/products/image-to-video).
  

**API Endpoint**: `POST /v1/image-to-video`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .image_to_video()
    .create(magic_hour::resources::v1::image_to_video::CreateRequest {
        assets: magic_hour::models::PostV1ImageToVideoBodyAssets {
            image_file_path: "image/id/1234.png".to_string(),
        },
        end_seconds: 5,
        height: 960,
        style: magic_hour::models::PostV1ImageToVideoBodyStyle {
            ..Default::default()
        },
        width: 512,
        ..Default::default()
    })
    .await;
```

**Upgrade to see all examples**
