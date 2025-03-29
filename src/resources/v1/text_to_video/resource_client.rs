#[derive(Debug)]
pub struct TextToVideoClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> TextToVideoClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Text-to-Video
    ///
    /// Create a Text To Video video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
    ///
    /// Get more information about this mode at our [product page](/products/text-to-video).
    ///
    ///
    /// POST /v1/text-to-video
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1TextToVideoCreateResponse> {
        let url = self.base_client.build_url("/v1/text-to-video");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1TextToVideoCreateBody {
                    name: request.name,
                    end_seconds: request.end_seconds,
                    orientation: request.orientation,
                    style: request.style,
                },
            );
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("POST", response).await?;
        crate::core::response::process_json::<
            crate::models::V1TextToVideoCreateResponse,
        >(response)
            .await
    }
}
