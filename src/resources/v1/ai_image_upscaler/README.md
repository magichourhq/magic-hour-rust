
### AI Image Upscaler <a name="create"></a>

Upscale your image using AI. Each 2x upscale costs 50 credits, and 4x upscale costs 200 credits.

**API Endpoint**: `POST /v1/ai-image-upscaler`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_image_upscaler()
    .create(magic_hour::resources::v1::ai_image_upscaler::CreateRequest {
        assets: magic_hour::models::V1AiImageUpscalerCreateBodyAssets {
            image_file_path: "api-assets/id/1234.png".to_string(),
        },
        name: Some("Image Upscaler image".to_string()),
        scale_factor: 2.0,
        style: magic_hour::models::V1AiImageUpscalerCreateBodyStyle {
            enhancement: magic_hour::models::V1AiImageUpscalerCreateBodyStyleEnhancementEnum::Balanced,
            ..Default::default()
        },
    })
    .await;
```

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for upscaling | `V1AiImageUpscalerCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `scale_factor` | ✓ | How much to scale the image. Must be either 2 or 4 | `2.0` |
| `style` | ✓ |  | `V1AiImageUpscalerCreateBodyStyle {enhancement: V1AiImageUpscalerCreateBodyStyleEnhancementEnum::Balanced, ..Default::default()}` |
| `name` | ✗ | The name of image | `"Image Upscaler image".to_string()` |
