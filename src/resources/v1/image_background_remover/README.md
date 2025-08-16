
### Image Background Remover <a name="create"></a>

Remove background from image. Each image costs 5 credits.

**API Endpoint**: `POST /v1/image-background-remover`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for background removal | `V1ImageBackgroundRemoverCreateBodyAssets {background_image_file_path: Some("api-assets/id/1234.png".to_string()), image_file_path: "api-assets/id/1234.png".to_string()}` |
| `name` | ✗ | The name of image. This value is mainly used for your own identification of the image. | `"Background Remover image".to_string()` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .image_background_remover()
    .create(magic_hour::resources::v1::image_background_remover::CreateRequest {
        assets: magic_hour::models::V1ImageBackgroundRemoverCreateBodyAssets {
            background_image_file_path: Some("api-assets/id/1234.png".to_string()),
            image_file_path: "api-assets/id/1234.png".to_string(),
        },
        name: Some("Background Remover image".to_string()),
    })
    .await;
```

#### Response

##### Type
[V1ImageBackgroundRemoverCreateResponse](/src/models/v1_image_background_remover_create_response.rs)

##### Example
`V1ImageBackgroundRemoverCreateResponse {credits_charged: 5, frame_cost: 5, id: "cuid-example".to_string()}`
