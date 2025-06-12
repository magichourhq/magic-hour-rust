
### AI Talking Photo <a name="create"></a>

Create a talking photo from an image and audio or text input.

**API Endpoint**: `POST /v1/ai-talking-photo`

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

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for creating a talking photo | `V1AiTalkingPhotoCreateBodyAssets {audio_file_path: "api-assets/id/1234.mp3".to_string(), image_file_path: "api-assets/id/1234.png".to_string()}` |
| `end_seconds` | ✓ | The end time of the input audio in seconds. The maximum duration allowed is 30 seconds. | `15.0` |
| `start_seconds` | ✓ | The start time of the input audio in seconds. The maximum duration allowed is 30 seconds. | `0.0` |
| `name` | ✗ | The name of image | `"Talking Photo image".to_string()` |
| `style` | ✗ | Attributes used to dictate the style of the output | `V1AiTalkingPhotoCreateBodyStyle {generation_mode: Some(V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum::Expressive), intensity: Some(1.5)}` |
