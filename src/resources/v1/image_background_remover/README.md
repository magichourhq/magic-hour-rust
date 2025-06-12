
### Image Background Remover <a name="create"></a>

Remove background from image. Each image costs 5 credits.

**API Endpoint**: `POST /v1/image-background-remover`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .image_background_remover()
    .create(magic_hour::resources::v1::image_background_remover::CreateRequest {
        assets: magic_hour::models::V1ImageBackgroundRemoverCreateBodyAssets {
            image_file_path: "api-assets/id/1234.png".to_string(),
        },
        name: Some("Background Remover image".to_string()),
    })
    .await;
```

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for background removal | `V1ImageBackgroundRemoverCreateBodyAssets {image_file_path: "api-assets/id/1234.png".to_string()}` |
| `name` | ✗ | The name of image | `"Background Remover image".to_string()` |
