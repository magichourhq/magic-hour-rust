
### Get face detection details <a name="get"></a>

Get the details of a face detection task.

**API Endpoint**: `GET /v1/face-detection/{id}`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `id` | ✓ | The id of the task | `"string".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .face_detection()
    .get(magic_hour::resources::v1::face_detection::GetRequest {
        id: "string".to_string(),
    })
    .await;
```

#### Response

##### Type
[V1FaceDetectionGetResponse](/src/models/v1_face_detection_get_response.rs)

##### Example
`V1FaceDetectionGetResponse {credits_charged: 123, faces: vec![V1FaceDetectionGetResponseFacesItem {path: "api-assets/id/0-0.png".to_string(), url: "https://videos.magichour.ai/api-assets/id/0-0.png".to_string()}], id: "string".to_string(), status: V1FaceDetectionGetResponseStatusEnum::Complete}`

### Face Detection <a name="create"></a>

Detect faces in an image or video. 

Note: Face detection is free to use for the near future. Pricing may change in the future.

**API Endpoint**: `POST /v1/face-detection`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for face detection | `V1FaceDetectionCreateBodyAssets {target_file_path: "api-assets/id/1234.png".to_string()}` |
| `confidence_score` | ✗ | Confidence threshold for filtering detected faces.  * Higher values (e.g., 0.9) include only faces detected with high certainty, reducing false positives.  * Lower values (e.g., 0.3) include more faces, but may increase the chance of incorrect detections. | `0.5` |

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
`V1FaceDetectionCreateResponse {credits_charged: 123, id: "string".to_string()}`
