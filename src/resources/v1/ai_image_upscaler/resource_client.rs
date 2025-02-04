#[derive(Debug)]
pub struct AiImageUpscalerClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AiImageUpscalerClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// AI Image Upscaler
    ///
    /// Upscale your image using AI. Each 2x upscale costs 50 frames, and 4x upscale costs 200 frames.
    ///
    /// POST /v1/ai-image-upscaler
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AiImageUpscalercreateResponse> {
        let url = self.base_client.build_url("/v1/ai-image-upscaler");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::PostV1AiImageUpscalerBody {
                    name: request.name,
                    assets: request.assets,
                    scale_factor: request.scale_factor,
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
            crate::models::V1AiImageUpscalercreateResponse,
        >(response)
            .await
    }
}
