/// V1AiPhotoEditorCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiPhotoEditorCreateBodyStyle {
    /// Use this to describe what your input image is. This helps maintain aspects of the image you don't want to change.
    pub image_description: String,
    /// Determines the input image's influence. Higher values align the output more with the initial image.
    pub likeness_strength: f64,
    /// What you want to avoid seeing in the final output; has a minor effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// What you want your final output to look like. We recommend starting with the image description and making minor edits for best results.
    pub prompt: String,
    /// Determines the prompt's influence. Higher values align the output more with the prompt.
    pub prompt_strength: f64,
    /// Number of iterations used to generate the output. Higher values improve quality and increase the strength of the prompt but increase processing time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
    /// The multiplier applied to an image's original dimensions during the upscaling process. For example, a scale of 2 doubles the width and height (e.g., from 512x512 to 1024x1024).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upscale_factor: Option<i64>,
    /// Upscale fidelity refers to the level of quality desired in the generated image. Fidelity value of 1 means more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upscale_fidelity: Option<f64>,
}
