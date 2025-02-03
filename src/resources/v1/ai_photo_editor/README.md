
### create <a name="create"></a>
AI Photo Editor

> **NOTE**: this API is still in early development stages, and should be avoided. Please reach out to us if you're interested in this API. 

Edit photo using AI. Each photo costs 10 frames.

**API Endpoint**: `POST /v1/ai-photo-editor`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_photo_editor()
    .create(magic_hour::resources::v1::ai_photo_editor::CreateRequest {
        assets: magic_hour::models::PostV1AiPhotoEditorBodyAssets {
            image_file_path: "api-assets/id/1234.png".to_string(),
        },
        resolution: 768,
        style: magic_hour::models::PostV1AiPhotoEditorBodyStyle {
            image_description: "A photo of a person".to_string(),
            likeness_strength: 5.2,
            negative_prompt: Some("painting, cartoon, sketch".to_string()),
            prompt: "A photo portrait of a person wearing a hat".to_string(),
            prompt_strength: 3.75,
            steps: Some(4),
        },
        ..Default::default()
    })
    .await;
```
