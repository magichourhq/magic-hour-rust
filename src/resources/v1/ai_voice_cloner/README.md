# v1.ai_voice_cloner

## Module Functions

### AI Voice Cloner <a name="create"></a>

Clone a voice from an audio sample and generate speech.

- Each character costs 0.05 credits.
- The cost is rounded up to the nearest whole number

**API Endpoint**: `POST /v1/ai-voice-cloner`

#### Parameters

| Parameter            | Required | Description                                                                                                                                                                                                                                                                                                                                                      | Example                                                                                   |
| -------------------- | :------: | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| `assets`             |    ✓     | Provide the assets for voice cloning.                                                                                                                                                                                                                                                                                                                            | `V1AiVoiceClonerCreateBodyAssets {audio_file_path: "api-assets/id/1234.mp3".to_string()}` |
| `└─ audio_file_path` |    ✓     | The audio used to clone the voice. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details. | `"api-assets/id/1234.mp3".to_string()`                                                    |
| `style`              |    ✓     |                                                                                                                                                                                                                                                                                                                                                                  | `V1AiVoiceClonerCreateBodyStyle {prompt: "Hello, this is my cloned voice.".to_string()}`  |
| `└─ prompt`          |    ✓     | Text used to generate speech from the cloned voice. The character limit is 1000 characters.                                                                                                                                                                                                                                                                      | `"Hello, this is my cloned voice.".to_string()`                                           |
| `name`               |    ✗     | Give your audio a custom name for easy identification.                                                                                                                                                                                                                                                                                                           | `"My Voice Cloner audio".to_string()`                                                     |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_voice_cloner()
    .create(magic_hour::resources::v1::ai_voice_cloner::CreateRequest {
        assets: magic_hour::models::V1AiVoiceClonerCreateBodyAssets {
            audio_file_path: "api-assets/id/1234.mp3".to_string(),
        },
        name: Some("My Voice Cloner audio".to_string()),
        style: magic_hour::models::V1AiVoiceClonerCreateBodyStyle {
            prompt: "Hello, this is my cloned voice.".to_string(),
        },
    })
    .await;
```

#### Response

##### Type

[V1AiVoiceClonerCreateResponse](/src/models/v1_ai_voice_cloner_create_response.rs)

##### Example

```rust
V1AiVoiceClonerCreateResponse {credits_charged: 1, id: "cuid-example".to_string()}
```
