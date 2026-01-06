# v1.ai_image_editor

## Module Functions

### AI Image Editor <a name="create"></a>

Edit images with AI. Each edit costs 10 credits.

**API Endpoint**: `POST /v1/ai-image-editor`

#### Parameters

| Parameter             | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Example                                                                                                                                                                                                   |
| --------------------- | :------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `assets`              |    ✓     | Provide the assets for image edit                                                                                                                                                                                                                                                                                                                                                                                                                            | `V1AiImageEditorCreateBodyAssets {image_file_path: Some("api-assets/id/1234.png".to_string()), image_file_paths: Some(vec!["api-assets/id/1234.png".to_string(), "api-assets/id/1235.png".to_string()])}` |
| `└─ image_file_path`  |    ✗     | Deprecated: Please use `image_file_paths` instead as edits with multiple images are now supported. The image used in the edit. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details. | `"api-assets/id/1234.png".to_string()`                                                                                                                                                                    |
| `└─ image_file_paths` |    ✗     | The image(s) used in the edit, maximum of 10 images. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.                                                                           | `vec!["api-assets/id/1234.png".to_string(), "api-assets/id/1235.png".to_string()]`                                                                                                                        |
| `style`               |    ✓     |                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `V1AiImageEditorCreateBodyStyle {model: Some(V1AiImageEditorCreateBodyStyleModelEnum::NanoBanana), prompt: "Give me sunglasses".to_string()}`                                                             |
| `└─ model`            |    ✗     | The AI model to use for image editing. * `Nano Banana` - Precise, realistic edits with consistent results * `Seedream` - Creative, imaginative images with artistic freedom * `default` - Use the model we recommend, which will change over time. This is recommended unless you need a specific model. This is the default behavior.                                                                                                                       | `V1AiImageEditorCreateBodyStyleModelEnum::NanoBanana`                                                                                                                                                     |
| `└─ prompt`           |    ✓     | The prompt used to edit the image.                                                                                                                                                                                                                                                                                                                                                                                                                           | `"Give me sunglasses".to_string()`                                                                                                                                                                        |
| `name`                |    ✗     | Give your image a custom name for easy identification.                                                                                                                                                                                                                                                                                                                                                                                                       | `"My Ai Image Editor image".to_string()`                                                                                                                                                                  |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_image_editor()
    .create(magic_hour::resources::v1::ai_image_editor::CreateRequest {
        assets: magic_hour::models::V1AiImageEditorCreateBodyAssets {
            image_file_path: Some("api-assets/id/1234.png".to_string()),
            image_file_paths: Some(
                vec![
                    "api-assets/id/1234.png".to_string(), "api-assets/id/1235.png"
                    .to_string()
                ],
            ),
        },
        name: Some("My Ai Image Editor image".to_string()),
        style: magic_hour::models::V1AiImageEditorCreateBodyStyle {
            model: Some(
                magic_hour::models::V1AiImageEditorCreateBodyStyleModelEnum::NanoBanana,
            ),
            prompt: "Give me sunglasses".to_string(),
        },
    })
    .await;
```

#### Response

##### Type

[V1AiImageEditorCreateResponse](/src/models/v1_ai_image_editor_create_response.rs)

##### Example

```rust
V1AiImageEditorCreateResponse {credits_charged: 10, frame_cost: 10, id: "cuid-example".to_string()}
```
