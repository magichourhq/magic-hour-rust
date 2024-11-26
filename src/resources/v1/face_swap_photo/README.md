
### create <a name="create"></a>
Create Face Swap Photo

Create a face swap photo. Each photo costs 5 frames. The height/width of the output image depends on your subscription. Please refer to our [pricing](/pricing) page for more details

**API Endpoint**: `POST /v1/face-swap-photo`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .face_swap_photo()
    .create(magic_hour::resources::v1::face_swap_photo::CreateRequest {
        data: magic_hour::models::PostV1FaceSwapPhotoBody {
            assets: magic_hour::models::PostV1FaceSwapPhotoBodyAssets {
                source_file_path: "image/id/1234.png".to_string(),
                target_file_path: "image/id/1234.png".to_string(),
            },
            name: Some("Face Swap image".to_string()),
        },
    })
    .await;
```

**Upgrade to see all examples**
