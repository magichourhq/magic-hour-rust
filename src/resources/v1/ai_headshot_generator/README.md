# v1.ai_headshot_generator

## Module Functions

### AI Headshot Generator <a name="create"></a>

Create an AI headshot. Each headshot costs 50 credits.

**API Endpoint**: `POST /v1/ai-headshot-generator`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for headshot photo | `V1AiHeadshotGeneratorCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `└─ image_file_path` | ✓ | The image used to generate the headshot. This image must contain one detectable face. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.png".to_string()` |
| `name` | ✗ | The name of image. This value is mainly used for your own identification of the image. | `"Ai Headshot image".to_string()` |
| `style` | ✗ |  | `V1AiHeadshotGeneratorCreateBodyStyle {..Default::default()}` |
| `└─ prompt` | ✗ | Prompt used to guide the style of your headshot. We recommend omitting the prompt unless you want to customize your headshot. You can visit [AI headshot generator](https://magichour.ai/create/ai-headshot-generator) to view an example of a good prompt used for our 'Professional' style. | `"string".to_string()` |

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

#### Response

##### Type
[V1AiHeadshotGeneratorCreateResponse](/src/models/v1_ai_headshot_generator_create_response.rs)

##### Example
`V1AiHeadshotGeneratorCreateResponse {credits_charged: 50, frame_cost: 50, id: "cuid-example".to_string()}`

