/// Optional style controls for replace vs animate mode and subject selection.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1CharacterReplaceCreateBodyStyle {
    /// Processing mode. `replace` swaps the detected subject with your reference character. `animate` transfers motion from the video onto your character image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::models::V1CharacterReplaceCreateBodyStyleModeEnum>,
    /// On-frame markers for manual subject selection. Required when `selection_mode` is `point`. Ignored when `selection_mode` is `auto` or omitted. Rejected for models without subject selection (supported by `wan-animate`, `minimax-h3`). `minimax-h3` subject `points` are `replace`-only; `animate` with `selection_mode` `point` returns 422, and with `auto` or no `selection_mode` the points are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<crate::models::V1CharacterReplaceCreateBodyStylePointsItem>>,
    /// Optional direction. Supported by kling-3.0, minimax-h3.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// How to locate the subject in the source video. `auto` detects a person automatically. `point` uses your `points` to mark the subject and is supported by `wan-animate`, `minimax-h3`. `minimax-h3` subject `points` are `replace`-only; `animate` with `selection_mode` `point` returns 422, and with `auto` or no `selection_mode` the points are ignored. Defaults to `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_mode: Option<
        crate::models::V1CharacterReplaceCreateBodyStyleSelectionModeEnum,
    >,
}
