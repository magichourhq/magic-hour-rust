#[derive(Debug)]
pub struct AnimationClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AnimationClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Animation
    ///
    /// Create a Animation video. The estimated frame cost is calculated based on the `fps` and `end_seconds` input.
    ///
    /// POST /v1/animation
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AnimationCreateResponse> {
        let url = self.base_client.build_url("/v1/animation");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1AnimationCreateBody {
                    name: request.name,
                    assets: request.assets,
                    end_seconds: request.end_seconds,
                    fps: request.fps,
                    height: request.height,
                    style: request.style,
                    width: request.width,
                },
            );
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("POST", response).await?;
        crate::core::response::process_json::<
            crate::models::V1AnimationCreateResponse,
        >(response)
            .await
    }
}
