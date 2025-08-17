
### Face Swap video <a name="create"></a>

Create a Face Swap video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
  
Get more information about this mode at our [product page](https://magichour.ai/products/face-swap).
  

**API Endpoint**: `POST /v1/face-swap`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for face swap. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used | `V1FaceSwapCreateBodyAssets {face_mappings: Some(vec![V1FaceSwapCreateBodyAssetsFaceMappingsItem {new_face: "api-assets/id/1234.png".to_string(), original_face: "api-assets/id/0-0.png".to_string()}]), face_swap_mode: Some(V1FaceSwapCreateBodyAssetsFaceSwapModeEnum::AllFaces), image_file_path: Some("image/id/1234.png".to_string()), video_file_path: Some("api-assets/id/1234.mp4".to_string()), video_source: V1FaceSwapCreateBodyAssetsVideoSourceEnum::File, ..Default::default()}` |
| `end_seconds` | ✓ | The end time of the input video in seconds. This value is used to trim the input video. The value must be greater than 0.1, and more than the start_seconds. | `15.0` |
| `start_seconds` | ✓ | The start time of the input video in seconds. This value is used to trim the input video. The value must be greater than 0. | `0.0` |
| `height` | ✗ | `height` is deprecated and no longer influences the output video's resolution.  Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details.  This field is retained only for backward compatibility and will be removed in a future release. | `123` |
| `name` | ✗ | The name of video. This value is mainly used for your own identification of the video. | `"Face Swap video".to_string()` |
| `width` | ✗ | `width` is deprecated and no longer influences the output video's resolution.  Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details.  This field is retained only for backward compatibility and will be removed in a future release. | `123` |

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
        ..Default::default()
    })
    .await;
```

#### Response

##### Type
[V1FaceSwapCreateResponse](/src/models/v1_face_swap_create_response.rs)

##### Example
`V1FaceSwapCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}`
