#[derive(Debug)]
pub struct FaceSwapClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> FaceSwapClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Face Swap video
    ///
    /// Create a Face Swap video. The estimated frame cost is calculated using 30 FPS. This amount is deducted from your account balance when a video is queued. Once the video is complete, the cost will be updated based on the actual number of frames rendered.
    ///
    /// Get more information about this mode at our [product page](https://magichour.ai/products/face-swap).
    ///
    ///
    /// POST /v1/face-swap
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1FaceSwapCreateResponse> {
        let url = self.base_client.build_url("/v1/face-swap", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1FaceSwapCreateBody {
                    height: request.height,
                    name: request.name,
                    style: request.style,
                    width: request.width,
                    assets: request.assets,
                    end_seconds: request.end_seconds,
                    start_seconds: request.start_seconds,
                },
            );
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("POST", response).await?;
        crate::core::response::process_json::<
            crate::models::V1FaceSwapCreateResponse,
        >(response)
            .await
    }
}
