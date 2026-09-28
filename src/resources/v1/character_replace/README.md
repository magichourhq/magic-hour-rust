# v1.character_replace

## Module Functions

### Character Replace <a name="create"></a>

**What this API does**

Create the same Character Replace you can make in the browser, but programmatically, so you can automate it, run it at scale, or connect it to your own app or workflow.

**Good for**

- Automation and batch processing
- Adding character replace into apps, pipelines, or tools

**How it works (3 steps)**

1. Upload your inputs (video, image, or audio) with [Generate Upload URLs](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls) and copy the `file_path`.
2. Send a request to create a character replace job with the basic fields.
3. Check the job status until it's `complete`, then download the result from `downloads`.

**Key options**

- Inputs: usually a file, sometimes a YouTube link, depending on project type
- Resolution: free users are limited to 576px; higher plans unlock HD and larger sizes
- Extra fields: e.g. `face_swap_mode`, `start_seconds`/`end_seconds`, or a text prompt

**Cost**\
Credits are only charged for the frames that actually render. You'll see an estimate when the job is queued, and the final total after it's done.

For detailed examples, see the [product page](https://magichour.ai/products/character-replace).

**API Endpoint**: `POST /v1/character-replace`

#### Parameters

| Parameter            | Required | Description                                                                                                                                                                                                                                                                                                                                                                                          | Example                                                                                                                                                                                                    |
| -------------------- | :------: | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `assets`             |    ✓     | Source video and reference character image for the job.                                                                                                                                                                                                                                                                                                                                              | `V1CharacterReplaceCreateBodyAssets {image_file_path: "api-assets/id/5678.png".to_string(), video_file_path: "api-assets/id/1234.mp4".to_string()}`                                                        |
| `└─ image_file_path` |    ✓     | Reference character image used as the replacement or animation target. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details. | `"api-assets/id/5678.png".to_string()`                                                                                                                                                                     |
| `└─ video_file_path` |    ✓     | Source video containing the subject to replace or animate. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.             | `"api-assets/id/1234.mp4".to_string()`                                                                                                                                                                     |
| `end_seconds`        |    ✓     | End time of your clip (seconds). Must be greater than start_seconds.                                                                                                                                                                                                                                                                                                                                 | `15.0`                                                                                                                                                                                                     |
| `model`              |    ✗     | Model to use. Defaults to `wan-animate`. * **`wan-animate`**: 480p, 720p. Supports `points` subject selection. * **`kling-3.0`**: 720p, 1080p. Clips of 3–10 seconds in `replace` mode or 3–30 seconds in `animate` mode. Picks the main person automatically, so `points` are rejected.                                                                                                             | `V1CharacterReplaceCreateBodyModelEnum::WanAnimate`                                                                                                                                                        |
| `name`               |    ✗     | Give your video a custom name for easy identification.                                                                                                                                                                                                                                                                                                                                               | `"My Character Replace video".to_string()`                                                                                                                                                                 |
| `resolution`         |    ✗     | Output video resolution. Must be supported by `model`. Defaults to the lowest resolution available on your plan for that model.                                                                                                                                                                                                                                                                      | `V1CharacterReplaceCreateBodyResolutionEnum::Enum720p`                                                                                                                                                     |
| `start_seconds`      |    ✗     | Start time of your clip (seconds). Must be ≥ 0.                                                                                                                                                                                                                                                                                                                                                      | `0.0`                                                                                                                                                                                                      |
| `style`              |    ✗     | Optional style controls for replace vs animate mode and subject selection.                                                                                                                                                                                                                                                                                                                           | `V1CharacterReplaceCreateBodyStyle {mode: Some(V1CharacterReplaceCreateBodyStyleModeEnum::Replace), selection_mode: Some(V1CharacterReplaceCreateBodyStyleSelectionModeEnum::Auto), ..Default::default()}` |
| `└─ mode`            |    ✗     | Processing mode. `replace` swaps the detected subject with your reference character. `animate` transfers motion from the video onto your character image.                                                                                                                                                                                                                                            | `V1CharacterReplaceCreateBodyStyleModeEnum::Replace`                                                                                                                                                       |
| `└─ points`          |    ✗     | On-frame markers for manual subject selection. Required when `selection_mode` is `point`. Ignored when `selection_mode` is `auto` or omitted. Rejected for models without subject selection (supported by `wan-animate`).                                                                                                                                                                            | `vec![V1CharacterReplaceCreateBodyStylePointsItem {position_x: 320, position_y: 180, time_seconds: 2.5}]`                                                                                                  |
| `└─ selection_mode`  |    ✗     | How to locate the subject in the source video. `auto` detects a person automatically. `point` uses your `points` to mark the subject and is supported by `wan-animate`. Defaults to `auto`.                                                                                                                                                                                                          | `V1CharacterReplaceCreateBodyStyleSelectionModeEnum::Auto`                                                                                                                                                 |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .character_replace()
    .create(magic_hour::resources::v1::character_replace::CreateRequest {
        assets: magic_hour::models::V1CharacterReplaceCreateBodyAssets {
            image_file_path: "api-assets/id/5678.png".to_string(),
            video_file_path: "api-assets/id/1234.mp4".to_string(),
        },
        end_seconds: 15.0,
        model: Some(
            magic_hour::models::V1CharacterReplaceCreateBodyModelEnum::WanAnimate,
        ),
        name: Some("My Character Replace video".to_string()),
        resolution: Some(
            magic_hour::models::V1CharacterReplaceCreateBodyResolutionEnum::Enum720p,
        ),
        start_seconds: Some(0.0),
        style: Some(magic_hour::models::V1CharacterReplaceCreateBodyStyle {
            mode: Some(
                magic_hour::models::V1CharacterReplaceCreateBodyStyleModeEnum::Replace,
            ),
            selection_mode: Some(
                magic_hour::models::V1CharacterReplaceCreateBodyStyleSelectionModeEnum::Auto,
            ),
            ..Default::default()
        }),
    })
    .await;
```

#### Response

##### Type

[V1CharacterReplaceCreateResponse](/src/models/v1_character_replace_create_response.rs)

##### Example

```rust
V1CharacterReplaceCreateResponse {credits_charged: 450, estimated_frame_cost: 123, id: "cuid-example".to_string()}
```
