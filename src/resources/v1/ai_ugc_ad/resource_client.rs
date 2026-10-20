#[derive(Debug)]
pub struct AiUgcAdClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AiUgcAdClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// AI UGC Ad Generator
    ///
    /// Create a spoken UGC video from a prepared image and script. Returns a project ID; poll GET /v1/video-projects/{id} for status and downloads. To compose an image, generate the image separately with the Image Editor API and supply its download URL.
    ///
    /// POST /v1/ai-ugc-ad
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AiUgcAdCreateResponse> {
        let url = self.base_client.build_url("/v1/ai-ugc-ad", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1AiUgcAdCreateBody {
                    additional_properties: request.additional_properties,
                    model: request.model,
                    resolution: request.resolution,
                    assets: request.assets,
                    end_seconds: request.end_seconds,
                    script: request.script,
                },
            );
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("POST", response).await?;
        crate::core::response::process_json::<
            crate::models::V1AiUgcAdCreateResponse,
        >(response)
            .await
    }
}
