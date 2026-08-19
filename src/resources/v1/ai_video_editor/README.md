# v1.ai_video_editor

## Module Functions

### AI Video Editor <a name="create"></a>

**What this API does**

Create the same Video Editor you can make in the browser, but programmatically, so you can automate it, run it at scale, or connect it to your own app or workflow.

**Good for**

- Automation and batch processing
- Adding video editor into apps, pipelines, or tools

**How it works (3 steps)**

1. Upload your inputs (video, image, or audio) with [Generate Upload URLs](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls) and copy the `file_path`.
2. Send a request to create a video editor job with the basic fields.
3. Check the job status until it's `complete`, then download the result from `downloads`.

**Key options**

- Inputs: usually a file, sometimes a YouTube link, depending on project type
- Resolution: free users are limited to 576px; higher plans unlock HD and larger sizes
- Extra fields: e.g. `face_swap_mode`, `start_seconds`/`end_seconds`, or a text prompt

**Cost**\
Credits are only charged for the frames that actually render. You'll see an estimate when the job is queued, and the final total after it's done.

For detailed examples, see the [product page](https://magichour.ai/products/ai-video-editor).

**API Endpoint**: `POST /v1/ai-video-editor`

#### Parameters

| Parameter            | Required | Description                                                                                                                                                                                                                                                                                                                                      | Example                                                                                   |
| -------------------- | :------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| `assets`             |    ✓     | Provide the assets for video editing.                                                                                                                                                                                                                                                                                                            | `V1AiVideoEditorCreateBodyAssets {video_file_path: "api-assets/id/1234.mp4".to_string()}` |
| `└─ video_file_path` |    ✓     | The video to edit. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details. | `"api-assets/id/1234.mp4".to_string()`                                                    |
| `end_seconds`        |    ✓     | End time of your clip in seconds. Must be greater than `start_seconds`. Minimum duration depends on model: `gemini-omni`: 3s, `ltx-2.3`: 0.5s. Maximum duration depends on model: `gemini-omni`: 10s, `ltx-2.3`: 45s.                                                                                                                            | `5.0`                                                                                     |
| `style`              |    ✓     |                                                                                                                                                                                                                                                                                                                                                  | `V1AiVideoEditorCreateBodyStyle {prompt: "Change the car color to blue".to_string()}`     |
| `└─ prompt`          |    ✓     | The prompt used to edit the video.                                                                                                                                                                                                                                                                                                               | `"Change the car color to blue".to_string()`                                              |
| `model`              |    ✗     | Editing model. Defaults to `ltx-2.3` for free tier and `gemini-omni` for paid. Use `ltx-2.3` for LTX video edit.                                                                                                                                                                                                                                 | `V1AiVideoEditorCreateBodyModelEnum::GeminiOmni`                                          |
| `name`               |    ✗     | Give your video a custom name for easy identification.                                                                                                                                                                                                                                                                                           | `"My Video Editor video".to_string()`                                                     |
| `resolution`         |    ✗     | Output resolution. Defaults to `480p` for free tier and `720p` for paid. Google Omni supports 720p only; LTX-2.3 supports 480p, 720p, and 1080p.                                                                                                                                                                                                 | `V1AiVideoEditorCreateBodyResolutionEnum::Enum720p`                                       |
| `start_seconds`      |    ✗     | Start time of your clip (seconds). Must be ≥ 0.                                                                                                                                                                                                                                                                                                  | `0.0`                                                                                     |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_video_editor()
    .create(magic_hour::resources::v1::ai_video_editor::CreateRequest {
        assets: magic_hour::models::V1AiVideoEditorCreateBodyAssets {
            video_file_path: "api-assets/id/1234.mp4".to_string(),
        },
        end_seconds: 5.0,
        model: Some(
            magic_hour::models::V1AiVideoEditorCreateBodyModelEnum::GeminiOmni,
        ),
        name: Some("My Video Editor video".to_string()),
        resolution: Some(
            magic_hour::models::V1AiVideoEditorCreateBodyResolutionEnum::Enum720p,
        ),
        start_seconds: Some(0.0),
        style: magic_hour::models::V1AiVideoEditorCreateBodyStyle {
            prompt: "Change the car color to blue".to_string(),
        },
    })
    .await;
```

#### Response

##### Type

[V1AiVideoEditorCreateResponse](/src/models/v1_ai_video_editor_create_response.rs)

##### Example

```rust
V1AiVideoEditorCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}
```
