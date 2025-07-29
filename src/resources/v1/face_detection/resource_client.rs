#[derive(Debug)]
pub struct FaceDetectionClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> FaceDetectionClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Get face detection details
    ///
    /// Get the details of a face detection task.
    ///
    /// GET /v1/face-detection/{id}
    pub async fn get(
        &mut self,
        request: super::request_types::GetRequest,
    ) -> crate::SdkResult<crate::models::V1FaceDetectionGetResponse> {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/v1/face-detection/{}", crate ::core::params::format_string_param(&
                    request.id)
                ),
                None,
            );
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("GET", response).await?;
        crate::core::response::process_json::<
            crate::models::V1FaceDetectionGetResponse,
        >(response)
            .await
    }
    /// Face Detection
    ///
    /// Detect faces in an image or video.
    ///
    /// Note: Face detection is free to use for the near future. Pricing may change in the future.
    ///
    /// POST /v1/face-detection
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1FaceDetectionCreateResponse> {
        let url = self.base_client.build_url("/v1/face-detection", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1FaceDetectionCreateBody {
                    confidence_score: request.confidence_score,
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
            crate::models::V1FaceDetectionCreateResponse,
        >(response)
            .await
    }
}
