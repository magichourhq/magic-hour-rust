/// Provide the assets for clothes changer
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiClothesChangerCreateBodyAssets {
    /// The image of the outfit. This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    pub garment_file_path: String,
    /// Type of garment to swap. If not provided, swaps the entire outfit.
    /// * `upper_body` - for shirts/jackets
    /// * `lower_body` - for pants/skirts
    /// * `dresses` - for entire outfit (deprecated, use `entire_outfit` instead)
    /// * `entire_outfit` - for entire outfit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub garment_type: Option<
        crate::models::V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum,
    >,
    /// The image with the person. This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    pub person_file_path: String,
}
