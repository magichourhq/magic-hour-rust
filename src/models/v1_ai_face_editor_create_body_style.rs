/// Face editing parameters
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiFaceEditorCreateBodyStyle {
    /// Enhance face features
    pub enhance_face: bool,
    /// Horizontal eye gaze (-100 to 100), in increments of 5
    pub eye_gaze_horizontal: f64,
    /// Vertical eye gaze (-100 to 100), in increments of 5
    pub eye_gaze_vertical: f64,
    /// Eye open ratio (-100 to 100), in increments of 5
    pub eye_open_ratio: f64,
    /// Eyebrow direction (-100 to 100), in increments of 5
    pub eyebrow_direction: f64,
    /// Head pitch (-100 to 100), in increments of 5
    pub head_pitch: f64,
    /// Head roll (-100 to 100), in increments of 5
    pub head_roll: f64,
    /// Head yaw (-100 to 100), in increments of 5
    pub head_yaw: f64,
    /// Lip open ratio (-100 to 100), in increments of 5
    pub lip_open_ratio: f64,
    /// Mouth grim (-100 to 100), in increments of 5
    pub mouth_grim: f64,
    /// Horizontal mouth position (-100 to 100), in increments of 5
    pub mouth_position_horizontal: f64,
    /// Vertical mouth position (-100 to 100), in increments of 5
    pub mouth_position_vertical: f64,
    /// Mouth pout (-100 to 100), in increments of 5
    pub mouth_pout: f64,
    /// Mouth purse (-100 to 100), in increments of 5
    pub mouth_purse: f64,
    /// Mouth smile (-100 to 100), in increments of 5
    pub mouth_smile: f64,
}
