# v1.photo_colorizer

## Module Functions

### Photo Colorizer <a name="create"></a>

Colorize image. Each image costs 5 credits.

**API Endpoint**: `POST /v1/photo-colorizer`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for photo colorization | `V1PhotoColorizerCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `└─ image_file_path` | ✓ | The image used to generate the colorized image. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).  Please refer to the [Input File documentation](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) to learn more.  | `"api-assets/id/1234.png".to_string()` |
| `name` | ✗ | The name of image. This value is mainly used for your own identification of the image. | `"Photo Colorizer image".to_string()` |

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
        name: Some("Photo Colorizer image".to_string()),
    })
    .await;
```

#### Response

##### Type
[V1PhotoColorizerCreateResponse](/src/models/v1_photo_colorizer_create_response.rs)

##### Example
`V1PhotoColorizerCreateResponse {credits_charged: 5, frame_cost: 5, id: "cuid-example".to_string()}`

