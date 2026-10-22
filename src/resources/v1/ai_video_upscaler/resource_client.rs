#[derive(Debug)]
pub struct AiVideoUpscalerClient<'a> {
    base_client: &'a mut crate::core::base_client::BaseClient,
}
impl<'a> AiVideoUpscalerClient<'a> {
    pub(crate) fn _new(
        base_client: &'a mut crate::core::base_client::BaseClient,
    ) -> Self {
        Self { base_client }
    }
    /// AI Video Upscaler
    ///
    /// **What this API does**
    ///
    /// Create the same Video Upscaler you can make in the browser, but programmatically, so you can automate it, run it at scale, or connect it to your own app or workflow.
    ///
    /// **Good for**
    /// - Automation and batch processing
    /// - Adding video upscaler into apps, pipelines, or tools
    ///
    /// **How it works (3 steps)**
    /// 1) Upload your inputs (video, image, or audio) with [Generate Upload URLs](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls) and copy the `file_path`.
    /// 2) Send a request to create a video upscaler job with the basic fields.
    /// 3) Check the job status until it's `complete`, then download the result from `downloads`.
    ///
    /// **Key options**
    /// - Inputs: usually a file, sometimes a YouTube link, depending on project type
    /// - Resolution: output size and plan requirements vary by workflow and model. Some workflows derive output size from the input video.
    /// - Extra fields: e.g. `face_swap_mode`, `start_seconds`/`end_seconds`, or a text prompt
    ///
    /// **Cost**
    /// Credits are charged when the job is queued (`credits_charged` in the response). Tools billed per frame charge an estimate and settle to the frames that actually render once the job finishes; tools billed per second charge the final amount upfront. Failed jobs are refunded.
    ///
    /// For detailed examples, see the [product page](https://magichour.ai/products/video-upscaler).
    ///
    /// POST /v1/ai-video-upscaler
    pub async fn create(
        &mut self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<crate::models::V1AiVideoUpscalerCreateResponse> {
        let url = self.base_client.build_url("/v1/ai-video-upscaler", None);
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder
            .json(
                &crate::models::V1AiVideoUpscalerCreateBody {
                    name: request.name,
                    start_seconds: request.start_seconds,
                    assets: request.assets,
                    end_seconds: request.end_seconds,
                    resolution: request.resolution,
                },
            );
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["bearerAuth"])
            .await?;
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status("POST", response).await?;
        crate::core::response::process_json::<
            crate::models::V1AiVideoUpscalerCreateResponse,
        >(response)
            .await
    }
}
