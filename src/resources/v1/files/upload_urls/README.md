
### Generate asset upload urls <a name="create"></a>

Create a list of urls used to upload the assets needed to generate a video. Each video type has their own requirements on what assets are required. Please refer to the specific mode API for more details. The response array will be in the same order as the request body.

Below is the list of valid extensions for each asset type:

- video: mp4, m4v, mov, webm
- audio: mp3, mpeg, wav, aac, aiff, flac
- image: png, jpg, jpeg, webp, avif, jp2, tiff, bmp

Note: `.gif` is supported for face swap API `video_file_path` field.

After receiving the upload url, you can upload the file by sending a PUT request with the header `'Content-Type: application/octet-stream'`.

For example using curl

```
curl -X PUT -H 'Content-Type: application/octet-stream' \
  --data '@/path/to/file/video.mp4' \
  https://videos.magichour.ai/api-assets/id/video.mp4?auth-value=1234567890
```


**API Endpoint**: `POST /v1/files/upload-urls`

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

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `items` | ✓ |  | `vec![V1FilesUploadUrlsCreateBodyItemsItem {extension: "mp4".to_string(), type_: V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Video}, V1FilesUploadUrlsCreateBodyItemsItem {extension: "mp3".to_string(), type_: V1FilesUploadUrlsCreateBodyItemsItemTypeEnum::Audio}]` |
