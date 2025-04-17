
### create <a name="create"></a>
AI Clothes Changer

Change outfits in photos in seconds with just a photo reference. Each photo costs 25 frames.

**API Endpoint**: `POST /v1/ai-clothes-changer`

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .ai_clothes_changer()
    .create(magic_hour::resources::v1::ai_clothes_changer::CreateRequest {
        assets: magic_hour::models::V1AiClothesChangerCreateBodyAssets {
            garment_file_path: "api-assets/id/outfit.png".to_string(),
            garment_type: magic_hour::models::V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::Dresses,
            person_file_path: "api-assets/id/model.png".to_string(),
        },
        name: Some("Clothes Changer image".to_string()),
    })
    .await;
```
