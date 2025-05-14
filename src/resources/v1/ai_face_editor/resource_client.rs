#[derive(Debug)]
pub struct AiFaceEditorClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AiFaceEditorClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// AI Face Editor
    ///
    /// Edit facial features of an image using AI. Each edit costs 1 frame. The height/width of the output image depends on your subscription. Please refer to our [pricing](/pricing) page for more details
    ///
    /// POST /v1/ai-face-editor
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AiFaceEditorCreateResponse> {
        let url = self.base_client.build_url("/v1/ai-face-editor", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1AiFaceEditorCreateBody {
                    name: request.name,
                    assets: request.assets,
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
            crate::models::V1AiFaceEditorCreateResponse,
        >(response)
            .await
    }
}
