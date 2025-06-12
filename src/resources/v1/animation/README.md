
### Animation <a name="create"></a>

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
        assets: magic_hour::models::V1AnimationCreateBodyAssets {
            audio_file_path: Some("api-assets/id/1234.mp3".to_string()),
            audio_source: magic_hour::models::V1AnimationCreateBodyAssetsAudioSourceEnum::File,
            image_file_path: Some("api-assets/id/1234.png".to_string()),
            ..Default::default()
        },
        end_seconds: 15.0,
        fps: 12.0,
        height: 960,
        name: Some("Animation video".to_string()),
        style: magic_hour::models::V1AnimationCreateBodyStyle {
            art_style: magic_hour::models::V1AnimationCreateBodyStyleArtStyleEnum::PainterlyIllustration,
            camera_effect: magic_hour::models::V1AnimationCreateBodyStyleCameraEffectEnum::Accelerate,
            prompt: Some("Cyberpunk city".to_string()),
            prompt_type: magic_hour::models::V1AnimationCreateBodyStylePromptTypeEnum::AiChoose,
            transition_speed: 5,
            ..Default::default()
        },
        width: 512,
    })
    .await;
```

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for animation. | `V1AnimationCreateBodyAssets {audio_file_path: Some("api-assets/id/1234.mp3".to_string()), audio_source: V1AnimationCreateBodyAssetsAudioSourceEnum::File, image_file_path: Some("api-assets/id/1234.png".to_string()), ..Default::default()}` |
| `end_seconds` | ✓ | The end time of the input video in seconds | `15.0` |
| `fps` | ✓ | The desire output video frame rate | `12.0` |
| `height` | ✓ | The height of the final output video. The maximum height depends on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details | `960` |
| `style` | ✓ | Defines the style of the output video | `V1AnimationCreateBodyStyle {art_style: V1AnimationCreateBodyStyleArtStyleEnum::PainterlyIllustration, camera_effect: V1AnimationCreateBodyStyleCameraEffectEnum::Accelerate, prompt: Some("Cyberpunk city".to_string()), prompt_type: V1AnimationCreateBodyStylePromptTypeEnum::AiChoose, transition_speed: 5, ..Default::default()}` |
| `width` | ✓ | The width of the final output video. The maximum width depends on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details | `512` |
| `name` | ✗ | The name of video | `"Animation video".to_string()` |
