# v1.lip_sync

## Module Functions

### Lip Sync <a name="create"></a>

**What this API does**

Create the same Lip Sync you can make in the browser, but programmatically, so you can automate it, run it at scale, or connect it to your own app or workflow.

**Good for**

- Automation and batch processing
- Adding lip sync into apps, pipelines, or tools

**How it works (3 steps)**

1. Upload your inputs (video, image, or audio) with [Generate Upload URLs](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls) and copy the `file_path`.
2. Send a request to create a lip sync job with the basic fields.
3. Check the job status until it's `complete`, then download the result from `downloads`.

**Key options**

- Inputs: usually a file, sometimes a YouTube link, depending on project type
- Resolution: free users are limited to 576px; higher plans unlock HD and larger sizes
- Extra fields: e.g. `face_swap_mode`, `start_seconds`/`end_seconds`, or a text prompt

**Cost**\
Credits are only charged for the frames that actually render. You'll see an estimate when the job is queued, and the final total after it's done.

For detailed examples, see the [product page](https://magichour.ai/products/lip-sync).

**API Endpoint**: `POST /v1/lip-sync`

#### Parameters

| Parameter            | Required | Deprecated | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Example                                                                                                                                                                                                                              |
| -------------------- | :------: | :--------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `assets`             |    ✓     |     ✗      | Provide the assets for lip-sync. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used                                                                                                                                                                                                                                                                                                                                                         | `V1LipSyncCreateBodyAssets {audio_file_path: "api-assets/id/1234.mp3".to_string(), video_file_path: Some("api-assets/id/1234.mp4".to_string()), video_source: V1LipSyncCreateBodyAssetsVideoSourceEnum::File, ..Default::default()}` |
| `└─ audio_file_path` |    ✓     |     —      | The path of the audio file. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.                                                                                                                                        | `"api-assets/id/1234.mp3".to_string()`                                                                                                                                                                                               |
| `└─ video_file_path` |    ✗     |     —      | Your video file. Required if `video_source` is `file`. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.                                                                                                             | `"api-assets/id/1234.mp4".to_string()`                                                                                                                                                                                               |
| `└─ video_source`    |    ✓     |     —      | Choose your video source.                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `V1LipSyncCreateBodyAssetsVideoSourceEnum::File`                                                                                                                                                                                     |
| `└─ youtube_url`     |    ✗     |     —      | YouTube URL (required if `video_source` is `youtube`).                                                                                                                                                                                                                                                                                                                                                                                                                                           | `"http://www.example.com".to_string()`                                                                                                                                                                                               |
| `end_seconds`        |    ✓     |     ✗      | End time of your clip (seconds). Must be greater than start_seconds.                                                                                                                                                                                                                                                                                                                                                                                                                             | `15.0`                                                                                                                                                                                                                               |
| `start_seconds`      |    ✓     |     ✗      | Start time of your clip (seconds). Must be ≥ 0.                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `0.0`                                                                                                                                                                                                                                |
| `height`             |    ✗     |     ✓      | `height` is deprecated and no longer influences the output video's resolution. Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details. This field is retained only for backward compatibility and will be removed in a future release.                                                                                     | `123`                                                                                                                                                                                                                                |
| `max_fps_limit`      |    ✗     |     ✗      | Defines the maximum FPS (frames per second) for the output video. If the input video's FPS is lower than this limit, the output video will retain the input FPS. This is useful for reducing unnecessary frame usage in scenarios where high FPS is not required.                                                                                                                                                                                                                                | `12.0`                                                                                                                                                                                                                               |
| `name`               |    ✗     |     ✗      | Give your video a custom name for easy identification.                                                                                                                                                                                                                                                                                                                                                                                                                                           | `"My Lip Sync video".to_string()`                                                                                                                                                                                                    |
| `style`              |    ✗     |     ✗      | Attributes used to dictate the style of the output                                                                                                                                                                                                                                                                                                                                                                                                                                               | `V1LipSyncCreateBodyStyle {generation_mode: Some(V1LipSyncCreateBodyStyleGenerationModeEnum::Lite)}`                                                                                                                                 |
| `└─ generation_mode` |    ✗     |     —      | A specific version of our lip sync system, optimized for different needs. * `lite` - Fast and affordable lip sync - best for simple videos. Costs 1 credit per frame of video. * `standard` - Natural, accurate lip sync - best for most creators. Costs 1 credit per frame of video. * `pro` - Premium fidelity with enhanced detail - best for professionals. Costs 2 credits per frame of video. Note: `standard` and `pro` are only available for users on Creator, Pro, and Business tiers. | `V1LipSyncCreateBodyStyleGenerationModeEnum::Lite`                                                                                                                                                                                   |
| `width`              |    ✗     |     ✓      | `width` is deprecated and no longer influences the output video's resolution. Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details. This field is retained only for backward compatibility and will be removed in a future release.                                                                                      | `123`                                                                                                                                                                                                                                |

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
        name: Some("My Lip Sync video".to_string()),
        start_seconds: 0.0,
        ..Default::default()
    })
    .await;
```

#### Response

##### Type

[V1LipSyncCreateResponse](/src/models/v1_lip_sync_create_response.rs)

##### Example

```rust
V1LipSyncCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}
```
