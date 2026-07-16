/// V1CharacterReplaceCreateBodyStylePointsItem
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1CharacterReplaceCreateBodyStylePointsItem {
    /// Horizontal pixel coordinate in the source video frame at `time_seconds`, measured from the left edge.
    pub position_x: i64,
    /// Vertical pixel coordinate in the source video frame at `time_seconds`, measured from the top edge.
    pub position_y: i64,
    /// Timestamp on the source video timeline in seconds. Uses the same clock as `start_seconds` and `end_seconds`.
    pub time_seconds: f64,
}
