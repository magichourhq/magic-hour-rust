# v1.ai_gif_generator

## Module Functions

### AI GIF Generator <a name="create"></a>

Create an AI GIF. Each GIF costs 50 credits.

**API Endpoint**: `POST /v1/ai-gif-generator`

#### Parameters

| Parameter       | Required | Description                                          | Example                                                                               |
| --------------- | :------: | ---------------------------------------------------- | ------------------------------------------------------------------------------------- |
| `style`         |    ✓     |                                                      | `V1AiGifGeneratorCreateBodyStyle {prompt: "Cute dancing cat, pixel art".to_string()}` |
| `└─ prompt`     |    ✓     | The prompt used for the GIF.                         | `"Cute dancing cat, pixel art".to_string()`                                           |
| `name`          |    ✗     | Give your gif a custom name for easy identification. | `"My Ai Gif gif".to_string()`                                                         |
| `output_format` |    ✗     | The output file format for the generated animation.  | `V1AiGifGeneratorCreateBodyOutputFormatEnum::Gif`                                     |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_gif_generator()
    .create(magic_hour::resources::v1::ai_gif_generator::CreateRequest {
        name: Some("My Ai Gif gif".to_string()),
        output_format: Some(
            magic_hour::models::V1AiGifGeneratorCreateBodyOutputFormatEnum::Gif,
        ),
        style: magic_hour::models::V1AiGifGeneratorCreateBodyStyle {
            prompt: "Cute dancing cat, pixel art".to_string(),
        },
    })
    .await;
```

#### Response

##### Type

[V1AiGifGeneratorCreateResponse](/src/models/v1_ai_gif_generator_create_response.rs)

##### Example

```rust
V1AiGifGeneratorCreateResponse {credits_charged: 50, frame_cost: 50, id: "cuid-example".to_string()}
```
