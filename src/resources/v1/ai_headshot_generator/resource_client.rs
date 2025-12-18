#[derive(Debug)]
pub struct AiHeadshotGeneratorClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AiHeadshotGeneratorClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// AI Headshot Generator
    ///
    /// Create an AI headshot. Each headshot costs 50 credits.
    ///
    /// POST /v1/ai-headshot-generator
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AiHeadshotGeneratorCreateResponse> {
        let url = self.base_client.build_url("/v1/ai-headshot-generator", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1AiHeadshotGeneratorCreateBody {
                    name: request.name,
                    style: request.style,
                    assets: request.assets,
                },
            );
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("POST", response).await?;
        crate::core::response::process_json::<
            crate::models::V1AiHeadshotGeneratorCreateResponse,
        >(response)
            .await
    }
}
