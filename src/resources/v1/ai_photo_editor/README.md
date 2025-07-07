
### AI Photo Editor <a name="create"></a>

> **NOTE**: this API is still in early development stages, and should be avoided. Please reach out to us if you're interested in this API. 

Edit photo using AI. Each photo costs 10 credits.

**API Endpoint**: `POST /v1/ai-photo-editor`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for photo editor | `V1AiPhotoEditorCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `resolution` | ✓ | The resolution of the final output image. The allowed value is based on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details | `768` |
| `style` | ✓ |  | `V1AiPhotoEditorCreateBodyStyle {image_description: "A photo of a person".to_string(), likeness_strength: 5.2, negative_prompt: Some("painting, cartoon, sketch".to_string()), prompt: "A photo portrait of a person wearing a hat".to_string(), prompt_strength: 3.75, steps: Some(4), upscale_factor: Some(2), upscale_fidelity: Some(0.5)}` |
| `name` | ✗ | The name of image | `"Photo Editor image".to_string()` |
| `steps` | ✗ | Deprecated: Please use `.style.steps` instead. Number of iterations used to generate the output. Higher values improve quality and increase the strength of the prompt but increase processing time. | `123` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_photo_editor()
    .create(magic_hour::resources::v1::ai_photo_editor::CreateRequest {
        assets: magic_hour::models::V1AiPhotoEditorCreateBodyAssets {
            image_file_path: "api-assets/id/1234.png".to_string(),
        },
        name: Some("Photo Editor image".to_string()),
        resolution: 768,
        style: magic_hour::models::V1AiPhotoEditorCreateBodyStyle {
            image_description: "A photo of a person".to_string(),
            likeness_strength: 5.2,
            negative_prompt: Some("painting, cartoon, sketch".to_string()),
            prompt: "A photo portrait of a person wearing a hat".to_string(),
            prompt_strength: 3.75,
            steps: Some(4),
            upscale_factor: Some(2),
            upscale_fidelity: Some(0.5),
        },
        ..Default::default()
    })
    .await;
```

#### Response

##### Type
[V1AiPhotoEditorCreateResponse](/src/models/v1_ai_photo_editor_create_response.rs)

##### Example
`V1AiPhotoEditorCreateResponse {credits_charged: 10, frame_cost: 10, id: "clx7uu86w0a5qp55yxz315r6r".to_string()}`
