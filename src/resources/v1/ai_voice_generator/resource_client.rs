#[derive(Debug)]
pub struct AiVoiceGeneratorClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AiVoiceGeneratorClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// AI Voice Generator
    ///
    /// Generate speech from text. Each character costs 0.05 credits. The cost is rounded up to the nearest whole number.
    ///
    /// POST /v1/ai-voice-generator
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AiVoiceGeneratorCreateResponse> {
        let url = self.base_client.build_url("/v1/ai-voice-generator", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1AiVoiceGeneratorCreateBody {
                    name: request.name,
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
            crate::models::V1AiVoiceGeneratorCreateResponse,
        >(response)
            .await
    }
}
