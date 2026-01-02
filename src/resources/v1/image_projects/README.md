# v1.image_projects

## Module Functions

### Delete image <a name="delete"></a>

Permanently delete the rendered image(s). This action is not reversible, please be sure before deleting.

**API Endpoint**: `DELETE /v1/image-projects/{id}`

#### Parameters

| Parameter | Required | Description                                                                                          | Example                      |
| --------- | :------: | ---------------------------------------------------------------------------------------------------- | ---------------------------- |
| `id`      |    ✓     | Unique ID of the image project. This value is returned by all of the POST APIs that create an image. | `"cuid-example".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .image_projects()
    .delete(magic_hour::resources::v1::image_projects::DeleteRequest {
        id: "cuid-example".to_string(),
    })
    .await;
```

### Get image details <a name="get"></a>

Get the details of a image project. The `downloads` field will be empty unless the image was successfully rendered.

The image can be one of the following status

- `draft` - not currently used
- `queued` - the job is queued and waiting for a GPU
- `rendering` - the generation is in progress
- `complete` - the image is successful created
- `error` - an error occurred during rendering
- `canceled` - image render is canceled by the user

**API Endpoint**: `GET /v1/image-projects/{id}`

#### Parameters

| Parameter | Required | Description                                                                                          | Example                      |
| --------- | :------: | ---------------------------------------------------------------------------------------------------- | ---------------------------- |
| `id`      |    ✓     | Unique ID of the image project. This value is returned by all of the POST APIs that create an image. | `"cuid-example".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .image_projects()
    .get(magic_hour::resources::v1::image_projects::GetRequest {
        id: "cuid-example".to_string(),
    })
    .await;
```

#### Response

##### Type

[V1ImageProjectsGetResponse](/src/models/v1_image_projects_get_response.rs)

##### Example

```rust
V1ImageProjectsGetResponse {created_at: "1970-01-01T00:00:00".to_string(), credits_charged: 5, downloads: vec![V1ImageProjectsGetResponseDownloadsItem {expires_at: "2024-10-19T05:16:19.027Z".to_string(), url: "https://videos.magichour.ai/id/output.png".to_string()}], enabled: true, error: Some(V1ImageProjectsGetResponseError {code: "no_source_face".to_string(), message: "Please use an image with a detectable face".to_string()}), id: "cuid-example".to_string(), image_count: 1, name: Some("Example Name".to_string()), status: V1ImageProjectsGetResponseStatusEnum::Complete, total_frame_cost: 5, type_: "AI_IMAGE".to_string()}
```
