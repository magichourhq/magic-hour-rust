
### create <a name="create"></a>
Create Animation

Create a Animation video. The estimated frame cost is calculated based on the `fps` and `end_seconds` input.

**API Endpoint**: `POST /v1/animation`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .animation()
    .create(magic_hour::resources::v1::animation::CreateRequest {
        data: magic_hour::models::PostV1AnimationBody {
            assets: magic_hour::models::PostV1AnimationBodyAssets {
                audio_file_path: Some("api-assets/id/1234.mp3".to_string()),
                audio_source: magic_hour::models::PostV1AnimationBodyAssetsAudioSourceEnum::File,
                image_file_path: Some("api-assets/id/1234.png".to_string()),
                youtube_url: Some("http://www.example.com".to_string()),
            },
            end_seconds: 15,
            fps: 12,
            height: 960,
            name: Some("Animation video".to_string()),
            style: magic_hour::models::PostV1AnimationBodyStyle {
                art_style: magic_hour::models::PostV1AnimationBodyStyleArtStyleEnum::PainterlyIllustration,
                art_style_custom: Some("string".to_string()),
                camera_effect: magic_hour::models::PostV1AnimationBodyStyleCameraEffectEnum::Accelerate,
                prompt: Some("Cyberpunk city".to_string()),
                prompt_type: magic_hour::models::PostV1AnimationBodyStylePromptTypeEnum::AiChoose,
                transition_speed: 5,
            },
            width: 512,
        },
    })
    .await;
```

**Upgrade to see all examples**
