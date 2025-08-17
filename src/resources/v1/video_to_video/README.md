
### Video-to-Video <a name="create"></a>

Create a Video To Video video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
  
Get more information about this mode at our [product page](https://magichour.ai/products/video-to-video).
  

**API Endpoint**: `POST /v1/video-to-video`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for video-to-video. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used | `V1VideoToVideoCreateBodyAssets {video_file_path: Some("api-assets/id/1234.mp4".to_string()), video_source: V1VideoToVideoCreateBodyAssetsVideoSourceEnum::File, ..Default::default()}` |
| `end_seconds` | ✓ | The end time of the input video in seconds. This value is used to trim the input video. The value must be greater than 0.1, and more than the start_seconds. | `15.0` |
| `start_seconds` | ✓ | The start time of the input video in seconds. This value is used to trim the input video. The value must be greater than 0. | `0.0` |
| `style` | ✓ |  | `V1VideoToVideoCreateBodyStyle {art_style: V1VideoToVideoCreateBodyStyleArtStyleEnum::Enum3dRender, model: V1VideoToVideoCreateBodyStyleModelEnum::Default, prompt: Some("string".to_string()), prompt_type: V1VideoToVideoCreateBodyStylePromptTypeEnum::Default, version: V1VideoToVideoCreateBodyStyleVersionEnum::Default}` |
| `fps_resolution` | ✗ | Determines whether the resulting video will have the same frame per second as the original video, or half.  * `FULL` - the result video will have the same FPS as the input video * `HALF` - the result video will have half the FPS as the input video | `V1VideoToVideoCreateBodyFpsResolutionEnum::Half` |
| `height` | ✗ | `height` is deprecated and no longer influences the output video's resolution.  Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details.  This field is retained only for backward compatibility and will be removed in a future release. | `123` |
| `name` | ✗ | The name of video. This value is mainly used for your own identification of the video. | `"Video To Video video".to_string()` |
| `width` | ✗ | `width` is deprecated and no longer influences the output video's resolution.  Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details.  This field is retained only for backward compatibility and will be removed in a future release. | `123` |

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
        name: Some("Video To Video video".to_string()),
        start_seconds: 0.0,
        style: magic_hour::models::V1VideoToVideoCreateBodyStyle {
            art_style: magic_hour::models::V1VideoToVideoCreateBodyStyleArtStyleEnum::Enum3dRender,
            model: magic_hour::models::V1VideoToVideoCreateBodyStyleModelEnum::Default,
            prompt: Some("string".to_string()),
            prompt_type: magic_hour::models::V1VideoToVideoCreateBodyStylePromptTypeEnum::Default,
            version: magic_hour::models::V1VideoToVideoCreateBodyStyleVersionEnum::Default,
        },
        ..Default::default()
    })
    .await;
```

#### Response

##### Type
[V1VideoToVideoCreateResponse](/src/models/v1_video_to_video_create_response.rs)

##### Example
`V1VideoToVideoCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}`
