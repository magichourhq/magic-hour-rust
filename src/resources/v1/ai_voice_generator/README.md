# v1.ai_voice_generator

## Module Functions

### AI Voice Generator <a name="create"></a>

Generate speech from text. Each character costs 0.1 credits. The cost is rounded up to the nearest whole number.

**API Endpoint**: `POST /v1/ai-voice-generator`

#### Parameters

| Parameter       | Required | Description                                                                                                               | Example                                                                                                                                                                                                                                          |
| --------------- | :------: | ------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `style`         |    ✓     | The content used to generate speech.                                                                                      | `V1AiVoiceGeneratorCreateBodyStyle {language: Some(V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Spanish), prompt: "Hello, how are you?".to_string(), speed: Some(1.15), voice_name: V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElonMusk}` |
| `└─ language`   |    ✗     | Spoken language. Use a canonical lowercase language name, or auto to detect the language from the text. Defaults to auto. | `V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Spanish`                                                                                                                                                                                         |
| `└─ prompt`     |    ✓     | Text used to generate speech. The character limit is 5000 characters.                                                     | `"Hello, how are you?".to_string()`                                                                                                                                                                                                              |
| `└─ speed`      |    ✗     | Speech speed multiplier, from 0.75 to 1.30 inclusive. Defaults to 1 (normal speed).                                       | `1.15`                                                                                                                                                                                                                                           |
| `└─ voice_name` |    ✓     | The voice to use for the speech. Use the `id` of a voice returned by `GET /v1/voices`.                                    | `V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElonMusk`                                                                                                                                                                                       |
| `name`          |    ✗     | Give your audio a custom name for easy identification.                                                                    | `"My Voice Generator audio".to_string()`                                                                                                                                                                                                         |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_voice_generator()
    .create(magic_hour::resources::v1::ai_voice_generator::CreateRequest {
        name: Some("My Voice Generator audio".to_string()),
        style: magic_hour::models::V1AiVoiceGeneratorCreateBodyStyle {
            language: Some(
                magic_hour::models::V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Spanish,
            ),
            prompt: "Hello, how are you?".to_string(),
            speed: Some(1.15),
            voice_name: magic_hour::models::V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElonMusk,
        },
    })
    .await;
```

#### Response

##### Type

[V1AiVoiceGeneratorCreateResponse](/src/models/v1_ai_voice_generator_create_response.rs)

##### Example

```rust
V1AiVoiceGeneratorCreateResponse {credits_charged: 1, id: "cuid-example".to_string()}
```
