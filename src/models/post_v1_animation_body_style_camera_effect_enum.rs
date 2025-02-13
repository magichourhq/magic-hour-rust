/// PostV1AnimationBodyStyleCameraEffectEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1AnimationBodyStyleCameraEffectEnum {
    #[default]
    #[serde(rename = "Accelerate")]
    Accelerate,
    #[serde(rename = "Aggressive Zoom In - Audio Sync")]
    AggressiveZoomInAudioSync,
    #[serde(rename = "Aggressive Zoom Out - Audio Sync")]
    AggressiveZoomOutAudioSync,
    #[serde(rename = "Boost Zoom In")]
    BoostZoomIn,
    #[serde(rename = "Boost Zoom Out")]
    BoostZoomOut,
    #[serde(rename = "Bounce In And Out")]
    BounceInAndOut,
    #[serde(rename = "Bounce Out")]
    BounceOut,
    #[serde(rename = "Bounce Out - Audio Sync")]
    BounceOutAudioSync,
    #[serde(rename = "Bounce and Spin - Audio Sync")]
    BounceAndSpinAudioSync,
    #[serde(rename = "Bounce in Place")]
    BounceInPlace,
    #[serde(rename = "Cog in the Machine")]
    CogInTheMachine,
    #[serde(rename = "Devolve - Audio Sync")]
    DevolveAudioSync,
    #[serde(rename = "Directed by AI")]
    DirectedByAi,
    #[serde(rename = "Dramatic Zoom In")]
    DramaticZoomIn,
    #[serde(rename = "Dramatic Zoom Out")]
    DramaticZoomOut,
    #[serde(rename = "Drift Spin")]
    DriftSpin,
    #[serde(rename = "Earthquake Bounce")]
    EarthquakeBounce,
    #[serde(rename = "Earthquake Bounce - Audio Sync")]
    EarthquakeBounceAudioSync,
    #[serde(rename = "Evolve - Audio Sync")]
    EvolveAudioSync,
    #[serde(rename = "Heartbeat")]
    Heartbeat,
    #[serde(rename = "Hesitate In")]
    HesitateIn,
    #[serde(rename = "Jump")]
    Jump,
    #[serde(rename = "Pan Left")]
    PanLeft,
    #[serde(rename = "Pan Right")]
    PanRight,
    #[serde(rename = "Pulse - Audio Sync")]
    PulseAudioSync,
    #[serde(rename = "Pusher")]
    Pusher,
    #[serde(rename = "Pusher - Audio Sync")]
    PusherAudioSync,
    #[serde(rename = "Quadrant")]
    Quadrant,
    #[serde(rename = "Rise and Climb")]
    RiseAndClimb,
    #[serde(rename = "Road Trip")]
    RoadTrip,
    #[serde(rename = "Rodeo")]
    Rodeo,
    #[serde(rename = "Roll In")]
    RollIn,
    #[serde(rename = "Roll In - Audio Sync")]
    RollInAudioSync,
    #[serde(rename = "Rolling Bounces")]
    RollingBounces,
    #[serde(rename = "Rubber Band")]
    RubberBand,
    #[serde(rename = "Simple Zoom In")]
    SimpleZoomIn,
    #[serde(rename = "Simple Zoom Out")]
    SimpleZoomOut,
    #[serde(rename = "Slice Bounce")]
    SliceBounce,
    #[serde(rename = "Slideshow")]
    Slideshow,
    #[serde(rename = "Speed of Light")]
    SpeedOfLight,
    #[serde(rename = "Spin Bounce")]
    SpinBounce,
    #[serde(rename = "Sway Out")]
    SwayOut,
    #[serde(rename = "Sway Out - Audio Sync")]
    SwayOutAudioSync,
    #[serde(rename = "Tilt Down")]
    TiltDown,
    #[serde(rename = "Tilt Up")]
    TiltUp,
    #[serde(rename = "Traverse")]
    Traverse,
    #[serde(rename = "Tron")]
    Tron,
    #[serde(rename = "Vertigo")]
    Vertigo,
    #[serde(rename = "Vertigo - Audio Sync")]
    VertigoAudioSync,
    #[serde(rename = "Zoom In - Audio Sync")]
    ZoomInAudioSync,
    #[serde(rename = "Zoom In and Spin - Audio Sync")]
    ZoomInAndSpinAudioSync,
    #[serde(rename = "Zoom Out - Audio Sync")]
    ZoomOutAudioSync,
}
impl std::fmt::Display for PostV1AnimationBodyStyleCameraEffectEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1AnimationBodyStyleCameraEffectEnum::Accelerate => "Accelerate",
            PostV1AnimationBodyStyleCameraEffectEnum::AggressiveZoomInAudioSync => {
                "Aggressive Zoom In - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::AggressiveZoomOutAudioSync => {
                "Aggressive Zoom Out - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::BoostZoomIn => "Boost Zoom In",
            PostV1AnimationBodyStyleCameraEffectEnum::BoostZoomOut => "Boost Zoom Out",
            PostV1AnimationBodyStyleCameraEffectEnum::BounceInAndOut => {
                "Bounce In And Out"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::BounceOut => "Bounce Out",
            PostV1AnimationBodyStyleCameraEffectEnum::BounceOutAudioSync => {
                "Bounce Out - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::BounceAndSpinAudioSync => {
                "Bounce and Spin - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::BounceInPlace => "Bounce in Place",
            PostV1AnimationBodyStyleCameraEffectEnum::CogInTheMachine => {
                "Cog in the Machine"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::DevolveAudioSync => {
                "Devolve - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::DirectedByAi => "Directed by AI",
            PostV1AnimationBodyStyleCameraEffectEnum::DramaticZoomIn => {
                "Dramatic Zoom In"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::DramaticZoomOut => {
                "Dramatic Zoom Out"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::DriftSpin => "Drift Spin",
            PostV1AnimationBodyStyleCameraEffectEnum::EarthquakeBounce => {
                "Earthquake Bounce"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::EarthquakeBounceAudioSync => {
                "Earthquake Bounce - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::EvolveAudioSync => {
                "Evolve - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::Heartbeat => "Heartbeat",
            PostV1AnimationBodyStyleCameraEffectEnum::HesitateIn => "Hesitate In",
            PostV1AnimationBodyStyleCameraEffectEnum::Jump => "Jump",
            PostV1AnimationBodyStyleCameraEffectEnum::PanLeft => "Pan Left",
            PostV1AnimationBodyStyleCameraEffectEnum::PanRight => "Pan Right",
            PostV1AnimationBodyStyleCameraEffectEnum::PulseAudioSync => {
                "Pulse - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::Pusher => "Pusher",
            PostV1AnimationBodyStyleCameraEffectEnum::PusherAudioSync => {
                "Pusher - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::Quadrant => "Quadrant",
            PostV1AnimationBodyStyleCameraEffectEnum::RiseAndClimb => "Rise and Climb",
            PostV1AnimationBodyStyleCameraEffectEnum::RoadTrip => "Road Trip",
            PostV1AnimationBodyStyleCameraEffectEnum::Rodeo => "Rodeo",
            PostV1AnimationBodyStyleCameraEffectEnum::RollIn => "Roll In",
            PostV1AnimationBodyStyleCameraEffectEnum::RollInAudioSync => {
                "Roll In - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::RollingBounces => "Rolling Bounces",
            PostV1AnimationBodyStyleCameraEffectEnum::RubberBand => "Rubber Band",
            PostV1AnimationBodyStyleCameraEffectEnum::SimpleZoomIn => "Simple Zoom In",
            PostV1AnimationBodyStyleCameraEffectEnum::SimpleZoomOut => "Simple Zoom Out",
            PostV1AnimationBodyStyleCameraEffectEnum::SliceBounce => "Slice Bounce",
            PostV1AnimationBodyStyleCameraEffectEnum::Slideshow => "Slideshow",
            PostV1AnimationBodyStyleCameraEffectEnum::SpeedOfLight => "Speed of Light",
            PostV1AnimationBodyStyleCameraEffectEnum::SpinBounce => "Spin Bounce",
            PostV1AnimationBodyStyleCameraEffectEnum::SwayOut => "Sway Out",
            PostV1AnimationBodyStyleCameraEffectEnum::SwayOutAudioSync => {
                "Sway Out - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::TiltDown => "Tilt Down",
            PostV1AnimationBodyStyleCameraEffectEnum::TiltUp => "Tilt Up",
            PostV1AnimationBodyStyleCameraEffectEnum::Traverse => "Traverse",
            PostV1AnimationBodyStyleCameraEffectEnum::Tron => "Tron",
            PostV1AnimationBodyStyleCameraEffectEnum::Vertigo => "Vertigo",
            PostV1AnimationBodyStyleCameraEffectEnum::VertigoAudioSync => {
                "Vertigo - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::ZoomInAudioSync => {
                "Zoom In - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::ZoomInAndSpinAudioSync => {
                "Zoom In and Spin - Audio Sync"
            }
            PostV1AnimationBodyStyleCameraEffectEnum::ZoomOutAudioSync => {
                "Zoom Out - Audio Sync"
            }
        };
        write!(f, "{}", str_val)
    }
}
