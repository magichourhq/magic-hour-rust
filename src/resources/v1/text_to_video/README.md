
### Text-to-Video <a name="create"></a>

Create a Text To Video video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
  
Get more information about this mode at our [product page](/products/text-to-video).
  

**API Endpoint**: `POST /v1/text-to-video`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `end_seconds` | ✓ | The total duration of the output video in seconds. | `5.0` |
| `orientation` | ✓ | Determines the orientation of the output video | `V1TextToVideoCreateBodyOrientationEnum::Landscape` |
| `style` | ✓ |  | `V1TextToVideoCreateBodyStyle {prompt: "a dog running".to_string(), ..Default::default()}` |
| `name` | ✗ | The name of video | `"Text To Video video".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .text_to_video()
    .create(magic_hour::resources::v1::text_to_video::CreateRequest {
        end_seconds: 5.0,
        name: Some("Text To Video video".to_string()),
        orientation: magic_hour::models::V1TextToVideoCreateBodyOrientationEnum::Landscape,
        style: magic_hour::models::V1TextToVideoCreateBodyStyle {
            prompt: "a dog running".to_string(),
            ..Default::default()
        },
    })
    .await;
```

#### Response

##### Type
[V1TextToVideoCreateResponse](/src/models/v1_text_to_video_create_response.rs)

##### Example
`V1TextToVideoCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "clx7uu86w0a5qp55yxz315r6r".to_string()}`
