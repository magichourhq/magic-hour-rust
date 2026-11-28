#[derive(Debug)]
pub struct VideoClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> VideoClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Estimate video credits
    ///
    /// Get the credits a video create request charges when it is submitted. Pass `tool`, then the fields below that the tool's create request uses, with the same values. Tools billed per frame charge an estimate at submission, assuming 30 fps for an uploaded video, and settle to the frames that render. Does not create a project or spend credits. It prices the fields passed here. It does not check your plan, your balance or the rest of the create request.
    ///
    /// GET /v1/estimate/video
    pub async fn list(
        &mut self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::V1EstimateVideoListResponse> {
        let url = self.base_client.build_url("/v1/estimate/video", None);
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries
            .add(
                "end_seconds",
                &request.end_seconds,
                crate::core::params::QueryStyle::Form,
                true,
            );
        queries.add("tool", &request.tool, crate::core::params::QueryStyle::Form, true);
        queries
            .add_option(
                "audio",
                &request.audio,
                crate::core::params::QueryStyle::Form,
                true,
            );
        queries
            .add_option(
                "has_video_reference",
                &request.has_video_reference,
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
                "start_seconds",
                &request.start_seconds,
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
            crate::models::V1EstimateVideoListResponse,
        >(response)
            .await
    }
}
