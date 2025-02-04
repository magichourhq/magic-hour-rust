#[derive(Debug)]
pub struct ImageBackgroundRemoverClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> ImageBackgroundRemoverClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Image Background Remover
    ///
    /// Remove background from image. Each image costs 5 frames.
    ///
    /// POST /v1/image-background-remover
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1ImageBackgroundRemovercreateResponse> {
        let url = self.base_client.build_url("/v1/image-background-remover");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::PostV1ImageBackgroundRemoverBody {
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
            crate::models::V1ImageBackgroundRemovercreateResponse,
        >(response)
            .await
    }
}
