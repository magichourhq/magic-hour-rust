
### create <a name="create"></a>
AI Talking Photo

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
        start_seconds: 0.0,
        ..Default::default()
    })
    .await;
```
