/// Style of the face swap video.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceSwapCreateBodyStyle {
    /// * `v1` - May preserve skin detail and texture better, but weaker identity preservation.
    /// * `v2` - Faster, sharper, better handling of hair and glasses. stronger identity preservation.
    /// * `default` - Use the version we recommend, which will change over time. This is recommended unless you need a specific earlier version. This is the default behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::models::V1FaceSwapCreateBodyStyleVersionEnum>,
}
