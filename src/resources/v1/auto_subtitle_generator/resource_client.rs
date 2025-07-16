#[derive(Debug)]
pub struct AutoSubtitleGeneratorClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AutoSubtitleGeneratorClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Auto Subtitle Generator
    ///
    /// Automatically generate subtitles for your video in multiple languages.
    ///
    /// POST /v1/auto-subtitle-generator
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AutoSubtitleGeneratorCreateResponse> {
        let url = self.base_client.build_url("/v1/auto-subtitle-generator", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1AutoSubtitleGeneratorCreateBody {
                    name: request.name,
                    assets: request.assets,
                    end_seconds: request.end_seconds,
                    start_seconds: request.start_seconds,
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
            crate::models::V1AutoSubtitleGeneratorCreateResponse,
        >(response)
            .await
    }
}
