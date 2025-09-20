# v1.audio_projects

## Module Functions

### Delete audio <a name="delete"></a>

Permanently delete the rendered audio file(s). This action is not reversible, please be sure before deleting.

**API Endpoint**: `DELETE /v1/audio-projects/{id}`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `id` | ✓ | Unique ID of the audio project. This value is returned by all of the POST APIs that create an audio. | `"cuid-example".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .audio_projects()
    .delete(magic_hour::resources::v1::audio_projects::DeleteRequest {
        id: "cuid-example".to_string(),
    })
    .await;
```

### Get audio details <a name="get"></a>

Get the details of a audio project. The `downloads` field will be empty unless the audio was successfully rendered.

The audio can be one of the following status
- `draft` - not currently used
- `queued` - the job is queued and waiting for a GPU
- `rendering` - the generation is in progress
- `complete` - the audio is successful created
- `error` - an error occurred during rendering
- `canceled` - audio render is canceled by the user


**API Endpoint**: `GET /v1/audio-projects/{id}`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `id` | ✓ | Unique ID of the audio project. This value is returned by all of the POST APIs that create an audio. | `"cuid-example".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .audio_projects()
    .get(magic_hour::resources::v1::audio_projects::GetRequest {
        id: "cuid-example".to_string(),
    })
    .await;
```

#### Response

##### Type
[V1AudioProjectsGetResponse](/src/models/v1_audio_projects_get_response.rs)

##### Example
`V1AudioProjectsGetResponse {created_at: "1970-01-01T00:00:00".to_string(), credits_charged: 2, downloads: vec![V1AudioProjectsGetResponseDownloadsItem {expires_at: "2024-10-19T05:16:19.027Z".to_string(), url: "https://videos.magichour.ai/id/output.wav".to_string()}], enabled: true, error: Some(V1AudioProjectsGetResponseError {code: "no_source_face".to_string(), message: "Please use an image with a detectable face".to_string()}), id: "cuid-example".to_string(), name: Some("Example Name".to_string()), status: V1AudioProjectsGetResponseStatusEnum::Complete, type_: "VOICE_GENERATOR".to_string()}`


