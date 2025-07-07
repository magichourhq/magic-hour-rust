
### Face Swap Photo <a name="create"></a>

Create a face swap photo. Each photo costs 5 credits. The height/width of the output image depends on your subscription. Please refer to our [pricing](/pricing) page for more details

**API Endpoint**: `POST /v1/face-swap-photo`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for face swap photo | `V1FaceSwapPhotoCreateBodyAssets {source_file_path: "api-assets/id/1234.png".to_string(), target_file_path: "api-assets/id/1234.png".to_string()}` |
| `name` | ✗ | The name of image | `"Face Swap image".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .face_swap_photo()
    .create(magic_hour::resources::v1::face_swap_photo::CreateRequest {
        assets: magic_hour::models::V1FaceSwapPhotoCreateBodyAssets {
            source_file_path: "api-assets/id/1234.png".to_string(),
            target_file_path: "api-assets/id/1234.png".to_string(),
        },
        name: Some("Face Swap image".to_string()),
    })
    .await;
```

#### Response

##### Type
[V1FaceSwapPhotoCreateResponse](/src/models/v1_face_swap_photo_create_response.rs)

##### Example
`V1FaceSwapPhotoCreateResponse {credits_charged: 5, frame_cost: 5, id: "clx7uu86w0a5qp55yxz315r6r".to_string()}`
