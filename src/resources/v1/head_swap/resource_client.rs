#[derive(Debug)]
pub struct HeadSwapClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> HeadSwapClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Head Swap
    ///
    /// Swap a head onto a body image. Each image costs 10 credits. Output resolution depends on your subscription; you may set `max_resolution` lower than your plan maximum if desired.
    ///
    /// POST /v1/head-swap
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1HeadSwapCreateResponse> {
        let url = self.base_client.build_url("/v1/head-swap", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1HeadSwapCreateBody {
                    max_resolution: request.max_resolution,
                    name: request.name,
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
            crate::models::V1HeadSwapCreateResponse,
        >(response)
            .await
    }
}
