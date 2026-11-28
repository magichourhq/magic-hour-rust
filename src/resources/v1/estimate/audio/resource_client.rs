#[derive(Debug)]
pub struct AudioClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AudioClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Estimate audio credits
    ///
    /// Get the credits an audio create request charges. Pass `tool`, then the field below that the tool's create request uses. Does not create a project or spend credits. It prices the fields passed here. It does not check your plan, your balance or the rest of the create request.
    ///
    /// GET /v1/estimate/audio
    pub async fn list(
        &mut self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::V1EstimateAudioListResponse> {
        let url = self.base_client.build_url("/v1/estimate/audio", None);
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add("tool", &request.tool, crate::core::params::QueryStyle::Form, true);
        queries
            .add_option(
                "character_count",
                &request.character_count,
                crate::core::params::QueryStyle::Form,
                true,
            );
        queries
            .add_option(
                "duration",
                &request.duration,
                crate::core::params::QueryStyle::Form,
                true,
            );
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("GET", response).await?;
        crate::core::response::process_json::<
            crate::models::V1EstimateAudioListResponse,
        >(response)
            .await
    }
}
