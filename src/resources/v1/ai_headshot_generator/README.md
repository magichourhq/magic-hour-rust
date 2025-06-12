
### AI Headshots <a name="create"></a>

Create an AI headshot. Each headshot costs 50 credits.

**API Endpoint**: `POST /v1/ai-headshot-generator`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_headshot_generator()
    .create(magic_hour::resources::v1::ai_headshot_generator::CreateRequest {
        assets: magic_hour::models::V1AiHeadshotGeneratorCreateBodyAssets {
            image_file_path: "api-assets/id/1234.png".to_string(),
        },
        name: Some("Ai Headshot image".to_string()),
        ..Default::default()
    })
    .await;
```

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for headshot photo | `V1AiHeadshotGeneratorCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `name` | ✗ | The name of image | `"Ai Headshot image".to_string()` |
| `style` | ✗ |  | `V1AiHeadshotGeneratorCreateBodyStyle {prompt: Some("professional passport photo, business attire, smiling, good posture, light blue background, centered, plain background".to_string())}` |
