# v1.lip_sync

## Module Functions

### Lip Sync <a name="create"></a>

Create a Lip Sync video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
  
Get more information about this mode at our [product page](https://magichour.ai/products/lip-sync).
  

**API Endpoint**: `POST /v1/lip-sync`

#### Parameters

| Parameter | Required | Deprecated | Description | Example |
|-----------|:--------:|:----------:|-------------|--------|
| `assets` | ✓ | ✗ | Provide the assets for lip-sync. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used | `V1LipSyncCreateBodyAssets {audio_file_path: "api-assets/id/1234.mp3".to_string(), video_file_path: Some("api-assets/id/1234.mp4".to_string()), video_source: V1LipSyncCreateBodyAssetsVideoSourceEnum::File, ..Default::default()}` |
| `└─ audio_file_path` | ✓ | — | The path of the audio file. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.mp3".to_string()` |
| `└─ video_file_path` | ✗ | — | Required if `video_source` is `file`. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.mp4".to_string()` |
| `└─ video_source` | ✓ | — |  | `V1LipSyncCreateBodyAssetsVideoSourceEnum::File` |
| `└─ youtube_url` | ✗ | — | Using a youtube video as the input source. This field is required if `video_source` is `youtube` | `"http://www.example.com".to_string()` |
| `end_seconds` | ✓ | ✗ | The end time of the input video in seconds. This value is used to trim the input video. The value must be greater than 0.1, and more than the start_seconds. | `15.0` |
| `start_seconds` | ✓ | ✗ | The start time of the input video in seconds. This value is used to trim the input video. The value must be greater than 0. | `0.0` |
| `height` | ✗ | ✓ | `height` is deprecated and no longer influences the output video's resolution.  Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details.  This field is retained only for backward compatibility and will be removed in a future release. | `123` |
| `max_fps_limit` | ✗ | ✗ | Defines the maximum FPS (frames per second) for the output video. If the input video's FPS is lower than this limit, the output video will retain the input FPS. This is useful for reducing unnecessary frame usage in scenarios where high FPS is not required. | `12.0` |
| `name` | ✗ | ✗ | The name of video. This value is mainly used for your own identification of the video. | `"Lip Sync video".to_string()` |
| `style` | ✗ | ✗ | Attributes used to dictate the style of the output | `V1LipSyncCreateBodyStyle {generation_mode: Some(V1LipSyncCreateBodyStyleGenerationModeEnum::Lite)}` |
| `└─ generation_mode` | ✗ | — | A specific version of our lip sync system, optimized for different needs. * `lite` -  Fast and affordable lip sync - best for simple videos. Costs 1 credit per frame of video. * `standard` -  Natural, accurate lip sync - best for most creators. Costs 1 credit per frame of video. * `pro` -  Premium fidelity with enhanced detail - best for professionals. Costs 2 credits per frame of video.  Note: `standard` and `pro` are only available for users on Creator, Pro, and Business tiers.                | `V1LipSyncCreateBodyStyleGenerationModeEnum::Lite` |
| `width` | ✗ | ✓ | `width` is deprecated and no longer influences the output video's resolution.  Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details.  This field is retained only for backward compatibility and will be removed in a future release. | `123` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .lip_sync()
    .create(magic_hour::resources::v1::lip_sync::CreateRequest {
        assets: magic_hour::models::V1LipSyncCreateBodyAssets {
            audio_file_path: "api-assets/id/1234.mp3".to_string(),
            video_file_path: Some("api-assets/id/1234.mp4".to_string()),
            video_source: magic_hour::models::V1LipSyncCreateBodyAssetsVideoSourceEnum::File,
            ..Default::default()
        },
        end_seconds: 15.0,
        max_fps_limit: Some(12.0),
        name: Some("Lip Sync video".to_string()),
        start_seconds: 0.0,
        ..Default::default()
    })
    .await;
```

#### Response

##### Type
[V1LipSyncCreateResponse](/src/models/v1_lip_sync_create_response.rs)

##### Example
`V1LipSyncCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}`


