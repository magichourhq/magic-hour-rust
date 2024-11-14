/// Generated by Sideko (sideko.dev)
#[derive(Clone, Debug)]
pub struct TextToVideoClient {
    base_client: crate::core::base_client::BaseClient,
}
impl TextToVideoClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Create a Text To Video video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
    ///
    /// Get more information about this mode at our [product page](/products/text-to-video).
    ///
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::PostV1TextToVideoResponse> {
        let url = self.base_client.build_url("/v1/text-to-video");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self.base_client.apply_auths_to_builder(builder, &["bearerAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::PostV1TextToVideoResponse,
        >(response)
            .await
    }
}
