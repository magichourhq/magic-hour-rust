
### AI GIFs <a name="create"></a>

Create an AI GIF. Each GIF costs 25 credits.

**API Endpoint**: `POST /v1/ai-gif-generator`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_gif_generator()
    .create(magic_hour::resources::v1::ai_gif_generator::CreateRequest {
        name: Some("Ai Gif gif".to_string()),
        style: magic_hour::models::V1AiGifGeneratorCreateBodyStyle {
            prompt: "Cute dancing cat, pixel art".to_string(),
        },
    })
    .await;
```
