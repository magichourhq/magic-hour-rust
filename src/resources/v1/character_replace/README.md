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

| Parameter          | Required | Description                                                                              | Example                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------------ | :------: | ---------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `data`             |    ✗     |                                                                                          | `V1CharacterReplaceCreateBody {assets: V1CharacterReplaceCreateBodyAssets {image_file_path: "api-assets/id/5678.png".to_string(), video_file_path: "api-assets/id/1234.mp4".to_string()}, end_seconds: 15.0, name: Some("My Character Replace video".to_string()), resolution: Some(V1CharacterReplaceCreateBodyResolutionEnum::Enum720p), start_seconds: Some(0.0), ..Default::default()}` |
| `└─ assets`        |    ✓     | Source video and reference character image for the job.                                  | `V1CharacterReplaceCreateBodyAssets {image_file_path: "api-assets/id/5678.png".to_string(), video_file_path: "api-assets/id/1234.mp4".to_string()}`                                                                                                                                                                                                                                         |
| `└─ end_seconds`   |    ✓     | End time of your clip (seconds). Must be greater than start_seconds.                     | `15.0`                                                                                                                                                                                                                                                                                                                                                                                      |
| `└─ name`          |    ✗     | Give your video a custom name for easy identification.                                   | `"My Character Replace video".to_string()`                                                                                                                                                                                                                                                                                                                                                  |
| `└─ resolution`    |    ✗     | Output video resolution. Defaults to 480p, the lowest resolution available on your plan. | `V1CharacterReplaceCreateBodyResolutionEnum::Enum720p`                                                                                                                                                                                                                                                                                                                                      |
| `└─ start_seconds` |    ✗     | Start time of your clip (seconds). Must be ≥ 0.                                          | `0.0`                                                                                                                                                                                                                                                                                                                                                                                       |
| `└─ style`         |    ✗     | Optional style controls for replace vs animate mode and subject selection.               | `V1CharacterReplaceCreateBodyStyle {mode: Some(V1CharacterReplaceCreateBodyStyleModeEnum::Replace), selection_mode: Some(V1CharacterReplaceCreateBodyStyleSelectionModeEnum::Auto), ..Default::default()}`                                                                                                                                                                                  |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .character_replace()
    .create(magic_hour::resources::v1::character_replace::CreateRequest {
        ..Default::default()
    })
    .await;
```

#### Response

##### Type

[V1CharacterReplaceCreateResponse](/src/models/v1_character_replace_create_response.rs)

##### Example

```rust
V1CharacterReplaceCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}
```
