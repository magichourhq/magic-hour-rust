# v1.face_swap

## Module Functions

### Face Swap Video <a name="create"></a>

**What this API does**

Create the same Face Swap you can make in the browser, but programmatically, so you can automate it, run it at scale, or connect it to your own app or workflow.

**Good for**

- Automation and batch processing
- Adding face swap into apps, pipelines, or tools

**How it works (3 steps)**

1. Upload your inputs (video, image, or audio) with [Generate Upload URLs](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls) and copy the `file_path`.
2. Send a request to create a face swap job with the basic fields.
3. Check the job status until it's `complete`, then download the result from `downloads`.

**Key options**

- Inputs: usually a file, sometimes a YouTube link, depending on project type
- Resolution: free users are limited to 576px; higher plans unlock HD and larger sizes
- Extra fields: e.g. `face_swap_mode`, `start_seconds`/`end_seconds`, or a text prompt

**Cost**\
Credits are only charged for the frames that actually render. You'll see an estimate when the job is queued, and the final total after it's done.

For detailed examples, see the [product page](https://magichour.ai/products/face-swap).

**API Endpoint**: `POST /v1/face-swap`

#### Parameters

| Parameter            | Required | Deprecated | Description                                                                                                                                                                                                                                                                                                                                                                                                                                      | Example                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| -------------------- | :------: | :--------: | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `assets`             |    ✓     |     ✗      | Provide the assets for face swap. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used                                                                                                                                                                                                                                                                                                        | `V1FaceSwapCreateBodyAssets {face_mappings: Some(vec![V1FaceSwapCreateBodyAssetsFaceMappingsItem {new_face: "api-assets/id/1234.png".to_string(), original_face: "api-assets/id/0-0.png".to_string()}]), face_swap_mode: Some(V1FaceSwapCreateBodyAssetsFaceSwapModeEnum::AllFaces), image_file_path: Some("image/id/1234.png".to_string()), video_file_path: Some("api-assets/id/1234.mp4".to_string()), video_source: V1FaceSwapCreateBodyAssetsVideoSourceEnum::File, ..Default::default()}` |
| `└─ face_mappings`   |    ✗     |     —      | This is the array of face mappings used for multiple face swap. The value is required if `face_swap_mode` is `individual-faces`.                                                                                                                                                                                                                                                                                                                 | `vec![V1FaceSwapCreateBodyAssetsFaceMappingsItem {new_face: "api-assets/id/1234.png".to_string(), original_face: "api-assets/id/0-0.png".to_string()}]`                                                                                                                                                                                                                                                                                                                                         |
| `└─ face_swap_mode`  |    ✗     |     —      | Choose how to swap faces: **all-faces** (recommended) — swap all detected faces using one source image (`source_file_path` required) +- **individual-faces** — specify exact mappings using `face_mappings`                                                                                                                                                                                                                                      | `V1FaceSwapCreateBodyAssetsFaceSwapModeEnum::AllFaces`                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `└─ image_file_path` |    ✗     |     —      | The path of the input image with the face to be swapped. The value is required if `face_swap_mode` is `all-faces`. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details. | `"image/id/1234.png".to_string()`                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `└─ video_file_path` |    ✗     |     —      | Your video file. Required if `video_source` is `file`. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.                                                             | `"api-assets/id/1234.mp4".to_string()`                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `└─ video_source`    |    ✓     |     —      | Choose your video source.                                                                                                                                                                                                                                                                                                                                                                                                                        | `V1FaceSwapCreateBodyAssetsVideoSourceEnum::File`                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `└─ youtube_url`     |    ✗     |     —      | YouTube URL (required if `video_source` is `youtube`).                                                                                                                                                                                                                                                                                                                                                                                           | `"http://www.example.com".to_string()`                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `end_seconds`        |    ✓     |     ✗      | End time of your clip (seconds). Must be greater than start_seconds.                                                                                                                                                                                                                                                                                                                                                                             | `15.0`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `start_seconds`      |    ✓     |     ✗      | Start time of your clip (seconds). Must be ≥ 0.                                                                                                                                                                                                                                                                                                                                                                                                  | `0.0`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `height`             |    ✗     |     ✓      | `height` is deprecated and no longer influences the output video's resolution. Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details. This field is retained only for backward compatibility and will be removed in a future release.                                     | `123`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `name`               |    ✗     |     ✗      | Give your video a custom name for easy identification.                                                                                                                                                                                                                                                                                                                                                                                           | `"My Face Swap video".to_string()`                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `style`              |    ✗     |     ✗      | Style of the face swap video.                                                                                                                                                                                                                                                                                                                                                                                                                    | `V1FaceSwapCreateBodyStyle {version: Some(V1FaceSwapCreateBodyStyleVersionEnum::Default)}`                                                                                                                                                                                                                                                                                                                                                                                                      |
| `└─ version`         |    ✗     |     —      | * `v1` - May preserve skin detail and texture better, but weaker identity preservation. * `v2` - Faster, sharper, better handling of hair and glasses. stronger identity preservation. * `default` - Use the version we recommend, which will change over time. This is recommended unless you need a specific earlier version. This is the default behavior.                                                                                    | `V1FaceSwapCreateBodyStyleVersionEnum::Default`                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `width`              |    ✗     |     ✓      | `width` is deprecated and no longer influences the output video's resolution. Output resolution is determined by the **minimum** of: - The resolution of the input video - The maximum resolution allowed by your subscription tier. See our [pricing page](https://magichour.ai/pricing) for more details. This field is retained only for backward compatibility and will be removed in a future release.                                      | `123`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |

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
        name: Some("My Face Swap video".to_string()),
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

```rust
V1FaceSwapCreateResponse {credits_charged: 450, estimated_frame_cost: 450, id: "cuid-example".to_string()}
```
