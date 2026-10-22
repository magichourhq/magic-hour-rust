# v1.ai_music_generator

## Module Functions

### AI Music Generator <a name="create"></a>

Generate music from a description, with optional custom lyrics or instrumental-only output. Costs 1 credits per requested second. Returns a project ID; poll GET /v1/audio-projects/{id} for status and downloads.

**API Endpoint**: `POST /v1/ai-music-generator`

#### Parameters

| Parameter               | Required | Description                                                                            | Example                                                                                                                                                                                                             |
| ----------------------- | :------: | -------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `duration`              |    ✓     | Requested track duration in seconds, from 15 to 300. Determines credits charged.       | `120`                                                                                                                                                                                                               |
| `style`                 |    ✓     |                                                                                        | `V1AiMusicGeneratorCreateBodyStyle {instrumental: Some(false), lyrics: Some("City lights are fading...".to_string()), prompt: "Dreamy synthwave with warm bass and soft vocals".to_string(), ..Default::default()}` |
| `└─ instrumental`       |    ✗     | Generate music without vocals. When true, lyrics is ignored.                           | `false`                                                                                                                                                                                                             |
| `└─ lyrics`             |    ✗     | Custom lyrics. Omit or leave blank for AI-generated lyrics when instrumental is false. | `"City lights are fading...".to_string()`                                                                                                                                                                           |
| `└─ prompt`             |    ✓     | Describe the song, including genre, mood, and instruments.                             | `"Dreamy synthwave with warm bass and soft vocals".to_string()`                                                                                                                                                     |
| `additional_properties` |    ✗     |                                                                                        | `std::collections::HashMap::<String, serde_json::Value>::new()`                                                                                                                                                     |
| `name`                  |    ✗     | Give your audio a custom name for easy identification.                                 | `"My Music Generator audio".to_string()`                                                                                                                                                                            |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_music_generator()
    .create(magic_hour::resources::v1::ai_music_generator::CreateRequest {
        duration: 120,
        name: Some("My Music Generator audio".to_string()),
        style: magic_hour::models::V1AiMusicGeneratorCreateBodyStyle {
            instrumental: Some(false),
            lyrics: Some("City lights are fading...".to_string()),
            prompt: "Dreamy synthwave with warm bass and soft vocals".to_string(),
            ..Default::default()
        },
        ..Default::default()
    })
    .await;
```

#### Response

##### Type

[V1AiMusicGeneratorCreateResponse](/src/models/v1_ai_music_generator_create_response.rs)

##### Example

```rust
V1AiMusicGeneratorCreateResponse {credits_charged: 120, id: "cuid-example".to_string()}
```
