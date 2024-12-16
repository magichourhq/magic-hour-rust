
### create <a name="create"></a>
Create AI Images

Create an AI image. Each image costs 5 frames.

**API Endpoint**: `POST /v1/ai-image-generator`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_image_generator()
    .create(magic_hour::resources::v1::ai_image_generator::CreateRequest {
        image_count: 1,
        orientation: magic_hour::models::PostV1AiImageGeneratorBodyOrientationEnum::Landscape,
        style: magic_hour::models::PostV1AiImageGeneratorBodyStyle {
            prompt: "Cool image".to_string(),
        },
        ..Default::default()
    })
    .await;
```

**Upgrade to see all examples**
