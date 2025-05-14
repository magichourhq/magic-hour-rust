
### AI Meme Generator <a name="create"></a>

Create an AI generated meme. Each meme costs 10 credits.

**API Endpoint**: `POST /v1/ai-meme-generator`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_meme_generator()
    .create(magic_hour::resources::v1::ai_meme_generator::CreateRequest {
        name: Some("My Funny Meme".to_string()),
        style: magic_hour::models::V1AiMemeGeneratorCreateBodyStyle {
            search_web: Some(false),
            template: magic_hour::models::V1AiMemeGeneratorCreateBodyStyleTemplateEnum::DrakeHotlineBling,
            topic: "When the code finally works".to_string(),
        },
    })
    .await;
```
