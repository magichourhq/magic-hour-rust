
### create <a name="create"></a>
Create Text-to-Video

Create a Text To Video video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
  
Get more information about this mode at our [product page](/products/text-to-video).
  

**API Endpoint**: `POST /v1/text-to-video`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .text_to_video()
    .create(magic_hour::resources::v1::text_to_video::CreateRequest {
        end_seconds: 5,
        orientation: magic_hour::models::PostV1TextToVideoBodyOrientationEnum::Landscape,
        style: magic_hour::models::PostV1TextToVideoBodyStyle {
            prompt: "string".to_string(),
        },
        ..Default::default()
    })
    .await;
```

**Upgrade to see all examples**
