
### Auto Subtitle Generator <a name="create"></a>

Automatically generate subtitles for your video in multiple languages.

**API Endpoint**: `POST /v1/auto-subtitle-generator`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for auto subtitle generator | `V1AutoSubtitleGeneratorCreateBodyAssets {video_file_path: "api-assets/id/1234.mp4".to_string()}` |
| `end_seconds` | ✓ | The end time of the input video in seconds | `15.0` |
| `start_seconds` | ✓ | The start time of the input video in seconds | `0.0` |
| `style` | ✓ | Style of the subtitle. At least one of `.style.template` or `.style.custom_config` must be provided.  * If only `.style.template` is provided, default values for the template will be used. * If both are provided, the fields in `.style.custom_config` will be used to overwrite the fields in `.style.template`. * If only `.style.custom_config` is provided, then all fields in `.style.custom_config` will be used.  To use custom config only, the following `custom_config` params are required: * `.style.custom_config.font` * `.style.custom_config.text_color` * `.style.custom_config.vertical_position` * `.style.custom_config.horizontal_position`  | `V1AutoSubtitleGeneratorCreateBodyStyle {..Default::default()}` |
| `name` | ✗ | The name of video | `"Auto Subtitle video".to_string()` |

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
`V1AutoSubtitleGeneratorCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "clx7uu86w0a5qp55yxz315r6r".to_string()}`
