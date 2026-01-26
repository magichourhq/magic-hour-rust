# v1.ai_talking_photo

## Module Functions

### AI Talking Photo <a name="create"></a>

Create a talking photo from an image and audio or text input.

**API Endpoint**: `POST /v1/ai-talking-photo`

#### Parameters

| Parameter            | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                    | Example                                                                                                                                           |
| -------------------- | :------: | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| `assets`             |    ✓     | Provide the assets for creating a talking photo                                                                                                                                                                                                                                                                                                                                                                | `V1AiTalkingPhotoCreateBodyAssets {audio_file_path: "api-assets/id/1234.mp3".to_string(), image_file_path: "api-assets/id/1234.png".to_string()}` |
| `└─ audio_file_path` |    ✓     | The audio file to sync with the image. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.                                           | `"api-assets/id/1234.mp3".to_string()`                                                                                                            |
| `└─ image_file_path` |    ✓     | The source image to animate. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.                                                     | `"api-assets/id/1234.png".to_string()`                                                                                                            |
| `end_seconds`        |    ✓     | The end time of the input audio in seconds. The maximum duration allowed is 60 seconds.                                                                                                                                                                                                                                                                                                                        | `15.0`                                                                                                                                            |
| `start_seconds`      |    ✓     | The start time of the input audio in seconds. The maximum duration allowed is 60 seconds.                                                                                                                                                                                                                                                                                                                      | `0.0`                                                                                                                                             |
| `max_resolution`     |    ✗     | Constrains the larger dimension (height or width) of the output video. Allows you to set a lower resolution than your plan's maximum if desired. The value is capped by your plan's max resolution.                                                                                                                                                                                                            | `1024`                                                                                                                                            |
| `name`               |    ✗     | Give your image a custom name for easy identification.                                                                                                                                                                                                                                                                                                                                                         | `"My Talking Photo image".to_string()`                                                                                                            |
| `style`              |    ✗     | Attributes used to dictate the style of the output                                                                                                                                                                                                                                                                                                                                                             | `V1AiTalkingPhotoCreateBodyStyle {generation_mode: Some(V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Realistic), ..Default::default()}`     |
| `└─ generation_mode` |    ✗     | Controls overall motion style. * `realistic` - Maintains likeness well, high quality, and reliable. * `prompted` - Slightly lower likeness; allows option to prompt scene. **Deprecated values (maintained for backward compatibility):** * `pro` - Deprecated: use `realistic` * `standard` - Deprecated: use `prompted` * `stable` - Deprecated: use `realistic` * `expressive` - Deprecated: use `prompted` | `V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Realistic`                                                                                    |
| `└─ intensity`       |    ✗     | Note: this value is only applicable when generation_mode is `expressive`. The value can include up to 2 decimal places. * Lower values yield more stability but can suppress mouth movement. * Higher values increase motion and expressiveness, with a higher risk of distortion.                                                                                                                             | `123.0`                                                                                                                                           |
| `└─ prompt`          |    ✗     | A text prompt to guide the generation. Only applicable when generation_mode is `prompted`. This field is ignored for other modes.                                                                                                                                                                                                                                                                              | `"string".to_string()`                                                                                                                            |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_talking_photo()
    .create(magic_hour::resources::v1::ai_talking_photo::CreateRequest {
        assets: magic_hour::models::V1AiTalkingPhotoCreateBodyAssets {
            audio_file_path: "api-assets/id/1234.mp3".to_string(),
            image_file_path: "api-assets/id/1234.png".to_string(),
        },
        end_seconds: 15.0,
        max_resolution: Some(1024),
        name: Some("My Talking Photo image".to_string()),
        start_seconds: 0.0,
        ..Default::default()
    })
    .await;
```

#### Response

##### Type

[V1AiTalkingPhotoCreateResponse](/src/models/v1_ai_talking_photo_create_response.rs)

##### Example

```rust
V1AiTalkingPhotoCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}
```
