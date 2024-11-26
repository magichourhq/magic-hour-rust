
### create <a name="create"></a>
Create AI Headshots

Create an AI headshot. Each headshot costs 50 frames.

**API Endpoint**: `POST /v1/ai-headshot-generator`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_headshot_generator()
    .create(magic_hour::resources::v1::ai_headshot_generator::CreateRequest {
        data: magic_hour::models::PostV1AiHeadshotGeneratorBody {
            assets: magic_hour::models::PostV1AiHeadshotGeneratorBodyAssets {
                image_file_path: "image/id/1234.png".to_string(),
            },
            name: Some("Ai Headshot image".to_string()),
        },
    })
    .await;
```

**Upgrade to see all examples**
