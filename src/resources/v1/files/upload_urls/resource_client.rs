#[derive(Debug)]
pub struct UploadUrlsClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> UploadUrlsClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Generate asset upload urls
    ///
    /// Create a list of urls used to upload the assets needed to generate a video. Each video type has their own requirements on what assets are required. Please refer to the specific mode API for more details. The response array will be in the same order as the request body.
    ///
    /// Below is the list of valid extensions for each asset type:
    ///
    /// - video: mp4, m4v, mov, webm
    /// - audio: mp3, mpeg, wav, aac, aiff, flac
    /// - image: png, jpg, jpeg, webp, avif, jp2, tiff, bmp
    ///
    /// Note: `.gif` is supported for face swap API `video_file_path` field.
    ///
    /// After receiving the upload url, you can upload the file by sending a PUT request.
    ///
    /// For example using curl
    ///
    /// `-`-`
    /// curl -X PUT --data '@/path/to/file/video.mp4' \
    ///   https://videos.magichour.ai/api-assets/id/video.mp4?<auth params from the API response>
    /// `-`-`
    ///
    ///
    /// POST /v1/files/upload-urls
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1FilesUploadUrlsCreateResponse> {
        let url = self.base_client.build_url("/v1/files/upload-urls", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1FilesUploadUrlsCreateBody {
                    items: request.items,
                },
            );
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("POST", response).await?;
        crate::core::response::process_json::<
            crate::models::V1FilesUploadUrlsCreateResponse,
        >(response)
            .await
    }
}
