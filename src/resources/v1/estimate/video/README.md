# v1.estimate.video

## Module Functions

### Estimate video credits <a name="list"></a>

Get the credits a video create request charges when it is submitted. Pass `tool`, then the fields below that the tool's create request uses, with the same values. Tools billed per frame charge an estimate at submission, assuming 30 fps for an uploaded video, and settle to the frames that render. Does not create a project or spend credits. It prices the fields passed here. It does not check your plan, your balance or the rest of the create request.

**API Endpoint**: `GET /v1/estimate/video`

#### Parameters

| Parameter             | Required | Description                                                                                                                                                                                                                                     | Example                                     |
| --------------------- | :------: | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- |
| `end_seconds`         |    ✓     | The `end_seconds` of the create request. Used by every tool.                                                                                                                                                                                    | `5.0`                                       |
| `tool`                |    ✓     | The create endpoint to price, named by its path. For example `image-to-video` for `POST /v1/image-to-video`.                                                                                                                                    | `V1EstimateVideoListToolEnum::ImageToVideo` |
| `audio`               |    ✗     | The `audio` of the create request. Used by `image-to-video`, `text-to-video`. Defaults to `false`.                                                                                                                                              | `true`                                      |
| `has_video_reference` |    ✗     | Whether any of the create request's `references` is a video. Used by `text-to-video`. Defaults to `false`.                                                                                                                                      | `true`                                      |
| `model`               |    ✗     | The `model` of the create request. Used by `image-to-video`, `text-to-video`, `ai-ugc-ad`, `ai-video-editor`, `character-replace`.                                                                                                              | `"kling-3.0".to_string()`                   |
| `resolution`          |    ✗     | The `resolution` of the create request. Used by `image-to-video`, `text-to-video`, `ai-ugc-ad`, `ai-video-editor`, `character-replace`, `audio-to-video`, `ai-video-translator`, `ai-video-upscaler`. `ai-video-translator` defaults to `480p`. | `"720p".to_string()`                        |
| `start_seconds`       |    ✗     | The `start_seconds` of the create request, for tools whose create request has it. Defaults to 0.                                                                                                                                                | `123.0`                                     |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .estimate()
    .video()
    .list(magic_hour::resources::v1::estimate::video::ListRequest {
        end_seconds: 5.0,
        tool: magic_hour::models::V1EstimateVideoListToolEnum::ImageToVideo,
        ..Default::default()
    })
    .await;
```

#### Response

##### Type

[V1EstimateVideoListResponse](/src/models/v1_estimate_video_list_response.rs)

##### Example

```rust
V1EstimateVideoListResponse {estimated_credits: 150}
```
