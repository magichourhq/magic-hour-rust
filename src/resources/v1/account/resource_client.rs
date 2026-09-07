#[derive(Debug)]
pub struct AccountClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AccountClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Get account details
    ///
    /// Get the current credit balance and subscription details of the account that owns the API key.
    ///
    /// GET /v1/account
    pub async fn list(
        &mut self,
    ) -> crate::SdkResult<crate::models::V1AccountListResponse> {
        let url = self.base_client.build_url("/v1/account", None);
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("GET", response).await?;
        crate::core::response::process_json::<
            crate::models::V1AccountListResponse,
        >(response)
            .await
    }
}
