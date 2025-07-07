
### Generate asset upload urls <a name="create"></a>

Create a list of urls used to upload the assets needed to generate a video. Each video type has their own requirements on what assets are required. Please refer to the specific mode API for more details. The response array will be in the same order as the request body.

Below is the list of valid extensions for each asset type:

- video: mp4, m4v, mov, webm
- audio: mp3, mpeg, wav, aac, aiff, flac
- image: png, jpg, jpeg, webp, avif, jp2, tiff, bmp

Note: `.gif` is supported for face swap API `video_file_path` field.

After receiving the upload url, you can upload the file by sending a PUT request.

For example using curl

```
curl -X PUT --data '@/path/to/file/video.mp4' \
  https://videos.magichour.ai/api-assets/id/video.mp4?<auth params from the API response>
```


**API Endpoint**: `POST /v1/files/upload-urls`

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `items` | ✓ |  | `vec![V1FilesUploadUrlsCreateBodyItemsItem {extension: "mp4".to_string(), type_: V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Video}, V1FilesUploadUrlsCreateBodyItemsItem {extension: "mp3".to_string(), type_: V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Audio}]` |

#### Example Snippet

```rust
let client = magic_hour::Client::default()
    .with_bearer_auth(&std::env::var("API_TOKEN").unwrap());
let res = client
    .v1()
    .files()
    .upload_urls()
    .create(magic_hour::resources::v1::files::upload_urls::CreateRequest {
        items: vec![
            magic_hour::models::V1FilesUploadUrlsCreateBodyItemsItem { extension :
            "mp4".to_string(), type_ :
            magic_hour::models::V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Video
            }, magic_hour::models::V1FilesUploadUrlsCreateBodyItemsItem { extension :
            "mp3".to_string(), type_ :
            magic_hour::models::V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Audio }
        ],
    })
    .await;
```

#### Response

##### Type
[V1FilesUploadUrlsCreateResponse](/src/models/v1_files_upload_urls_create_response.rs)

##### Example
`V1FilesUploadUrlsCreateResponse {items: vec![V1FilesUploadUrlsCreateResponseItemsItem {expires_at: "2024-07-25T16:56:21.932Z".to_string(), file_path: "api-assets/id/video.mp4".to_string(), upload_url: "https://videos.magichour.ai/api-assets/id/video.mp4?auth-value=1234567890".to_string()}, V1FilesUploadUrlsCreateResponseItemsItem {expires_at: "2024-07-25T16:56:21.932Z".to_string(), file_path: "api-assets/id/audio.mp3".to_string(), upload_url: "https://videos.magichour.ai/api-assets/id/audio.mp3?auth-value=1234567890".to_string()}]}`
