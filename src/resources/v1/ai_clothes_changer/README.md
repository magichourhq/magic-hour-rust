
### AI Clothes Changer <a name="create"></a>

Change outfits in photos in seconds with just a photo reference. Each photo costs 25 credits.

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

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `assets` | ✓ | Provide the assets for clothes changer | `V1AiClothesChangerCreateBodyAssets {garment_file_path: "api-assets/id/outfit.png".to_string(), garment_type: V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::Dresses, person_file_path: "api-assets/id/model.png".to_string()}` |
| `name` | ✗ | The name of image | `"Clothes Changer image".to_string()` |
