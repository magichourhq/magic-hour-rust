# v1.ai_image_upscaler

## Module Functions

### AI Image Upscaler <a name="create"></a>

Upscale your image using AI. Each 2x upscale costs 50 credits for balanced/creative modes, and 25 credits for preserve. 4x upscale costs 200 and 100 credits respectively.

**API Endpoint**: `POST /v1/ai-image-upscaler`

#### Parameters

| Parameter            | Required | Description                                                                                                                                                                                                                                                                                                                                                                                        | Example                                                                                                                   |
| -------------------- | :------: | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `assets`             |    ✓     | Provide the assets for upscaling                                                                                                                                                                                                                                                                                                                                                                   | `V1AiImageUpscalerCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}`                               |
| `└─ image_file_path` |    ✓     | The image to upscale. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details. . The maximum input image size is 4096x4096px. | `"api-assets/id/1234.png".to_string()`                                                                                    |
| `scale_factor`       |    ✓     | How much to scale the image. Must be either 2 or 4. Note: 4x upscale is only available on Creator, Pro, or Business tier.                                                                                                                                                                                                                                                                          | `2.0`                                                                                                                     |
| `name`               |    ✗     | Give your image a custom name for easy identification.                                                                                                                                                                                                                                                                                                                                             | `"My Image Upscaler image".to_string()`                                                                                   |
| `style`              |    ✗     | Style settings for the upscale. Use `mode` (`"preserve"`, `"balanced"`, or `"creative"`). Defaults to `"balanced"`.                                                                                                                                                                                                                                                                                | `V1AiImageUpscalerCreateBodyStyle {mode: Some(V1AiImageUpscalerCreateBodyStyleModeEnum::Balanced), ..Default::default()}` |
| `└─ enhancement`     |    ✗     | Deprecated: use `mode` instead. `"Resemblance"` maps to `"preserve"`. `"Balanced"` and `"Creative"` map to the same-named modes.                                                                                                                                                                                                                                                                   | `V1AiImageUpscalerCreateBodyStyleEnhancementEnum::Balanced`                                                               |
| `└─ mode`            |    ✗     | The upscaling mode. `"preserve"` uses the fast pro pipeline (1× credit multiplier). `"balanced"` and `"creative"` use the creative pipeline (2× credit multiplier). `"pro"` is deprecated and maps to `"preserve"`. Defaults to `"balanced"`.                                                                                                                                                      | `V1AiImageUpscalerCreateBodyStyleModeEnum::Balanced`                                                                      |
| `└─ prompt`          |    ✗     | A prompt to guide the final image. Only used when mode is `creative`.                                                                                                                                                                                                                                                                                                                              | `"string".to_string()`                                                                                                    |

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
        name: Some("My Image Upscaler image".to_string()),
        scale_factor: 2.0,
        ..Default::default()
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
