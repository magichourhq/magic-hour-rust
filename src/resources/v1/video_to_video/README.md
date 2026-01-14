# v1.video_to_video

## Module Functions

### Video-to-Video <a name="create"></a>

**What this API does**

Create the same Video To Video you can make in the browser, but programmatically, so you can automate it, run it at scale, or connect it to your own app or workflow.

**Good for**

- Automation and batch processing
- Adding video to video into apps, pipelines, or tools

**How it works (3 steps)**

1. Upload your inputs (video, image, or audio) with [Generate Upload URLs](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls) and copy the `file_path`.
2. Send a request to create a video to video job with the basic fields.
3. Check the job status until it's `complete`, then download the result from `downloads`.

**Key options**

- Inputs: usually a file, sometimes a YouTube link, depending on project type
- Resolution: free users are limited to 576px; higher plans unlock HD and larger sizes
- Extra fields: e.g. `face_swap_mode`, `start_seconds`/`end_seconds`, or a text prompt

**Cost**\
Credits are only charged for the frames that actually render. You'll see an estimate when the job is queued, and the final total after it's done.

For detailed examples, see the [product page](https://magichour.ai/products/video-to-video).

**API Endpoint**: `POST /v1/video-to-video`

#### Parameters

| Parameter            | Required | Deprecated | Description                                                                                                                                                                                                                                                                                                                                                                                                  | Example                                                                                                                                                                                                                                                                                                                             |
| -------------------- | :------: | :--------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `assets`             |    ✓     |     ✗      | Provide the assets for video-to-video. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used                                                                                                                                                                                                                                                               | `V1VideoToVideoCreateBodyAssets {video_file_path: Some("api-assets/id/1234.mp4".to_string()), video_source: V1VideoToVideoCreateBodyAssetsVideoSourceEnum::File, ..Default::default()}`                                                                                                                                             |
| `└─ video_file_path` |    ✗     |     —      | Your video file. Required if `video_source` is `file`. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.                         | `"api-assets/id/1234.mp4".to_string()`                                                                                                                                                                                                                                                                                              |
| `└─ video_source`    |    ✓     |     —      | Choose your video source.                                                                                                                                                                                                                                                                                                                                                                                    | `V1VideoToVideoCreateBodyAssetsVideoSourceEnum::File`                                                                                                                                                                                                                                                                               |
| `└─ youtube_url`     |    ✗     |     —      | YouTube URL (required if `video_source` is `youtube`).                                                                                                                                                                                                                                                                                                                                                       | `"http://www.example.com".to_string()`                                                                                                                                                                                                                                                                                              |
| `end_seconds`        |    ✓     |     ✗      | End time of your clip (seconds). Must be greater than start_seconds.                                                                                                                                                                                                                                                                                                                                         | `15.0`                                                                                                                                                                                                                                                                                                                              |
| `start_seconds`      |    ✓     |     ✗      | Start time of your clip (seconds). Must be ≥ 0.                                                                                                                                                                                                                                                                                                                                                              | `0.0`                                                                                                                                                                                                                                                                                                                               |
| `style`              |    ✓     |     ✗      |                                                                                                                                                                                                                                                                                                                                                                                                              | `V1VideoToVideoCreateBodyStyle {art_style: V1VideoToVideoCreateBodyStyleArtStyleEnum::Enum3dRender, model: Some(V1VideoToVideoCreateBodyStyleModelEnum::Default), prompt_type: Some(V1VideoToVideoCreateBodyStylePromptTypeEnum::Default), version: Some(V1VideoToVideoCreateBodyStyleVersionEnum::Default), ..Default::default()}` |
| `└─ art_style`       |    ✓     |     —      |                                                                                                                                                                                                                                                                                                                                                                                                              | `V1VideoToVideoCreateBodyStyleArtStyleEnum::Enum3dRender`                                                                                                                                                                                                                                                                           |
| `└─ model`           |    ✗     |     —      | * `Dreamshaper` - a good all-around model that works for both animations as well as realism. * `Absolute Reality` - better at realism, but you'll often get similar results with Dreamshaper as well. * `Flat 2D Anime` - best for a flat illustration style that's common in most anime. * `default` - use the default recommended model for the selected art style.                                        | `V1VideoToVideoCreateBodyStyleModelEnum::Default`                                                                                                                                                                                                                                                                                   |
| `└─ prompt`          |    ✗     |     —      | The prompt used for the video. Prompt is required if `prompt_type` is `custom` or `append_default`. If `prompt_type` is `default`, then the `prompt` value passed will be ignored.                                                                                                                                                                                                                           | `"string".to_string()`                                                                                                                                                                                                                                                                                                              |
| `└─ prompt_type`     |    ✗     |     —      | * `default` - Use the default recommended prompt for the art style. * `custom` - Only use the prompt passed in the API. Note: for v1, lora prompt will still be auto added to apply the art style properly. * `append_default` - Add the default recommended prompt to the end of the prompt passed in the API.                                                                                              | `V1VideoToVideoCreateBodyStylePromptTypeEnum::Default`                                                                                                                                                                                                                                                                              |
| `└─ version`         |    ✗     |     —      | * `v1` - more detail, closer prompt adherence, and frame-by-frame previews. * `v2` - faster, more consistent, and less noisy. * `default` - use the default version for the selected art style.                                                                                                                                                                                                              | `V1VideoToVideoCreateBodyStyleVersionEnum::Default`                                                                                                                                                                                                                                                                                 |
| `fps_resolution`     |    ✗     |     ✗      | Determines whether the resulting video will have the same frame per second as the original video, or half. * `FULL` - the result video will have the same FPS as the input video * `HALF` - the result video will have half the FPS as the input video                                                                                                                                                       | `V1VideoToVideoCreateBodyFpsResolutionEnum::Half`                                                                                                                                                                                                                                                                                   |
| `height`             |    ✗     |     ✓      | `height` is deprecated and no longer influences the output video's resolution. Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details. This field is retained only for backward compatibility and will be removed in a future release. | `123`                                                                                                                                                                                                                                                                                                                               |
| `name`               |    ✗     |     ✗      | Give your video a custom name for easy identification.                                                                                                                                                                                                                                                                                                                                                       | `"My Video To Video video".to_string()`                                                                                                                                                                                                                                                                                             |
| `width`              |    ✗     |     ✓      | `width` is deprecated and no longer influences the output video's resolution. Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details. This field is retained only for backward compatibility and will be removed in a future release.  | `123`                                                                                                                                                                                                                                                                                                                               |

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
        name: Some("My Video To Video video".to_string()),
        start_seconds: 0.0,
        style: magic_hour::models::V1VideoToVideoCreateBodyStyle {
            art_style: magic_hour::models::V1VideoToVideoCreateBodyStyleArtStyleEnum::Enum3dRender,
            model: Some(
                magic_hour::models::V1VideoToVideoCreateBodyStyleModelEnum::Default,
            ),
            prompt_type: Some(
                magic_hour::models::V1VideoToVideoCreateBodyStylePromptTypeEnum::Default,
            ),
            version: Some(
                magic_hour::models::V1VideoToVideoCreateBodyStyleVersionEnum::Default,
            ),
            ..Default::default()
        },
        ..Default::default()
    })
    .await;
```

#### Response

##### Type

[V1VideoToVideoCreateResponse](/src/models/v1_video_to_video_create_response.rs)

##### Example

```rust
V1VideoToVideoCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}
```
