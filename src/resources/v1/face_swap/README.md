
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
| `height` | ✗ | Used to determine the dimensions of the output video.     * If height is provided, width will also be required. The larger value between width and height will be used to determine the maximum output resolution while maintaining the original aspect ratio. * If both height and width are omitted, the video will be resized according to your subscription's maximum resolution, while preserving aspect ratio.  Note: if the video's original resolution is less than the maximum, the video will not be resized.  See our [pricing page](https://magichour.ai/pricing) for more details. | `960` |
| `name` | ✗ | The name of video. This value is mainly used for your own identification of the video. | `"Face Swap video".to_string()` |
| `width` | ✗ | Used to determine the dimensions of the output video.     * If width is provided, height will also be required. The larger value between width and height will be used to determine the maximum output resolution while maintaining the original aspect ratio. * If both height and width are omitted, the video will be resized according to your subscription's maximum resolution, while preserving aspect ratio.  Note: if the video's original resolution is less than the maximum, the video will not be resized.  See our [pricing page](https://magichour.ai/pricing) for more details. | `512` |

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
        height: Some(960),
        name: Some("Face Swap video".to_string()),
        start_seconds: 0.0,
        width: Some(512),
    })
    .await;
```

#### Response

##### Type
[V1FaceSwapCreateResponse](/src/models/v1_face_swap_create_response.rs)

##### Example
`V1FaceSwapCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}`
