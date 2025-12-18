# v1.auto_subtitle_generator

## Module Functions

### Auto Subtitle Generator <a name="create"></a>

Automatically generate subtitles for your video in multiple languages.

**API Endpoint**: `POST /v1/auto-subtitle-generator`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for auto subtitle generator | `V1AutoSubtitleGeneratorCreateBodyAssets {video_file_path: "api-assets/id/1234.mp4".to_string()}` |
| `└─ video_file_path` | ✓ | This is the video used to add subtitles. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.mp4".to_string()` |
| `end_seconds` | ✓ | The end time of the input video in seconds. This value is used to trim the input video. The value must be greater than 0.1, and more than the start_seconds. | `15.0` |
| `start_seconds` | ✓ | The start time of the input video in seconds. This value is used to trim the input video. The value must be greater than 0. | `0.0` |
| `style` | ✓ | Style of the subtitle. At least one of `.style.template` or `.style.custom_config` must be provided.  * If only `.style.template` is provided, default values for the template will be used. * If both are provided, the fields in `.style.custom_config` will be used to overwrite the fields in `.style.template`. * If only `.style.custom_config` is provided, then all fields in `.style.custom_config` will be used.  To use custom config only, the following `custom_config` params are required: * `.style.custom_config.font` * `.style.custom_config.text_color` * `.style.custom_config.vertical_position` * `.style.custom_config.horizontal_position`  | `V1AutoSubtitleGeneratorCreateBodyStyle {..Default::default()}` |
| `└─ custom_config` | ✗ | Custom subtitle configuration. | `V1AutoSubtitleGeneratorCreateBodyStyleCustomConfig {font: Some("Noto Sans".to_string()), font_size: Some(24.0), font_style: Some("normal".to_string()), highlighted_text_color: Some("#FFD700".to_string()), horizontal_position: Some("center".to_string()), stroke_color: Some("#000000".to_string()), stroke_width: Some(1.0), text_color: Some("#FFFFFF".to_string()), vertical_position: Some("bottom".to_string())}` |
| `└─ template` | ✗ | Preset subtitle templates. Please visit https://magichour.ai/create/auto-subtitle-generator to see the style of the existing templates. | `V1AutoSubtitleGeneratorCreateBodyStyleTemplateEnum::Cinematic` |
| `name` | ✗ | The name of video. This value is mainly used for your own identification of the video. | `"Auto Subtitle video".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .auto_subtitle_generator()
    .create(magic_hour::resources::v1::auto_subtitle_generator::CreateRequest {
        assets: magic_hour::models::V1AutoSubtitleGeneratorCreateBodyAssets {
            video_file_path: "api-assets/id/1234.mp4".to_string(),
        },
        end_seconds: 15.0,
        name: Some("Auto Subtitle video".to_string()),
        start_seconds: 0.0,
        style: magic_hour::models::V1AutoSubtitleGeneratorCreateBodyStyle {
            ..Default::default()
        },
    })
    .await;
```

#### Response

##### Type
[V1AutoSubtitleGeneratorCreateResponse](/src/models/v1_auto_subtitle_generator_create_response.rs)

##### Example
`V1AutoSubtitleGeneratorCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}`

