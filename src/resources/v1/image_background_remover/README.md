
### create <a name="create"></a>
Image Background Remover

Remove background from image. Each image costs 5 frames.

**API Endpoint**: `POST /v1/image-background-remover`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .image_background_remover()
    .create(magic_hour::resources::v1::image_background_remover::CreateRequest {
        assets: magic_hour::models::PostV1ImageBackgroundRemoverBodyAssets {
            image_file_path: "api-assets/id/1234.png".to_string(),
        },
        ..Default::default()
    })
    .await;
```
