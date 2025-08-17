/// The camera effect used to create the output video
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AnimationCreateBodyStyleCameraEffectEnum {
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
impl std::fmt::Display for V1AnimationCreateBodyStyleCameraEffectEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AnimationCreateBodyStyleCameraEffectEnum::Accelerate => "Accelerate",
            V1AnimationCreateBodyStyleCameraEffectEnum::AggressiveZoomInAudioSync => {
                "Aggressive Zoom In - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::AggressiveZoomOutAudioSync => {
                "Aggressive Zoom Out - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::BoostZoomIn => "Boost Zoom In",
            V1AnimationCreateBodyStyleCameraEffectEnum::BoostZoomOut => "Boost Zoom Out",
            V1AnimationCreateBodyStyleCameraEffectEnum::BounceInAndOut => {
                "Bounce In And Out"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::BounceOut => "Bounce Out",
            V1AnimationCreateBodyStyleCameraEffectEnum::BounceOutAudioSync => {
                "Bounce Out - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::BounceAndSpinAudioSync => {
                "Bounce and Spin - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::BounceInPlace => {
                "Bounce in Place"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::CogInTheMachine => {
                "Cog in the Machine"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::DevolveAudioSync => {
                "Devolve - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::DirectedByAi => "Directed by AI",
            V1AnimationCreateBodyStyleCameraEffectEnum::DramaticZoomIn => {
                "Dramatic Zoom In"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::DramaticZoomOut => {
                "Dramatic Zoom Out"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::DriftSpin => "Drift Spin",
            V1AnimationCreateBodyStyleCameraEffectEnum::EarthquakeBounce => {
                "Earthquake Bounce"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::EarthquakeBounceAudioSync => {
                "Earthquake Bounce - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::EvolveAudioSync => {
                "Evolve - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::Heartbeat => "Heartbeat",
            V1AnimationCreateBodyStyleCameraEffectEnum::HesitateIn => "Hesitate In",
            V1AnimationCreateBodyStyleCameraEffectEnum::Jump => "Jump",
            V1AnimationCreateBodyStyleCameraEffectEnum::PanLeft => "Pan Left",
            V1AnimationCreateBodyStyleCameraEffectEnum::PanRight => "Pan Right",
            V1AnimationCreateBodyStyleCameraEffectEnum::PulseAudioSync => {
                "Pulse - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::Pusher => "Pusher",
            V1AnimationCreateBodyStyleCameraEffectEnum::PusherAudioSync => {
                "Pusher - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::Quadrant => "Quadrant",
            V1AnimationCreateBodyStyleCameraEffectEnum::RiseAndClimb => "Rise and Climb",
            V1AnimationCreateBodyStyleCameraEffectEnum::RoadTrip => "Road Trip",
            V1AnimationCreateBodyStyleCameraEffectEnum::Rodeo => "Rodeo",
            V1AnimationCreateBodyStyleCameraEffectEnum::RollIn => "Roll In",
            V1AnimationCreateBodyStyleCameraEffectEnum::RollInAudioSync => {
                "Roll In - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::RollingBounces => {
                "Rolling Bounces"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::RubberBand => "Rubber Band",
            V1AnimationCreateBodyStyleCameraEffectEnum::SimpleZoomIn => "Simple Zoom In",
            V1AnimationCreateBodyStyleCameraEffectEnum::SimpleZoomOut => {
                "Simple Zoom Out"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::SliceBounce => "Slice Bounce",
            V1AnimationCreateBodyStyleCameraEffectEnum::Slideshow => "Slideshow",
            V1AnimationCreateBodyStyleCameraEffectEnum::SpeedOfLight => "Speed of Light",
            V1AnimationCreateBodyStyleCameraEffectEnum::SpinBounce => "Spin Bounce",
            V1AnimationCreateBodyStyleCameraEffectEnum::SwayOut => "Sway Out",
            V1AnimationCreateBodyStyleCameraEffectEnum::SwayOutAudioSync => {
                "Sway Out - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::TiltDown => "Tilt Down",
            V1AnimationCreateBodyStyleCameraEffectEnum::TiltUp => "Tilt Up",
            V1AnimationCreateBodyStyleCameraEffectEnum::Traverse => "Traverse",
            V1AnimationCreateBodyStyleCameraEffectEnum::Tron => "Tron",
            V1AnimationCreateBodyStyleCameraEffectEnum::Vertigo => "Vertigo",
            V1AnimationCreateBodyStyleCameraEffectEnum::VertigoAudioSync => {
                "Vertigo - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::ZoomInAudioSync => {
                "Zoom In - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::ZoomInAndSpinAudioSync => {
                "Zoom In and Spin - Audio Sync"
            }
            V1AnimationCreateBodyStyleCameraEffectEnum::ZoomOutAudioSync => {
                "Zoom Out - Audio Sync"
            }
        };
        write!(f, "{}", str_val)
    }
}
