
### create <a name="create"></a>
AI Image Upscaler

Upscale your image using AI. Each 2x upscale costs 50 frames, and 4x upscale costs 200 frames.

**API Endpoint**: `POST /v1/ai-image-upscaler`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_image_upscaler()
    .create(magic_hour::resources::v1::ai_image_upscaler::CreateRequest {
        assets: magic_hour::models::PostV1AiImageUpscalerBodyAssets {
            image_file_path: "image/id/1234.png".to_string(),
        },
        scale_factor: 123.45,
        style: magic_hour::models::PostV1AiImageUpscalerBodyStyle {
            enhancement: magic_hour::models::PostV1AiImageUpscalerBodyStyleEnhancementEnum::Balanced,
            ..Default::default()
        },
        ..Default::default()
    })
    .await;
```
