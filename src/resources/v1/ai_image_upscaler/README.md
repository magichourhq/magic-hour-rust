# v1.ai_image_upscaler

## Module Functions

### AI Image Upscaler <a name="create"></a>

Upscale your image using AI. Each 2x upscale costs 50 credits, and 4x upscale costs 200 credits.

**API Endpoint**: `POST /v1/ai-image-upscaler`

#### Parameters

| Parameter            | Required | Description                                                                                                                                                                                                                                                                                                                                                              | Example                                                                                                                           |
| -------------------- | :------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------- |
| `assets`             |    ✓     | Provide the assets for upscaling                                                                                                                                                                                                                                                                                                                                         | `V1AiImageUpscalerCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}`                                       |
| `└─ image_file_path` |    ✓     | The image to upscale. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more. | `"api-assets/id/1234.png".to_string()`                                                                                            |
| `scale_factor`       |    ✓     | How much to scale the image. Must be either 2 or 4. Note: 4x upscale is only available on Creator, Pro, or Business tier.                                                                                                                                                                                                                                                | `2.0`                                                                                                                             |
| `style`              |    ✓     |                                                                                                                                                                                                                                                                                                                                                                          | `V1AiImageUpscalerCreateBodyStyle {enhancement: V1AiImageUpscalerCreateBodyStyleEnhancementEnum::Balanced, ..Default::default()}` |
| `└─ enhancement`     |    ✓     |                                                                                                                                                                                                                                                                                                                                                                          | `V1AiImageUpscalerCreateBodyStyleEnhancementEnum::Balanced`                                                                       |
| `└─ prompt`          |    ✗     | A prompt to guide the final image. This value is ignored if `enhancement` is not Creative                                                                                                                                                                                                                                                                                | `"string".to_string()`                                                                                                            |
| `name`               |    ✗     | The name of image. This value is mainly used for your own identification of the image.                                                                                                                                                                                                                                                                                   | `"Image Upscaler image".to_string()`                                                                                              |

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

#### Response

##### Type

[V1AiImageUpscalerCreateResponse](/src/models/v1_ai_image_upscaler_create_response.rs)

##### Example

```rust
V1AiImageUpscalerCreateResponse {credits_charged: 50, frame_cost: 50, id: "cuid-example".to_string()}
```
