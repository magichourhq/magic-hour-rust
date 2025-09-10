# v1.face_swap

## Module Functions
### Face Swap video <a name="create"></a>

Create a Face Swap video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
  
Get more information about this mode at our [product page](https://magichour.ai/products/face-swap).
  

**API Endpoint**: `POST /v1/face-swap`

#### Parameters

| Parameter | Required | Deprecated | Description | Example |
|-----------|:--------:|:----------:|-------------|--------|
| `assets` | ✓ | ✗ | Provide the assets for face swap. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used | `V1FaceSwapCreateBodyAssets {face_mappings: Some(vec![V1FaceSwapCreateBodyAssetsFaceMappingsItem {new_face: "api-assets/id/1234.png".to_string(), original_face: "api-assets/id/0-0.png".to_string()}]), face_swap_mode: Some(V1FaceSwapCreateBodyAssetsFaceSwapModeEnum::AllFaces), image_file_path: Some("image/id/1234.png".to_string()), video_file_path: Some("api-assets/id/1234.mp4".to_string()), video_source: V1FaceSwapCreateBodyAssetsVideoSourceEnum::File, ..Default::default()}` |
| `└─ face_mappings` | ✗ | — | This is the array of face mappings used for multiple face swap. The value is required if `face_swap_mode` is `individual-faces`. | `vec![V1FaceSwapCreateBodyAssetsFaceMappingsItem {new_face: "api-assets/id/1234.png".to_string(), original_face: "api-assets/id/0-0.png".to_string()}]` |
| `└─ face_swap_mode` | ✗ | — | The mode of face swap. * `all-faces` - Swap all faces in the target image or video. `source_file_path` is required. * `individual-faces` - Swap individual faces in the target image or video. `source_faces` is required. | `V1FaceSwapCreateBodyAssetsFaceSwapModeEnum::AllFaces` |
| `└─ image_file_path` | ✗ | — | The path of the input image with the face to be swapped.  The value is required if `face_swap_mode` is `all-faces`.  This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"image/id/1234.png".to_string()` |
| `└─ video_file_path` | ✗ | — | Required if `video_source` is `file`. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.mp4".to_string()` |
| `└─ video_source` | ✓ | — |  | `V1FaceSwapCreateBodyAssetsVideoSourceEnum::File` |
| `└─ youtube_url` | ✗ | — | Using a youtube video as the input source. This field is required if `video_source` is `youtube` | `"http://www.example.com".to_string()` |
| `end_seconds` | ✓ | ✗ | The end time of the input video in seconds. This value is used to trim the input video. The value must be greater than 0.1, and more than the start_seconds. | `15.0` |
| `start_seconds` | ✓ | ✗ | The start time of the input video in seconds. This value is used to trim the input video. The value must be greater than 0. | `0.0` |
| `height` | ✗ | ✓ | `height` is deprecated and no longer influences the output video's resolution.  Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details.  This field is retained only for backward compatibility and will be removed in a future release. | `123` |
| `name` | ✗ | ✗ | The name of video. This value is mainly used for your own identification of the video. | `"Face Swap video".to_string()` |
| `style` | ✗ | ✗ | Style of the face swap video. | `V1FaceSwapCreateBodyStyle {version: Some(V1FaceSwapCreateBodyStyleVersionEnum::Default)}` |
| `└─ version` | ✗ | — | * `v1` - May preserve skin detail and texture better, but weaker identity preservation. * `v2` - Faster, sharper, better handling of hair and glasses. stronger identity preservation. (Recommended) * `default` - Use the version we recommend, which will change over time. This is recommended unless you need a specific earlier version. This is the default behavior. | `V1FaceSwapCreateBodyStyleVersionEnum::Default` |
| `width` | ✗ | ✓ | `width` is deprecated and no longer influences the output video's resolution.  Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details.  This field is retained only for backward compatibility and will be removed in a future release. | `123` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .face_swap()
    .create(magic_hour::resources::v1::face_swap::CreateRequest {
        assets: magic_hour::models::V1FaceSwapCreateBodyAssets {
            face_mappings: Some(
                vec![
                    magic_hour::models::V1FaceSwapCreateBodyAssetsFaceMappingsItem {
                    new_face : "api-assets/id/1234.png".to_string(), original_face :
                    "api-assets/id/0-0.png".to_string() }
                ],
            ),
            face_swap_mode: Some(
                magic_hour::models::V1FaceSwapCreateBodyAssetsFaceSwapModeEnum::AllFaces,
            ),
            image_file_path: Some("image/id/1234.png".to_string()),
            video_file_path: Some("api-assets/id/1234.mp4".to_string()),
            video_source: magic_hour::models::V1FaceSwapCreateBodyAssetsVideoSourceEnum::File,
            ..Default::default()
        },
        end_seconds: 15.0,
        name: Some("Face Swap video".to_string()),
        start_seconds: 0.0,
        style: Some(magic_hour::models::V1FaceSwapCreateBodyStyle {
            version: Some(
                magic_hour::models::V1FaceSwapCreateBodyStyleVersionEnum::Default,
            ),
        }),
        ..Default::default()
    })
    .await;
```

#### Response

##### Type
[V1FaceSwapCreateResponse](/src/models/v1_face_swap_create_response.rs)

##### Example
`V1FaceSwapCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}`
<!-- CUSTOM DOCS START -->

<!-- CUSTOM DOCS END -->

