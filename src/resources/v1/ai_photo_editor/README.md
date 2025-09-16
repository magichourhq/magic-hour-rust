# v1.ai_photo_editor

## Module Functions
### AI Photo Editor <a name="create"></a>

> **NOTE**: this API is still in early development stages, and should be avoided. Please reach out to us if you're interested in this API. 

Edit photo using AI. Each photo costs 10 credits.

**API Endpoint**: `POST /v1/ai-photo-editor`

#### Parameters

| Parameter | Required | Deprecated | Description | Example |
|-----------|:--------:|:----------:|-------------|--------|
| `assets` | ✓ | ✗ | Provide the assets for photo editor | `V1AiPhotoEditorCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `└─ image_file_path` | ✓ | — | The image used to generate the output. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.png".to_string()` |
| `resolution` | ✓ | ✗ | The resolution of the final output image. The allowed value is based on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details | `768` |
| `style` | ✓ | ✗ |  | `V1AiPhotoEditorCreateBodyStyle {image_description: "A photo of a person".to_string(), likeness_strength: 5.2, negative_prompt: Some("painting, cartoon, sketch".to_string()), prompt: "A photo portrait of a person wearing a hat".to_string(), prompt_strength: 3.75, steps: Some(4), upscale_factor: Some(2), upscale_fidelity: Some(0.5)}` |
| `└─ image_description` | ✓ | — | Use this to describe what your input image is. This helps maintain aspects of the image you don't want to change. | `"A photo of a person".to_string()` |
| `└─ likeness_strength` | ✓ | — | Determines the input image's influence. Higher values align the output more with the initial image. | `5.2` |
| `└─ negative_prompt` | ✗ | — | What you want to avoid seeing in the final output; has a minor effect. | `"painting, cartoon, sketch".to_string()` |
| `└─ prompt` | ✓ | — | What you want your final output to look like. We recommend starting with the image description and making minor edits for best results. | `"A photo portrait of a person wearing a hat".to_string()` |
| `└─ prompt_strength` | ✓ | — | Determines the prompt's influence. Higher values align the output more with the prompt. | `3.75` |
| `└─ steps` | ✗ | — | Number of iterations used to generate the output. Higher values improve quality and increase the strength of the prompt but increase processing time. | `4` |
| `└─ upscale_factor` | ✗ | — | The multiplier applied to an image's original dimensions during the upscaling process. For example, a scale of 2 doubles the width and height (e.g., from 512x512 to 1024x1024). | `2` |
| `└─ upscale_fidelity` | ✗ | — | Upscale fidelity refers to the level of quality desired in the generated image. Fidelity value of 1 means more details. | `0.5` |
| `name` | ✗ | ✗ | The name of image. This value is mainly used for your own identification of the image. | `"Photo Editor image".to_string()` |
| `steps` | ✗ | ✓ | Deprecated: Please use `.style.steps` instead. Number of iterations used to generate the output. Higher values improve quality and increase the strength of the prompt but increase processing time. | `123` |

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
`V1AiPhotoEditorCreateResponse {credits_charged: 10, frame_cost: 10, id: "cuid-example".to_string()}`
<!-- CUSTOM DOCS START -->

<!-- CUSTOM DOCS END -->

