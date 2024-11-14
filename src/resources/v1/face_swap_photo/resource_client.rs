/// Generated by Sideko (sideko.dev)
#[derive(Clone, Debug)]
pub struct FaceSwapPhotoClient {
    base_client: crate::core::base_client::BaseClient,
}
impl FaceSwapPhotoClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Create a face swap photo. Each photo costs 5 frames. The height/width of the output image depends on your subscription. Please refer to our [pricing](/pricing) page for more details
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::PostV1FaceSwapPhotoResponse> {
        let url = self.base_client.build_url("/v1/face-swap-photo");
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self.base_client.apply_auths_to_builder(builder, &["bearerAuth"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::PostV1FaceSwapPhotoResponse,
        >(response)
            .await
    }
}
