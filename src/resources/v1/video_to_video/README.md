
### Video-to-Video <a name="create"></a>

Create a Video To Video video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
  
Get more information about this mode at our [product page](https://magichour.ai/products/video-to-video).
  

**API Endpoint**: `POST /v1/video-to-video`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for video-to-video. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used | `V1VideoToVideoCreateBodyAssets {video_file_path: Some("api-assets/id/1234.mp4".to_string()), video_source: V1VideoToVideoCreateBodyAssetsVideoSourceEnum::File, ..Default::default()}` |
| `end_seconds` | ✓ | The end time of the input video in seconds | `15.0` |
| `start_seconds` | ✓ | The start time of the input video in seconds | `0.0` |
| `style` | ✓ |  | `V1VideoToVideoCreateBodyStyle {art_style: V1VideoToVideoCreateBodyStyleArtStyleEnum::Enum3dRender, model: V1VideoToVideoCreateBodyStyleModelEnum::AbsoluteReality, prompt: Some("string".to_string()), prompt_type: V1VideoToVideoCreateBodyStylePromptTypeEnum::AppendDefault, version: V1VideoToVideoCreateBodyStyleVersionEnum::Default}` |
| `fps_resolution` | ✗ | Determines whether the resulting video will have the same frame per second as the original video, or half.  * `FULL` - the result video will have the same FPS as the input video * `HALF` - the result video will have half the FPS as the input video | `V1VideoToVideoCreateBodyFpsResolutionEnum::Half` |
| `height` | ✗ | Used to determine the dimensions of the output video.     * If height is provided, width will also be required. The larger value between width and height will be used to determine the maximum output resolution while maintaining the original aspect ratio. * If both height and width are omitted, the video will be resized according to your subscription's maximum resolution, while preserving aspect ratio.  Note: if the video's original resolution is less than the maximum, the video will not be resized.  See our [pricing page](https://magichour.ai/pricing) for more details. | `960` |
| `name` | ✗ | The name of video | `"Video To Video video".to_string()` |
| `width` | ✗ | Used to determine the dimensions of the output video.     * If width is provided, height will also be required. The larger value between width and height will be used to determine the maximum output resolution while maintaining the original aspect ratio. * If both height and width are omitted, the video will be resized according to your subscription's maximum resolution, while preserving aspect ratio.  Note: if the video's original resolution is less than the maximum, the video will not be resized.  See our [pricing page](https://magichour.ai/pricing) for more details. | `512` |

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

#### Response

##### Type
[V1VideoToVideoCreateResponse](/src/models/v1_video_to_video_create_response.rs)

##### Example
`V1VideoToVideoCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "clx7uu86w0a5qp55yxz315r6r".to_string()}`
