#[derive(Debug)]
pub struct SavedItemsClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> SavedItemsClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// List saved items
    ///
    /// Returns active saved items owned by the authenticated account, newest first. Each item includes every saved asset with a durable file_path for reuse in compatible generation APIs and a temporary signed URL for previewing or downloading. Filter by type to find characters, references, voices, moodboards, or brand kits. To fetch the next page, pass the response's next_cursor as cursor.
    ///
    /// GET /v1/saved-items
    pub async fn list(
        &mut self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::V1SavedItemsListResponse> {
        let url = self.base_client.build_url("/v1/saved-items", None);
        let mut builder = reqwest::Client::default().get(&url);
        let mut queries = crate::core::params::QueryParams::default();
        queries
            .add_option(
                "cursor",
                &request.cursor,
                crate::core::params::QueryStyle::Form,
                true,
            );
        queries
            .add_option(
                "limit",
                &request.limit,
                crate::core::params::QueryStyle::Form,
                true,
            );
        queries
            .add_option(
                "type",
                &request.type_,
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
            crate::models::V1SavedItemsListResponse,
        >(response)
            .await
    }
}
