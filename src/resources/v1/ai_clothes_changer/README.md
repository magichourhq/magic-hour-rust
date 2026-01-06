# v1.ai_clothes_changer

## Module Functions

### AI Clothes Changer <a name="create"></a>

Change outfits in photos in seconds with just a photo reference. Each photo costs 25 credits.

**API Endpoint**: `POST /v1/ai-clothes-changer`

#### Parameters

| Parameter              | Required | Description                                                                                                                                                                                                                                                                                                                                              | Example                                                                                                                                                                                                                                     |
| ---------------------- | :------: | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `assets`               |    ✓     | Provide the assets for clothes changer                                                                                                                                                                                                                                                                                                                   | `V1AiClothesChangerCreateBodyAssets {garment_file_path: "api-assets/id/outfit.png".to_string(), garment_type: Some(V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::UpperBody), person_file_path: "api-assets/id/model.png".to_string()}` |
| `└─ garment_file_path` |    ✓     | The image of the outfit. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.   | `"api-assets/id/outfit.png".to_string()`                                                                                                                                                                                                    |
| `└─ garment_type`      |    ✗     | Deprecated: garment_type is no longer needed.                                                                                                                                                                                                                                                                                                            | `V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::UpperBody`                                                                                                                                                                              |
| `└─ person_file_path`  |    ✓     | The image with the person. This value is either - a direct URL to the video file - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls). See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details. | `"api-assets/id/model.png".to_string()`                                                                                                                                                                                                     |
| `name`                 |    ✗     | Give your image a custom name for easy identification.                                                                                                                                                                                                                                                                                                   | `"My Clothes Changer image".to_string()`                                                                                                                                                                                                    |

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
            garment_type: Some(
                magic_hour::models::V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::UpperBody,
            ),
            person_file_path: "api-assets/id/model.png".to_string(),
        },
        name: Some("My Clothes Changer image".to_string()),
    })
    .await;
```

#### Response

##### Type

[V1AiClothesChangerCreateResponse](/src/models/v1_ai_clothes_changer_create_response.rs)

##### Example

```rust
V1AiClothesChangerCreateResponse {credits_charged: 25, frame_cost: 25, id: "cuid-example".to_string()}
```
