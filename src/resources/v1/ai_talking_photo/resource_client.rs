#[derive(Debug)]
pub struct AiTalkingPhotoClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AiTalkingPhotoClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// AI Talking Photo
    ///
    /// Create a talking photo from an image and audio or text input.
    ///
    /// POST /v1/ai-talking-photo
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AiTalkingPhotoCreateResponse> {
        let url = self.base_client.build_url("/v1/ai-talking-photo", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1AiTalkingPhotoCreateBody {
                    name: request.name,
                    assets: request.assets,
                    end_seconds: request.end_seconds,
                    start_seconds: request.start_seconds,
                },
            );
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("POST", response).await?;
        crate::core::response::process_json::<
            crate::models::V1AiTalkingPhotoCreateResponse,
        >(response)
            .await
    }
}
