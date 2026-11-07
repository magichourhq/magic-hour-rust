#[derive(Debug)]
pub struct VoicesClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> VoicesClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// List voices
    ///
    /// List the voices available for text-to-speech. Use a voice's `id` as `style.voice_name` in the AI Voice Generator API. To fetch the next page, pass the response's next_cursor as cursor.
    ///
    /// GET /v1/voices
    pub async fn list(
        &mut self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::V1VoicesListResponse> {
        let url = self.base_client.build_url("/v1/voices", None);
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
        builder = builder.query(&queries.params);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("GET", response).await?;
        crate::core::response::process_json::<
            crate::models::V1VoicesListResponse,
        >(response)
            .await
    }
}
