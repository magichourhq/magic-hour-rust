# v1.voices

## Module Functions

### List voices <a name="list"></a>

List the voices available for text-to-speech. Use a voice's `id` as `style.voice_name` in the AI Voice Generator API. To fetch the next page, pass the response's next_cursor as cursor.

**API Endpoint**: `GET /v1/voices`

#### Parameters

| Parameter | Required | Description                                                        | Example                |
| --------- | :------: | ------------------------------------------------------------------ | ---------------------- |
| `cursor`  |    ✗     | Opaque pagination cursor from the previous response's next_cursor. | `"string".to_string()` |
| `limit`   |    ✗     | Maximum number of voices to return. Defaults to 20.                | `20`                   |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .voices()
    .list(magic_hour::resources::v1::voices::ListRequest {
        limit: Some(20),
        ..Default::default()
    })
    .await;
```

#### Response

##### Type

[V1VoicesListResponse](/src/models/v1_voices_list_response.rs)

##### Example

```rust
V1VoicesListResponse {next_cursor: Some("string".to_string()), voices: vec![V1VoicesListResponseVoicesItem {avatar_url: Some("https://d28dkohlqf5vwj.cloudfront.net/models/nano-banana/female-portrait-turtleneck.webp".to_string()), id: "cv-0001".to_string(), name: "Ashley".to_string(), preview_url: "https://videos.magichour.ai/presets/voices/v2/cv-0001-ashley.mp3".to_string()}]}
```
