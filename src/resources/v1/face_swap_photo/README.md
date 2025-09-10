# v1.face_swap_photo

## Module Functions
### Face Swap Photo <a name="create"></a>

Create a face swap photo. Each photo costs 5 credits. The height/width of the output image depends on your subscription. Please refer to our [pricing](https://magichour.ai/pricing) page for more details

**API Endpoint**: `POST /v1/face-swap-photo`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for face swap photo | `V1FaceSwapPhotoCreateBodyAssets {face_mappings: Some(vec![V1FaceSwapPhotoCreateBodyAssetsFaceMappingsItem {new_face: "api-assets/id/1234.png".to_string(), original_face: "api-assets/id/0-0.png".to_string()}]), face_swap_mode: Some(V1FaceSwapPhotoCreateBodyAssetsFaceSwapModeEnum::AllFaces), source_file_path: Some("api-assets/id/1234.png".to_string()), target_file_path: "api-assets/id/1234.png".to_string()}` |
| `└─ face_mappings` | ✗ | This is the array of face mappings used for multiple face swap. The value is required if `face_swap_mode` is `individual-faces`. | `vec![V1FaceSwapPhotoCreateBodyAssetsFaceMappingsItem {new_face: "api-assets/id/1234.png".to_string(), original_face: "api-assets/id/0-0.png".to_string()}]` |
| `└─ face_swap_mode` | ✗ | The mode of face swap. * `all-faces` - Swap all faces in the target image or video. `source_file_path` is required. * `individual-faces` - Swap individual faces in the target image or video. `source_faces` is required. | `V1FaceSwapPhotoCreateBodyAssetsFaceSwapModeEnum::AllFaces` |
| `└─ source_file_path` | ✗ | This is the image from which the face is extracted. The value is required if `face_swap_mode` is `all-faces`.  This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.png".to_string()` |
| `└─ target_file_path` | ✓ | This is the image where the face from the source image will be placed. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.png".to_string()` |
| `name` | ✗ | The name of image. This value is mainly used for your own identification of the image. | `"Face Swap image".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .face_swap_photo()
    .create(magic_hour::resources::v1::face_swap_photo::CreateRequest {
        assets: magic_hour::models::V1FaceSwapPhotoCreateBodyAssets {
            face_mappings: Some(
                vec![
                    magic_hour::models::V1FaceSwapPhotoCreateBodyAssetsFaceMappingsItem
                    { new_face : "api-assets/id/1234.png".to_string(), original_face
                    : "api-assets/id/0-0.png".to_string() }
                ],
            ),
            face_swap_mode: Some(
                magic_hour::models::V1FaceSwapPhotoCreateBodyAssetsFaceSwapModeEnum::AllFaces,
            ),
            source_file_path: Some("api-assets/id/1234.png".to_string()),
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
`V1FaceSwapPhotoCreateResponse {credits_charged: 5, frame_cost: 5, id: "cuid-example".to_string()}`
<!-- CUSTOM DOCS START -->

<!-- CUSTOM DOCS END -->

