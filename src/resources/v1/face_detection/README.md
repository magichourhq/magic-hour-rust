# v1.face_detection

## Module Functions

### Get face detection details <a name="get"></a>

Get the details of a face detection task.

Use this API to get the list of faces detected in the image or video to use in the [face swap photo](/api-reference/face-swap-photo/face-swap-photo) or [face swap video](/api-reference/face-swap/face-swap-video) API calls for multi-face swaps.

**API Endpoint**: `GET /v1/face-detection/{id}`

#### Parameters

| Parameter | Required | Description                                                                                                              | Example                      |
| --------- | :------: | ------------------------------------------------------------------------------------------------------------------------ | ---------------------------- |
| `id`      |    ✓     | The id of the task. This value is returned by the [face detection API](/api-reference/files/face-detection#response-id). | `"uuid-example".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .face_detection()
    .get(magic_hour::resources::v1::face_detection::GetRequest {
        id: "uuid-example".to_string(),
    })
    .await;
```

#### Response

##### Type

[V1FaceDetectionGetResponse](/src/models/v1_face_detection_get_response.rs)

##### Example

```rust
V1FaceDetectionGetResponse {credits_charged: 0, faces: vec![V1FaceDetectionGetResponseFacesItem {path: "api-assets/id/0-0.png".to_string(), url: "https://videos.magichour.ai/api-assets/id/0-0.png".to_string()}], id: "uuid-example".to_string(), status: V1FaceDetectionGetResponseStatusEnum::Complete}
```

### Face Detection <a name="create"></a>

Detect faces in an image or video.

Use this API to get the list of faces detected in the image or video to use in the [face swap photo](/api-reference/face-swap-photo/face-swap-photo) or [face swap video](/api-reference/face-swap/face-swap-video) API calls for multi-face swaps.

Note: Face detection is free to use for the near future. Pricing may change in the future.

**API Endpoint**: `POST /v1/face-detection`

#### Parameters

| Parameter             | Required | Description                                                                                                                                                                                                                                                                                                                                                                                                    | Example                                                                                    |
| --------------------- | :------: | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `assets`              |    ✓     | Provide the assets for face detection                                                                                                                                                                                                                                                                                                                                                                          | `V1FaceDetectionCreateBodyAssets {target_file_path: "api-assets/id/1234.png".to_string()}` |
| `└─ target_file_path` |    ✓     | This is the image or video where the face will be detected. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more. | `"api-assets/id/1234.png".to_string()`                                                     |
| `confidence_score`    |    ✗     | Confidence threshold for filtering detected faces. * Higher values (e.g., 0.9) include only faces detected with high certainty, reducing false positives. * Lower values (e.g., 0.3) include more faces, but may increase the chance of incorrect detections.                                                                                                                                                  | `0.5`                                                                                      |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .face_detection()
    .create(magic_hour::resources::v1::face_detection::CreateRequest {
        assets: magic_hour::models::V1FaceDetectionCreateBodyAssets {
            target_file_path: "api-assets/id/1234.png".to_string(),
        },
        confidence_score: Some(0.5),
    })
    .await;
```

#### Response

##### Type

[V1FaceDetectionCreateResponse](/src/models/v1_face_detection_create_response.rs)

##### Example

```rust
V1FaceDetectionCreateResponse {credits_charged: 123, id: "uuid-example".to_string()}
```
