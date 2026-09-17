# v1.saved_items

## Module Functions

### List saved items <a name="list"></a>

Returns active saved items owned by the authenticated account, newest first. Each item includes every saved asset with a durable file_path for reuse in compatible generation APIs and a temporary signed URL for previewing or downloading. Filter by type to find characters, references, voices, moodboards, or brand kits. To fetch the next page, pass the response's next_cursor as cursor.

**API Endpoint**: `GET /v1/saved-items`

#### Parameters

| Parameter | Required | Description                                                        | Example                               |
| --------- | :------: | ------------------------------------------------------------------ | ------------------------------------- |
| `cursor`  |    ✗     | Opaque pagination cursor from the previous response's next_cursor. | `"string".to_string()`                |
| `limit`   |    ✗     | Maximum number of saved items to return. Defaults to 20.           | `20`                                  |
| `type_`   |    ✗     | Only return saved items of this type.                              | `V1SavedItemsListTypeEnum::Character` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .saved_items()
    .list(magic_hour::resources::v1::saved_items::ListRequest {
        limit: Some(20),
        type_: Some(magic_hour::models::V1SavedItemsListTypeEnum::Character),
        ..Default::default()
    })
    .await;
```

#### Response

##### Type

[V1SavedItemsListResponse](/src/models/v1_saved_items_list_response.rs)

##### Example

```rust
V1SavedItemsListResponse {items: vec![V1SavedItemsListResponseItemsItem {assets: vec![V1SavedItemsListResponseItemsItemAssetsItem {file_path: "saved-items/user-id/item-id/image.png".to_string(), is_primary: true, media_kind: V1SavedItemsListResponseItemsItemAssetsItemMediaKindEnum::Image, url: "http://www.example.com".to_string(), url_expires_at: "2026-09-17T00:00:00.000Z".to_string()}], id: "cuid-example".to_string(), name: Some("Alex".to_string()), type_: V1SavedItemsListResponseItemsItemTypeEnum::Character}], next_cursor: Some("string".to_string())}
```
