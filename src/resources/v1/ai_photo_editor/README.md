
### AI Photo Editor <a name="create"></a>

> **NOTE**: this API is still in early development stages, and should be avoided. Please reach out to us if you're interested in this API. 

Edit photo using AI. Each photo costs 10 credits.

**API Endpoint**: `POST /v1/ai-photo-editor`

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
