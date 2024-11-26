
### create <a name="create"></a>
Create Upscaled Image

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
        data: magic_hour::models::PostV1AiImageUpscalerBody {
            assets: magic_hour::models::PostV1AiImageUpscalerBodyAssets {
                image_file_path: "image/id/1234.png".to_string(),
            },
            name: Some("Image Upscaler image".to_string()),
            scale_factor: 123.45,
            style: magic_hour::models::PostV1AiImageUpscalerBodyStyle {
                enhancement: magic_hour::models::PostV1AiImageUpscalerBodyStyleEnhancementEnum::Balanced,
                prompt: Some("string".to_string()),
            },
        },
    })
    .await;
```

**Upgrade to see all examples**