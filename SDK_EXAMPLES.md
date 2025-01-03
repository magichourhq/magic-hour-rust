
# SDK Usage Examples


## Module v1.image_projects
            
### Get image project details
Get the details of a image project. The `download` field will be `null` unless the image was successfully rendered.

The image can be one of the following status
- `draft` - not currently used
- `queued` - the job is queued and waiting for a GPU
- `rendering` - the generation is in progress
- `complete` - the image is successful created
- `error` - an error occurred during rendering
- `canceled` - image render is canceled by the user


**API Endpoint**: `GET /v1/image-projects/{id}`


#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .image_projects()
    .get(magic_hour::resources::v1::image_projects::GetRequest {
        id: "string".to_string(),
    })
    .await;
```

    
**Upgrade to see all examples**

