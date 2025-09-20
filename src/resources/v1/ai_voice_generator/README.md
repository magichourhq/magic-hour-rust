# v1.ai_voice_generator

## Module Functions

### AI Voice Generator <a name="create"></a>

Generate speech from text. Each character costs 0.05 credits. The cost is rounded up to the nearest whole number.

**API Endpoint**: `POST /v1/ai-voice-generator`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `style` | ✓ | The content used to generate speech. | `V1AiVoiceGeneratorCreateBodyStyle {prompt: "Hello, how are you?".to_string(), voice_name: V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElonMusk}` |
| `└─ prompt` | ✓ | Text used to generate speech. Starter tier users can use up to 200 characters, while Creator, Pro, or Business users can use up to 1000. | `"Hello, how are you?".to_string()` |
| `└─ voice_name` | ✓ | The voice to use for the speech. Available voices: Elon Musk, Mark Zuckerberg, Joe Rogan, Barack Obama, Morgan Freeman, Kanye West, Donald Trump, Joe Biden, Kim Kardashian, Taylor Swift | `V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElonMusk` |
| `name` | ✗ | The name of audio. This value is mainly used for your own identification of the audio. | `"Voice Generator audio".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_voice_generator()
    .create(magic_hour::resources::v1::ai_voice_generator::CreateRequest {
        name: Some("Voice Generator audio".to_string()),
        style: magic_hour::models::V1AiVoiceGeneratorCreateBodyStyle {
            prompt: "Hello, how are you?".to_string(),
            voice_name: magic_hour::models::V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElonMusk,
        },
    })
    .await;
```

#### Response

##### Type
[V1AiVoiceGeneratorCreateResponse](/src/models/v1_ai_voice_generator_create_response.rs)

##### Example
`V1AiVoiceGeneratorCreateResponse {credits_charged: 1, id: "cuid-example".to_string()}`


