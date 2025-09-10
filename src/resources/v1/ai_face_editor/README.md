# v1.ai_face_editor

## Module Functions
### AI Face Editor <a name="create"></a>

Edit facial features of an image using AI. Each edit costs 1 frame. The height/width of the output image depends on your subscription. Please refer to our [pricing](/pricing) page for more details

**API Endpoint**: `POST /v1/ai-face-editor`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for face editor | `V1AiFaceEditorCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `└─ image_file_path` | ✓ | This is the image whose face will be edited. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.png".to_string()` |
| `style` | ✓ | Face editing parameters | `V1AiFaceEditorCreateBodyStyle {enhance_face: Some(false), eye_gaze_horizontal: Some(0.0), eye_gaze_vertical: Some(0.0), eye_open_ratio: Some(0.0), eyebrow_direction: Some(0.0), head_pitch: Some(0.0), head_roll: Some(0.0), head_yaw: Some(0.0), lip_open_ratio: Some(0.0), mouth_grim: Some(0.0), mouth_position_horizontal: Some(0.0), mouth_position_vertical: Some(0.0), mouth_pout: Some(0.0), mouth_purse: Some(0.0), mouth_smile: Some(0.0)}` |
| `└─ enhance_face` | ✗ | Enhance face features | `false` |
| `└─ eye_gaze_horizontal` | ✗ | Horizontal eye gaze (-100 to 100), in increments of 5 | `0.0` |
| `└─ eye_gaze_vertical` | ✗ | Vertical eye gaze (-100 to 100), in increments of 5 | `0.0` |
| `└─ eye_open_ratio` | ✗ | Eye open ratio (-100 to 100), in increments of 5 | `0.0` |
| `└─ eyebrow_direction` | ✗ | Eyebrow direction (-100 to 100), in increments of 5 | `0.0` |
| `└─ head_pitch` | ✗ | Head pitch (-100 to 100), in increments of 5 | `0.0` |
| `└─ head_roll` | ✗ | Head roll (-100 to 100), in increments of 5 | `0.0` |
| `└─ head_yaw` | ✗ | Head yaw (-100 to 100), in increments of 5 | `0.0` |
| `└─ lip_open_ratio` | ✗ | Lip open ratio (-100 to 100), in increments of 5 | `0.0` |
| `└─ mouth_grim` | ✗ | Mouth grim (-100 to 100), in increments of 5 | `0.0` |
| `└─ mouth_position_horizontal` | ✗ | Horizontal mouth position (-100 to 100), in increments of 5 | `0.0` |
| `└─ mouth_position_vertical` | ✗ | Vertical mouth position (-100 to 100), in increments of 5 | `0.0` |
| `└─ mouth_pout` | ✗ | Mouth pout (-100 to 100), in increments of 5 | `0.0` |
| `└─ mouth_purse` | ✗ | Mouth purse (-100 to 100), in increments of 5 | `0.0` |
| `└─ mouth_smile` | ✗ | Mouth smile (-100 to 100), in increments of 5 | `0.0` |
| `name` | ✗ | The name of image. This value is mainly used for your own identification of the image. | `"Face Editor image".to_string()` |

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
            enhance_face: Some(false),
            eye_gaze_horizontal: Some(0.0),
            eye_gaze_vertical: Some(0.0),
            eye_open_ratio: Some(0.0),
            eyebrow_direction: Some(0.0),
            head_pitch: Some(0.0),
            head_roll: Some(0.0),
            head_yaw: Some(0.0),
            lip_open_ratio: Some(0.0),
            mouth_grim: Some(0.0),
            mouth_position_horizontal: Some(0.0),
            mouth_position_vertical: Some(0.0),
            mouth_pout: Some(0.0),
            mouth_purse: Some(0.0),
            mouth_smile: Some(0.0),
        },
    })
    .await;
```

#### Response

##### Type
[V1AiFaceEditorCreateResponse](/src/models/v1_ai_face_editor_create_response.rs)

##### Example
`V1AiFaceEditorCreateResponse {credits_charged: 1, frame_cost: 1, id: "cuid-example".to_string()}`
<!-- CUSTOM DOCS START -->

<!-- CUSTOM DOCS END -->

