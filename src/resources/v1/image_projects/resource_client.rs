#[derive(Debug)]
pub struct ImageProjectsClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> ImageProjectsClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Delete image
    ///
    /// Permanently delete the rendered image. This action is not reversible, please be sure before deleting.
    ///
    /// DELETE /v1/image-projects/{id}
    pub async fn delete(
        &mut self,
        request: super::request_types::DeleteRequest,
    ) -> crate::SdkResult<()> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/v1/image-projects/{}", crate ::core::params::format_string_param(&
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
    /// Get image details
    ///
    /// Get the details of a image project. The `downloads` field will be empty unless the image was successfully rendered.
    ///
    /// The image can be one of the following status
    /// - `draft` - not currently used
    /// - `queued` - the job is queued and waiting for a GPU
    /// - `rendering` - the generation is in progress
    /// - `complete` - the image is successful created
    /// - `error` - an error occurred during rendering
    /// - `canceled` - image render is canceled by the user
    ///
    ///
    /// GET /v1/image-projects/{id}
    pub async fn get(
        &mut self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<crate::models::GetV1ImageProjectsIdResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/v1/image-projects/{}", crate ::core::params::format_string_param(&
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
            crate::models::GetV1ImageProjectsIdResponse,
        >(response)
            .await
    }
}
