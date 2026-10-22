# v1.ai_video_upscaler

## Module Functions

### AI Video Upscaler <a name="create"></a>

**What this API does**

Create the same Video Upscaler you can make in the browser, but programmatically, so you can automate it, run it at scale, or connect it to your own app or workflow.

**Good for**

- Automation and batch processing
- Adding video upscaler into apps, pipelines, or tools

**How it works (3 steps)**

1. Upload your inputs (video, image, or audio) with [Generate Upload URLs](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls) and copy the `file_path`.
2. Send a request to create a video upscaler job with the basic fields.
3. Check the job status until it's `complete`, then download the result from `downloads`.

**Key options**

- Inputs: usually a file, sometimes a YouTube link, depending on project type
- Resolution: output size and plan requirements vary by workflow and model. Some workflows derive output size from the input video.
- Extra fields: e.g. `face_swap_mode`, `start_seconds`/`end_seconds`, or a text prompt

**Cost**\
Credits are charged when the job is queued (`credits_charged` in the response). Tools billed per frame charge an estimate and settle to the frames that actually render once the job finishes; tools billed per second charge the final amount upfront. Failed jobs are refunded.

For detailed examples, see the [product page](https://magichour.ai/products/video-upscaler).

**API Endpoint**: `POST /v1/ai-video-upscaler`

#### Parameters

| Parameter            | Required | Description                                                                                                                                                                                                                                                                                                                                     | Example                                                                                     |
| -------------------- | :------: | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| `assets`             |    ✓     | Source video for the upscale job.                                                                                                                                                                                                                                                                                                               | `V1AiVideoUpscalerCreateBodyAssets {video_file_path: "api-assets/id/1234.mp4".to_string()}` |
| `└─ video_file_path` |    ✓     | Video to upscale. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details. | `"api-assets/id/1234.mp4".to_string()`                                                      |
| `end_seconds`        |    ✓     | End time of your clip (seconds). Must be greater than start_seconds.                                                                                                                                                                                                                                                                            | `5.0`                                                                                       |
| `resolution`         |    ✓     | Output resolution. `720p` and `1080p` set the output's short side, `2k` and `4k` its long side; the aspect ratio is kept. Requires a paid plan.                                                                                                                                                                                                 | `V1AiVideoUpscalerCreateBodyResolutionEnum::Enum1080p`                                      |
| `name`               |    ✗     | Give your video a custom name for easy identification.                                                                                                                                                                                                                                                                                          | `"My Video Upscaler video".to_string()`                                                     |
| `start_seconds`      |    ✗     | Start time of your clip (seconds). Must be ≥ 0.                                                                                                                                                                                                                                                                                                 | `0.0`                                                                                       |

#### Example Snippet

```rust
let mut client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_video_upscaler()
    .create(magic_hour::resources::v1::ai_video_upscaler::CreateRequest {
        assets: magic_hour::models::V1AiVideoUpscalerCreateBodyAssets {
            video_file_path: "api-assets/id/1234.mp4".to_string(),
        },
        end_seconds: 5.0,
        name: Some("My Video Upscaler video".to_string()),
        resolution: magic_hour::models::V1AiVideoUpscalerCreateBodyResolutionEnum::Enum1080p,
        start_seconds: Some(0.0),
    })
    .await;
```

#### Response

##### Type

[V1AiVideoUpscalerCreateResponse](/src/models/v1_ai_video_upscaler_create_response.rs)

##### Example

```rust
V1AiVideoUpscalerCreateResponse {credits_charged: 450, estimated_frame_cost: 123, id: "cuid-example".to_string()}
```
