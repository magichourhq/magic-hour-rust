#[derive(Debug)]
pub struct AiClothesChangerClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AiClothesChangerClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// AI Clothes Changer
    ///
    /// Change outfits in photos in seconds with just a photo reference. Each photo costs 25 frames.
    ///
    /// POST /v1/ai-clothes-changer
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AiClothesChangerCreateResponse> {
        let url = self.base_client.build_url("/v1/ai-clothes-changer");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1AiClothesChangerCreateBody {
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
            crate::models::V1AiClothesChangerCreateResponse,
        >(response)
            .await
    }
}
