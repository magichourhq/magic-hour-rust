#[derive(Debug)]
pub struct BodySwapClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> BodySwapClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Body Swap
    ///
    /// Swap a person into a scene image using Nano Banana 2. Credits depend on `resolution` (from 100 credits at 640px upward).
    ///
    /// POST /v1/body-swap
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1BodySwapCreateResponse> {
        let url = self.base_client.build_url("/v1/body-swap", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1BodySwapCreateBody {
                    name: request.name,
                    assets: request.assets,
                    resolution: request.resolution,
                },
            );
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("POST", response).await?;
        crate::core::response::process_json::<
            crate::models::V1BodySwapCreateResponse,
        >(response)
            .await
    }
}
