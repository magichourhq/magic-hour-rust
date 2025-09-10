# v1.ai_image_generator

## Module Functions
### AI Images <a name="create"></a>

Create an AI image. Each image costs 5 credits.

**API Endpoint**: `POST /v1/ai-image-generator`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `image_count` | ✓ | Number of images to generate. | `1` |
| `orientation` | ✓ | The orientation of the output image(s). | `V1AiImageGeneratorCreateBodyOrientationEnum::Landscape` |
| `style` | ✓ | The art style to use for image generation. | `V1AiImageGeneratorCreateBodyStyle {prompt: "Cool image".to_string(), tool: Some(V1AiImageGeneratorCreateBodyStyleToolEnum::AiAnimeGenerator)}` |
| `└─ prompt` | ✓ | The prompt used for the image(s). | `"Cool image".to_string()` |
| `└─ tool` | ✗ | The art style to use for image generation. Defaults to 'general' if not provided. | `V1AiImageGeneratorCreateBodyStyleToolEnum::AiAnimeGenerator` |
| `name` | ✗ | The name of image. This value is mainly used for your own identification of the image. | `"Ai Image image".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_image_generator()
    .create(magic_hour::resources::v1::ai_image_generator::CreateRequest {
        image_count: 1,
        name: Some("Ai Image image".to_string()),
        orientation: magic_hour::models::V1AiImageGeneratorCreateBodyOrientationEnum::Landscape,
        style: magic_hour::models::V1AiImageGeneratorCreateBodyStyle {
            prompt: "Cool image".to_string(),
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
`V1AiImageGeneratorCreateResponse {credits_charged: 5, frame_cost: 5, id: "cuid-example".to_string()}`
<!-- CUSTOM DOCS START -->

<!-- CUSTOM DOCS END -->

