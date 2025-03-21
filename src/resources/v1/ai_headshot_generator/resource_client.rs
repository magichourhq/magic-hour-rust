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
    /// AI Headshots
    ///
    /// Create an AI headshot. Each headshot costs 50 frames.
    ///
    /// POST /v1/ai-headshot-generator
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::PostV1AiHeadshotGeneratorResponse> {
        let url = self.base_client.build_url("/v1/ai-headshot-generator");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::PostV1AiHeadshotGeneratorBody {
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
            crate::models::PostV1AiHeadshotGeneratorResponse,
        >(response)
            .await
    }
}
