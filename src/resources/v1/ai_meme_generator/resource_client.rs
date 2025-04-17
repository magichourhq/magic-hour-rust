#[derive(Debug)]
pub struct AiMemeGeneratorClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AiMemeGeneratorClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// AI Meme Generator
    ///
    /// Create an AI generated meme. Each meme costs 10 frames.
    ///
    /// POST /v1/ai-meme-generator
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AiMemeGeneratorCreateResponse> {
        let url = self.base_client.build_url("/v1/ai-meme-generator");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1AiMemeGeneratorCreateBody {
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
            crate::models::V1AiMemeGeneratorCreateResponse,
        >(response)
            .await
    }
}
