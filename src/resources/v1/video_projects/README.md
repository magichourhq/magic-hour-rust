
### Delete video <a name="delete"></a>

Permanently delete the rendered video. This action is not reversible, please be sure before deleting.

**API Endpoint**: `DELETE /v1/video-projects/{id}`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .video_projects()
    .delete(magic_hour::resources::v1::video_projects::DeleteRequest {
        id: "cm6pvghix03bvyz0zwash6noj".to_string(),
    })
    .await;
```

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `id` | ✓ | The id of the video project | `"cm6pvghix03bvyz0zwash6noj".to_string()` |

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

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .video_projects()
    .get(magic_hour::resources::v1::video_projects::GetRequest {
        id: "cm6pvghix03bvyz0zwash6noj".to_string(),
    })
    .await;
```

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `id` | ✓ | The id of the video | `"cm6pvghix03bvyz0zwash6noj".to_string()` |
