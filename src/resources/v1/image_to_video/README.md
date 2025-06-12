
### Image-to-Video <a name="create"></a>

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
        assets: magic_hour::models::V1ImageToVideoCreateBodyAssets {
            image_file_path: "api-assets/id/1234.png".to_string(),
        },
        end_seconds: 5.0,
        height: Some(960),
        name: Some("Image To Video video".to_string()),
        style: magic_hour::models::V1ImageToVideoCreateBodyStyle {
            prompt: Some("a dog running".to_string()),
            ..Default::default()
        },
        width: Some(512),
    })
    .await;
```

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for image-to-video. | `V1ImageToVideoCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `end_seconds` | ✓ | The total duration of the output video in seconds. | `5.0` |
| `style` | ✓ | Attributed used to dictate the style of the output | `V1ImageToVideoCreateBodyStyle {prompt: Some("a dog running".to_string()), ..Default::default()}` |
| `height` | ✗ | This field does not affect the output video's resolution. The video's orientation will match that of the input image.  It is retained solely for backward compatibility and will be deprecated in the future. | `960` |
| `name` | ✗ | The name of video | `"Image To Video video".to_string()` |
| `width` | ✗ | This field does not affect the output video's resolution. The video's orientation will match that of the input image.  It is retained solely for backward compatibility and will be deprecated in the future. | `512` |
