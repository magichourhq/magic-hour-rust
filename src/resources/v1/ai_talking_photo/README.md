# v1.ai_talking_photo

## Module Functions
### AI Talking Photo <a name="create"></a>

Create a talking photo from an image and audio or text input.

**API Endpoint**: `POST /v1/ai-talking-photo`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for creating a talking photo | `V1AiTalkingPhotoCreateBodyAssets {audio_file_path: "api-assets/id/1234.mp3".to_string(), image_file_path: "api-assets/id/1234.png".to_string()}` |
| `└─ audio_file_path` | ✓ | The audio file to sync with the image. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.mp3".to_string()` |
| `└─ image_file_path` | ✓ | The source image to animate. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.png".to_string()` |
| `end_seconds` | ✓ | The end time of the input audio in seconds. The maximum duration allowed is 60 seconds. | `15.0` |
| `start_seconds` | ✓ | The start time of the input audio in seconds. The maximum duration allowed is 60 seconds. | `0.0` |
| `name` | ✗ | The name of image. This value is mainly used for your own identification of the image. | `"Talking Photo image".to_string()` |
| `style` | ✗ | Attributes used to dictate the style of the output | `V1AiTalkingPhotoCreateBodyStyle {generation_mode: Some(V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Expressive), intensity: Some(1.5)}` |
| `└─ generation_mode` | ✗ | Controls overall motion style. * `pro` -  Realistic, high fidelity, accurate lip sync, slower. * `expressive` - More motion and facial expressiveness; may introduce visual artifacts. * `stable` -  Reduced motion for cleaner output; may result in minimal animation. (Deprecated: passing this value will be treated as `pro`) | `V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Expressive` |
| `└─ intensity` | ✗ | Note: this value is only applicable when generation_mode is `expressive`. The value can include up to 2 decimal places. * Lower values yield more stability but can suppress mouth movement. * Higher values increase motion and expressiveness, with a higher risk of distortion. | `1.5` |

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
        name: Some("Talking Photo image".to_string()),
        start_seconds: 0.0,
        ..Default::default()
    })
    .await;
```

#### Response

##### Type
[V1AiTalkingPhotoCreateResponse](/src/models/v1_ai_talking_photo_create_response.rs)

##### Example
`V1AiTalkingPhotoCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}`
<!-- CUSTOM DOCS START -->

<!-- CUSTOM DOCS END -->

