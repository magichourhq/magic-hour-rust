# v1.ai_image_generator

## Module Functions

### AI Image Generator <a name="create"></a>

Create an AI image. Each standard image costs 5 credits. Pro quality images cost 30 credits.

**API Endpoint**: `POST /v1/ai-image-generator`

#### Parameters

| Parameter         | Required | Description                                                                                                                                                                                                                                                                                                                                        | Example                                                                                                                                                                                                                         |
| ----------------- | :------: | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `image_count`     |    ✓     | Number of images to generate.                                                                                                                                                                                                                                                                                                                      | `1`                                                                                                                                                                                                                             |
| `orientation`     |    ✓     | The orientation of the output image(s).                                                                                                                                                                                                                                                                                                            | `V1AiImageGeneratorCreateBodyOrientationEnum::Landscape`                                                                                                                                                                        |
| `style`           |    ✓     | The art style to use for image generation.                                                                                                                                                                                                                                                                                                         | `V1AiImageGeneratorCreateBodyStyle {prompt: "Cool image".to_string(), quality_mode: Some(V1AiImageGeneratorCreateBodyStyleQualityModeEnum::Standard), tool: Some(V1AiImageGeneratorCreateBodyStyleToolEnum::AiAnimeGenerator)}` |
| `└─ prompt`       |    ✓     | The prompt used for the image(s).                                                                                                                                                                                                                                                                                                                  | `"Cool image".to_string()`                                                                                                                                                                                                      |
| `└─ quality_mode` |    ✗     | Controls the quality of the generated image. Defaults to 'standard' if not specified. **Options:** - `standard` - Standard quality generation. Cost: 5 credits per image. - `pro` - Pro quality generation with enhanced details and quality. Cost: 30 credits per image. Note: Pro mode is available for users on Creator, Pro, or Business tier. | `V1AiImageGeneratorCreateBodyStyleQualityModeEnum::Standard`                                                                                                                                                                    |
| `└─ tool`         |    ✗     | The art style to use for image generation. Defaults to 'general' if not provided.                                                                                                                                                                                                                                                                  | `V1AiImageGeneratorCreateBodyStyleToolEnum::AiAnimeGenerator`                                                                                                                                                                   |
| `name`            |    ✗     | Give your image a custom name for easy identification.                                                                                                                                                                                                                                                                                             | `"My Ai Image image".to_string()`                                                                                                                                                                                               |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_image_generator()
    .create(magic_hour::resources::v1::ai_image_generator::CreateRequest {
        image_count: 1,
        name: Some("My Ai Image image".to_string()),
        orientation: magic_hour::models::V1AiImageGeneratorCreateBodyOrientationEnum::Landscape,
        style: magic_hour::models::V1AiImageGeneratorCreateBodyStyle {
            prompt: "Cool image".to_string(),
            quality_mode: Some(
                magic_hour::models::V1AiImageGeneratorCreateBodyStyleQualityModeEnum::Standard,
            ),
            tool: Some(
                magic_hour::models::V1AiImageGeneratorCreateBodyStyleToolEnum::AiAnimeGenerator,
            ),
        },
    })
    .await;
```

#### Response

##### Type

[V1AiImageGeneratorCreateResponse](/src/models/v1_ai_image_generator_create_response.rs)

##### Example

```rust
V1AiImageGeneratorCreateResponse {credits_charged: 5, frame_cost: 5, id: "cuid-example".to_string()}
```
