/// Generated by Sideko (sideko.dev)
#[derive(Clone, Debug)]
pub struct AiQrCodeGeneratorClient {
    base_client: crate::core::base_client::BaseClient,
}
impl AiQrCodeGeneratorClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Create an AI QR code. Each QR code costs 20 frames.
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::PostV1AiQrCodeGeneratorResponse> {
        let url = self.base_client.build_url("/v1/ai-qr-code-generator");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self.base_client.apply_auths_to_builder(builder, &["bearerAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::PostV1AiQrCodeGeneratorResponse,
        >(response)
            .await
    }
}