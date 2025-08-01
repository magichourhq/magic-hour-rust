
### AI Face Editor <a name="create"></a>

Edit facial features of an image using AI. Each edit costs 1 frame. The height/width of the output image depends on your subscription. Please refer to our [pricing](/pricing) page for more details

**API Endpoint**: `POST /v1/ai-face-editor`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for face editor | `V1AiFaceEditorCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `style` | ✓ | Face editing parameters | `V1AiFaceEditorCreateBodyStyle {enhance_face: false, eye_gaze_horizontal: 0.0, eye_gaze_vertical: 0.0, eye_open_ratio: 0.0, eyebrow_direction: 0.0, head_pitch: 0.0, head_roll: 0.0, head_yaw: 0.0, lip_open_ratio: 0.0, mouth_grim: 0.0, mouth_position_horizontal: 0.0, mouth_position_vertical: 0.0, mouth_pout: 0.0, mouth_purse: 0.0, mouth_smile: 0.0}` |
| `name` | ✗ | The name of image | `"Face Editor image".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_face_editor()
    .create(magic_hour::resources::v1::ai_face_editor::CreateRequest {
        assets: magic_hour::models::V1AiFaceEditorCreateBodyAssets {
            image_file_path: "api-assets/id/1234.png".to_string(),
        },
        name: Some("Face Editor image".to_string()),
        style: magic_hour::models::V1AiFaceEditorCreateBodyStyle {
            enhance_face: false,
            eye_gaze_horizontal: 0.0,
            eye_gaze_vertical: 0.0,
            eye_open_ratio: 0.0,
            eyebrow_direction: 0.0,
            head_pitch: 0.0,
            head_roll: 0.0,
            head_yaw: 0.0,
            lip_open_ratio: 0.0,
            mouth_grim: 0.0,
            mouth_position_horizontal: 0.0,
            mouth_position_vertical: 0.0,
            mouth_pout: 0.0,
            mouth_purse: 0.0,
            mouth_smile: 0.0,
        },
    })
    .await;
```

#### Response

##### Type
[V1AiFaceEditorCreateResponse](/src/models/v1_ai_face_editor_create_response.rs)

##### Example
`V1AiFaceEditorCreateResponse {credits_charged: 1, frame_cost: 1, id: "clx7uu86w0a5qp55yxz315r6r".to_string()}`
