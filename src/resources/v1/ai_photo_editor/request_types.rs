/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for photo editor
    pub assets: crate::models::V1AiPhotoEditorCreateBodyAssets,
    /// The name of image. This value is mainly used for your own identification of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The resolution of the final output image. The allowed value is based on your subscription. Please refer to our [pricing page](https://magichour.ai/pricing) for more details
    pub resolution: i64,
    /// Deprecated: Please use `.style.steps` instead. Number of iterations used to generate the output. Higher values improve quality and increase the strength of the prompt but increase processing time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
    pub style: crate::models::V1AiPhotoEditorCreateBodyStyle,
}
