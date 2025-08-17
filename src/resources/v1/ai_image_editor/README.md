
### AI Image Editor <a name="create"></a>

Edit images with AI. Each edit costs 50 credits.

**API Endpoint**: `POST /v1/ai-image-editor`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for image edit | `V1AiImageEditorCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `style` | ✓ |  | `V1AiImageEditorCreateBodyStyle {prompt: "Give me sunglasses".to_string()}` |
| `name` | ✗ | The name of image. This value is mainly used for your own identification of the image. | `"Ai Image Editor image".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_image_editor()
    .create(magic_hour::resources::v1::ai_image_editor::CreateRequest {
        assets: magic_hour::models::V1AiImageEditorCreateBodyAssets {
            image_file_path: "api-assets/id/1234.png".to_string(),
        },
        name: Some("Ai Image Editor image".to_string()),
        style: magic_hour::models::V1AiImageEditorCreateBodyStyle {
            prompt: "Give me sunglasses".to_string(),
        },
    })
    .await;
```

#### Response

##### Type
[V1AiImageEditorCreateResponse](/src/models/v1_ai_image_editor_create_response.rs)

##### Example
`V1AiImageEditorCreateResponse {credits_charged: 50, frame_cost: 50, id: "cuid-example".to_string()}`
