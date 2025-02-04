#[derive(Debug)]
pub struct VideoProjectsClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> VideoProjectsClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Delete video
    ///
    /// Permanently delete the rendered video. This action is not reversible, please be sure before deleting.
    ///
    /// DELETE /v1/video-projects/{id}
    pub async fn delete(
        &mut self,
        request: super::request_types::DeleteRequest,
    ) -> crate::SdkResult<()> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/v1/video-projects/{}", crate ::core::params::format_string_param(&
                    request.id)
                ),
            );
        let mut builder = reqwest::Client::default().delete(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let response = builder.send().await?;
        self.base_client.error_for_status("DELETE", response).await?;
        Ok(())
    }
    /// Get video details
    ///
    /// Get the details of a video project. The `download` field will be `null` unless the video was successfully rendered.
    ///
    /// The video can be one of the following status
    /// - `draft` - not currently used
    /// - `queued` - the job is queued and waiting for a GPU
    /// - `rendering` - the generation is in progress
    /// - `complete` - the video is successful created
    /// - `error` - an error occurred during rendering
    /// - `canceled` - video render is canceled by the user
    ///
    ///
    /// GET /v1/video-projects/{id}
    pub async fn get(
        &mut self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<crate::models::V1VideoProjectsgetResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/v1/video-projects/{}", crate ::core::params::format_string_param(&
                    request.id)
                ),
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("GET", response).await?;
        crate::core::response::process_json::<
            crate::models::V1VideoProjectsgetResponse,
        >(response)
            .await
    }
}
