/// Face editing parameters
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiFaceEditorCreateBodyStyle {
    /// Enhance face features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhance_face: Option<bool>,
    /// Horizontal eye gaze (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eye_gaze_horizontal: Option<f64>,
    /// Vertical eye gaze (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eye_gaze_vertical: Option<f64>,
    /// Eye open ratio (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eye_open_ratio: Option<f64>,
    /// Eyebrow direction (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyebrow_direction: Option<f64>,
    /// Head pitch (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_pitch: Option<f64>,
    /// Head roll (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_roll: Option<f64>,
    /// Head yaw (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_yaw: Option<f64>,
    /// Lip open ratio (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lip_open_ratio: Option<f64>,
    /// Mouth grim (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouth_grim: Option<f64>,
    /// Horizontal mouth position (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouth_position_horizontal: Option<f64>,
    /// Vertical mouth position (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouth_position_vertical: Option<f64>,
    /// Mouth pout (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouth_pout: Option<f64>,
    /// Mouth purse (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouth_purse: Option<f64>,
    /// Mouth smile (-100 to 100), in increments of 5
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouth_smile: Option<f64>,
}
