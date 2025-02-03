#[derive(Debug)]
pub struct AiPhotoEditorClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AiPhotoEditorClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// AI Photo Editor
    ///
    /// > **NOTE**: this API is still in early development stages, and should be avoided. Please reach out to us if you're interested in this API.
    ///
    /// Edit photo using AI. Each photo costs 10 frames.
    ///
    /// POST /v1/ai-photo-editor
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::PostV1AiPhotoEditorResponse> {
        let url = self.base_client.build_url("/v1/ai-photo-editor");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::PostV1AiPhotoEditorBody {
                    name: request.name,
                    steps: request.steps,
                    assets: request.assets,
                    resolution: request.resolution,
                    style: request.style,
                },
            );
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("POST", response).await?;
        crate::core::response::process_json::<
            crate::models::PostV1AiPhotoEditorResponse,
        >(response)
            .await
    }
}
