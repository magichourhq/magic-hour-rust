# v1.photo_colorizer

## Module Functions

### Photo Colorizer <a name="create"></a>

Colorize image. Each image costs 10 credits.

**API Endpoint**: `POST /v1/photo-colorizer`

#### Parameters

| Parameter            | Required | Description                                                                                                                                                                                                                                                                                                                                                                   | Example                                                                                    |
| -------------------- | :------: | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `assets`             |    ✓     | Provide the assets for photo colorization                                                                                                                                                                                                                                                                                                                                     | `V1PhotoColorizerCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `└─ image_file_path` |    ✓     | The image used to generate the colorized image. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details. | `"api-assets/id/1234.png".to_string()`                                                     |
| `name`               |    ✗     | Give your image a custom name for easy identification.                                                                                                                                                                                                                                                                                                                        | `"My Photo Colorizer image".to_string()`                                                   |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .photo_colorizer()
    .create(magic_hour::resources::v1::photo_colorizer::CreateRequest {
        assets: magic_hour::models::V1PhotoColorizerCreateBodyAssets {
            image_file_path: "api-assets/id/1234.png".to_string(),
        },
        name: Some("My Photo Colorizer image".to_string()),
    })
    .await;
```

#### Response

##### Type

[V1PhotoColorizerCreateResponse](/src/models/v1_photo_colorizer_create_response.rs)

##### Example

```rust
V1PhotoColorizerCreateResponse {credits_charged: 10, frame_cost: 10, id: "cuid-example".to_string()}
```
