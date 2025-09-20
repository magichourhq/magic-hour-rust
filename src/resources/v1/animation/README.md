# v1.animation

## Module Functions

### Animation <a name="create"></a>

Create a Animation video. The estimated frame cost is calculated based on the `fps` and `end_seconds` input.

**API Endpoint**: `POST /v1/animation`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for animation. | `V1AnimationCreateBodyAssets {audio_file_path: Some("api-assets/id/1234.mp3".to_string()), audio_source: V1AnimationCreateBodyAssetsAudioSourceEnum::File, image_file_path: Some("api-assets/id/1234.png".to_string()), ..Default::default()}` |
| `└─ audio_file_path` | ✗ | The path of the input audio. This field is required if `audio_source` is `file`. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.mp3".to_string()` |
| `└─ audio_source` | ✓ | Optionally add an audio source if you'd like to incorporate audio into your video | `V1AnimationCreateBodyAssetsAudioSourceEnum::File` |
| `└─ image_file_path` | ✗ | An initial image to use a the first frame of the video. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.png".to_string()` |
| `└─ youtube_url` | ✗ | Using a youtube video as the input source. This field is required if `audio_source` is `youtube` | `"http://www.example.com".to_string()` |
| `end_seconds` | ✓ | This value determines the duration of the output video. | `15.0` |
| `fps` | ✓ | The desire output video frame rate | `12.0` |
| `height` | ✓ | The height of the final output video. The maximum height depends on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details | `960` |
| `style` | ✓ | Defines the style of the output video | `V1AnimationCreateBodyStyle {art_style: V1AnimationCreateBodyStyleArtStyleEnum::PainterlyIllustration, camera_effect: V1AnimationCreateBodyStyleCameraEffectEnum::SimpleZoomIn, prompt: Some("Cyberpunk city".to_string()), prompt_type: V1AnimationCreateBodyStylePromptTypeEnum::Custom, transition_speed: 5, ..Default::default()}` |
| `└─ art_style` | ✓ | The art style used to create the output video | `V1AnimationCreateBodyStyleArtStyleEnum::PainterlyIllustration` |
| `└─ art_style_custom` | ✗ | Describe custom art style. This field is required if `art_style` is `Custom` | `"string".to_string()` |
| `└─ camera_effect` | ✓ | The camera effect used to create the output video | `V1AnimationCreateBodyStyleCameraEffectEnum::SimpleZoomIn` |
| `└─ prompt` | ✗ | The prompt used for the video. Prompt is required if `prompt_type` is `custom`. Otherwise this value is ignored | `"Cyberpunk city".to_string()` |
| `└─ prompt_type` | ✓ |  * `custom` - Use your own prompt for the video. * `use_lyrics` - Use the lyrics of the audio to create the prompt. If this option is selected, then `assets.audio_source` must be `file` or `youtube`. * `ai_choose` - Let AI write the prompt. If this option is selected, then `assets.audio_source` must be `file` or `youtube`. | `V1AnimationCreateBodyStylePromptTypeEnum::Custom` |
| `└─ transition_speed` | ✓ | Change determines how quickly the video's content changes across frames.  * Higher = more rapid transitions. * Lower = more stable visual experience. | `5` |
| `width` | ✓ | The width of the final output video. The maximum width depends on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details | `512` |
| `name` | ✗ | The name of video. This value is mainly used for your own identification of the video. | `"Animation video".to_string()` |

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
            camera_effect: magic_hour::models::V1AnimationCreateBodyStyleCameraEffectEnum::SimpleZoomIn,
            prompt: Some("Cyberpunk city".to_string()),
            prompt_type: magic_hour::models::V1AnimationCreateBodyStylePromptTypeEnum::Custom,
            transition_speed: 5,
            ..Default::default()
        },
        width: 512,
    })
    .await;
```

#### Response

##### Type
[V1AnimationCreateResponse](/src/models/v1_animation_create_response.rs)

##### Example
`V1AnimationCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}`


