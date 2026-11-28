#[derive(Debug)]
pub struct ImageClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> ImageClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Estimate image credits
    ///
    /// Get the credits an image create request charges. Pass `tool`, then the fields below that the tool's create request uses, with the same values. Does not create a project or spend credits. It prices the fields passed here. It does not check your plan, your balance or the rest of the create request.
    ///
    /// GET /v1/estimate/image
    pub async fn list(
        &mut self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::V1EstimateImageListResponse> {
        let url = self.base_client.build_url("/v1/estimate/image", None);
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries.add("tool", &request.tool, crate::core::params::QueryStyle::Form, true);
        queries
            .add_option(
                "image_count",
                &request.image_count,
                crate::core::params::QueryStyle::Form,
                true,
            );
        queries
            .add_option(
                "mode",
                &request.mode,
                crate::core::params::QueryStyle::Form,
                true,
            );
        queries
            .add_option(
                "model",
                &request.model,
                crate::core::params::QueryStyle::Form,
                true,
            );
        queries
            .add_option(
                "resolution",
                &request.resolution,
                crate::core::params::QueryStyle::Form,
                true,
            );
        queries
            .add_option(
                "scale_factor",
                &request.scale_factor,
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
            crate::models::V1EstimateImageListResponse,
        >(response)
            .await
    }
}
