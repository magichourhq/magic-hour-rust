# v1.video_projects

## Module Functions
### Delete video <a name="delete"></a>

Permanently delete the rendered video. This action is not reversible, please be sure before deleting.

**API Endpoint**: `DELETE /v1/video-projects/{id}`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `id` | ✓ | Unique ID of the video project. This value is returned by all of the POST APIs that create a video. | `"cuid-example".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .video_projects()
    .delete(magic_hour::resources::v1::video_projects::DeleteRequest {
        id: "cuid-example".to_string(),
    })
    .await;
```

### Get video details <a name="get"></a>

Get the details of a video project. The `downloads` field will be empty unless the video was successfully rendered.

The video can be one of the following status
- `draft` - not currently used
- `queued` - the job is queued and waiting for a GPU
- `rendering` - the generation is in progress
- `complete` - the video is successful created
- `error` - an error occurred during rendering
- `canceled` - video render is canceled by the user


**API Endpoint**: `GET /v1/video-projects/{id}`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `id` | ✓ | Unique ID of the video project. This value is returned by all of the POST APIs that create a video. | `"cuid-example".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .video_projects()
    .get(magic_hour::resources::v1::video_projects::GetRequest {
        id: "cuid-example".to_string(),
    })
    .await;
```

#### Response

##### Type
[V1VideoProjectsGetResponse](/src/models/v1_video_projects_get_response.rs)

##### Example
`V1VideoProjectsGetResponse {created_at: "1970-01-01T00:00:00".to_string(), credits_charged: 450, download: Some(V1VideoProjectsGetResponseDownload {expires_at: "2024-10-19T05:16:19.027Z".to_string(), url: "https://videos.magichour.ai/id/output.mp4".to_string()}), downloads: vec![V1VideoProjectsGetResponseDownloadsItem {expires_at: "2024-10-19T05:16:19.027Z".to_string(), url: "https://videos.magichour.ai/id/output.mp4".to_string()}], enabled: true, end_seconds: 15.0, error: Some(V1VideoProjectsGetResponseError {code: "no_source_face".to_string(), message: "Please use an image with a detectable face".to_string()}), fps: 30.0, height: 960, id: "cuid-example".to_string(), name: Some("Example Name".to_string()), start_seconds: 0.0, status: V1VideoProjectsGetResponseStatusEnum::Complete, total_frame_cost: 450, type_: "FACE_SWAP".to_string(), width: 512}`
<!-- CUSTOM DOCS START -->

<!-- CUSTOM DOCS END -->

