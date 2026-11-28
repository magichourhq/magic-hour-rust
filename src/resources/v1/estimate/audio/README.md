# v1.estimate.audio

## Module Functions

### Estimate audio credits <a name="list"></a>

Get the credits an audio create request charges. Pass `tool`, then the field below that the tool's create request uses. Does not create a project or spend credits. It prices the fields passed here. It does not check your plan, your balance or the rest of the create request.

**API Endpoint**: `GET /v1/estimate/audio`

#### Parameters

| Parameter         | Required | Description                                                                                                            | Example                                         |
| ----------------- | :------: | ---------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- |
| `tool`            |    ✓     | The create endpoint to price, named by its path. For example `ai-voice-generator` for `POST /v1/ai-voice-generator`.   | `V1EstimateAudioListToolEnum::AiVoiceGenerator` |
| `character_count` |    ✗     | The number of characters in the `style.prompt` of the create request. Used by `ai-voice-generator`, `ai-voice-cloner`. | `120`                                           |
| `duration`        |    ✗     | The `duration` of the create request. Used by `ai-music-generator`.                                                    | `123`                                           |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .estimate()
    .audio()
    .list(magic_hour::resources::v1::estimate::audio::ListRequest {
        tool: magic_hour::models::V1EstimateAudioListToolEnum::AiVoiceGenerator,
        ..Default::default()
    })
    .await;
```

#### Response

##### Type

[V1EstimateAudioListResponse](/src/models/v1_estimate_audio_list_response.rs)

##### Example

```rust
V1EstimateAudioListResponse {estimated_credits: 150}
```
