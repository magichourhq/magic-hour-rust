#[derive(Debug)]
pub struct TextToVideoClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> TextToVideoClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// Text-to-Video
    ///
    /// **What this API does**
    ///
    /// Create the same Text To Video you can make in the browser, but programmatically, so you can automate it, run it at scale, or connect it to your own app or workflow.
    ///
    /// **Good for**
    /// - Automation and batch processing
    /// - Adding text to video into apps, pipelines, or tools
    ///
    /// **How it works (3 steps)**
    /// 1) Upload your inputs (video, image, or audio) with [Generate Upload URLs](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls) and copy the `file_path`.
    /// 2) Send a request to create a text to video job with the basic fields.
    /// 3) Check the job status until it's `complete`, then download the result from `downloads`.
    ///
    /// **Key options**
    /// - Inputs: usually a file, sometimes a YouTube link, depending on project type
    /// - Resolution: free users are limited to 512px; higher plans unlock HD and larger sizes
    /// - Extra fields: e.g. `face_swap_mode`, `start_seconds`/`end_seconds`, or a text prompt
    ///
    /// **Cost**
    /// Credits are only charged for the frames that actually render. You'll see an estimate when the job is queued, and the final total after it's done.
    ///
    /// For detailed examples, see the [product page](https://magichour.ai/products/text-to-video).
    ///
    /// POST /v1/text-to-video
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1TextToVideoCreateResponse> {
        let url = self.base_client.build_url("/v1/text-to-video", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1TextToVideoCreateBody {
                    name: request.name,
                    resolution: request.resolution,
                    end_seconds: request.end_seconds,
                    orientation: request.orientation,
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
            crate::models::V1TextToVideoCreateResponse,
        >(response)
            .await
    }
}
