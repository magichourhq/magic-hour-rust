
### Photo Colorizer <a name="create"></a>

Colorize image. Each image costs 5 credits.

**API Endpoint**: `POST /v1/photo-colorizer`

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

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for photo colorization | `V1PhotoColorizerCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `name` | ✗ | The name of image | `"Photo Colorizer image".to_string()` |
