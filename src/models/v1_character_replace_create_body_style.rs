/// Optional style controls for replace vs animate mode and subject selection.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1CharacterReplaceCreateBodyStyle {
    /// Processing mode. `replace` swaps the detected subject with your reference character. `animate` transfers motion from the video onto your character image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::models::V1CharacterReplaceCreateBodyStyleModeEnum>,
    /// On-frame markers for manual subject selection. Required when `selection_mode` is `point`. Ignored when `selection_mode` is `auto` or omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<crate::models::V1CharacterReplaceCreateBodyStylePointsItem>>,
    /// How to locate the subject in the source video. `auto` detects a person automatically. `point` uses your `points` to mark the subject. Defaults to `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_mode: Option<
        crate::models::V1CharacterReplaceCreateBodyStyleSelectionModeEnum,
    >,
}
