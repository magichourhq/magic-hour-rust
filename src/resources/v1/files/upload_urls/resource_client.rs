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
    /// Generates a list of pre-signed upload URLs for the assets required. This API is only necessary if you want to upload to Magic Hour's storage. Refer to the [Input Files Guide](/integration/input-files) for more details.
    ///
    /// The response array will match the order of items in the request body.
    ///
    /// **Valid file extensions per asset type**:
    /// - video: mp4, m4v, mov, webm
    /// - audio: mp3, wav, aac, flac, webm, m4a
    /// - image: png, jpg, jpeg, heic, webp, avif, jp2, tiff, bmp
    /// - gif: gif, webp, webm
    ///
    /// > Note: `gif` is only supported for face swap API `video_file_path` field.
    ///
    /// Once you receive an upload URL, send a `PUT` request to upload the file directly.
    ///
    /// Example:
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
