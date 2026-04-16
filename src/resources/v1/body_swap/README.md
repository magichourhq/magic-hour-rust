# v1.body_swap

## Module Functions

### Body Swap <a name="create"></a>

Swap a person into a scene image using Nano Banana 2. Credits depend on `resolution` (from 100 credits at 640px upward).

**API Endpoint**: `POST /v1/body-swap`

#### Parameters

| Parameter             | Required | Description                                                                                                                                                                                                                                                                                                                                                                | Example                                                                                                                                      |
| --------------------- | :------: | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| `assets`              |    ✓     | Person image and scene image for body swap                                                                                                                                                                                                                                                                                                                                 | `V1BodySwapCreateBodyAssets {person_file_path: "api-assets/id/1234.png".to_string(), scene_file_path: "api-assets/id/5678.png".to_string()}` |
| `└─ person_file_path` |    ✓     | Image of the person to place into the scene. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details. | `"api-assets/id/1234.png".to_string()`                                                                                                       |
| `└─ scene_file_path`  |    ✓     | Target scene image (background). This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.             | `"api-assets/id/5678.png".to_string()`                                                                                                       |
| `resolution`          |    ✓     | Output resolution. Determines credits charged for the run.                                                                                                                                                                                                                                                                                                                 | `V1BodySwapCreateBodyResolutionEnum::Enum1k`                                                                                                 |
| `name`                |    ✗     | Give your image a custom name for easy identification.                                                                                                                                                                                                                                                                                                                     | `"My Body Swap image".to_string()`                                                                                                           |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .body_swap()
    .create(magic_hour::resources::v1::body_swap::CreateRequest {
        assets: magic_hour::models::V1BodySwapCreateBodyAssets {
            person_file_path: "api-assets/id/1234.png".to_string(),
            scene_file_path: "api-assets/id/5678.png".to_string(),
        },
        name: Some("My Body Swap image".to_string()),
        resolution: magic_hour::models::V1BodySwapCreateBodyResolutionEnum::Enum1k,
    })
    .await;
```

#### Response

##### Type

[V1BodySwapCreateResponse](/src/models/v1_body_swap_create_response.rs)

##### Example

```rust
V1BodySwapCreateResponse {credits_charged: 100, frame_cost: 100, id: "cuid-example".to_string()}
```
