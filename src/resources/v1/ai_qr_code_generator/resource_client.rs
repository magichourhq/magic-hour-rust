#[derive(Debug)]
pub struct AiQrCodeGeneratorClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AiQrCodeGeneratorClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// AI QR Code Generator
    ///
    /// Create an AI QR code. Each QR code costs 0 credits.
    ///
    /// POST /v1/ai-qr-code-generator
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AiQrCodeGeneratorCreateResponse> {
        let url = self.base_client.build_url("/v1/ai-qr-code-generator", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1AiQrCodeGeneratorCreateBody {
                    name: request.name,
                    content: request.content,
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
            crate::models::V1AiQrCodeGeneratorCreateResponse,
        >(response)
            .await
    }
}
