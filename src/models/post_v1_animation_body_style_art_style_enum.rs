/// The art style of the final output video
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1AnimationBodyStyleArtStyleEnum {
    #[default]
    #[serde(rename = "3D Render")]
    _3dRender,
    #[serde(rename = "90s Streets")]
    _90sStreets,
    #[serde(rename = "Abstract Minimalist")]
    AbstractMinimalist,
    #[serde(rename = "Arcane")]
    Arcane,
    #[serde(rename = "Art Deco")]
    ArtDeco,
    #[serde(rename = "Bold Colored Illustration")]
    BoldColoredIllustration,
    #[serde(rename = "Cinematic Landscape")]
    CinematicLandscape,
    #[serde(rename = "Cinematic Miyazaki")]
    CinematicMiyazaki,
    #[serde(rename = "Cosmic")]
    Cosmic,
    #[serde(rename = "Cubist")]
    Cubist,
    #[serde(rename = "Custom")]
    Custom,
    #[serde(rename = "Cyberpunk")]
    Cyberpunk,
    #[serde(rename = "Dark Graphic Illustration")]
    DarkGraphicIllustration,
    #[serde(rename = "Dark Watercolor")]
    DarkWatercolor,
    #[serde(rename = "Directed by AI")]
    DirectedByAi,
    #[serde(rename = "Double Exposure")]
    DoubleExposure,
    #[serde(rename = "Faded Illustration")]
    FadedIllustration,
    #[serde(rename = "Fantasy")]
    Fantasy,
    #[serde(rename = "Futuristic Anime")]
    FuturisticAnime,
    #[serde(rename = "Impressionism")]
    Impressionism,
    #[serde(rename = "Ink and Watercolor Portrait")]
    InkAndWatercolorPortrait,
    #[serde(rename = "Inkpunk")]
    Inkpunk,
    #[serde(rename = "Intricate Abstract Lines Portrait")]
    IntricateAbstractLinesPortrait,
    #[serde(rename = "Jackson Pollock")]
    JacksonPollock,
    #[serde(rename = "Landscape Painting")]
    LandscapePainting,
    #[serde(rename = "Low Poly")]
    LowPoly,
    #[serde(rename = "Miniatures")]
    Miniatures,
    #[serde(rename = "Minimal Cold Futurism")]
    MinimalColdFuturism,
    #[serde(rename = "Oil Painting")]
    OilPainting,
    #[serde(rename = "Old School Comic")]
    OldSchoolComic,
    #[serde(rename = "Overgrown")]
    Overgrown,
    #[serde(rename = "Painted Cityscape")]
    PaintedCityscape,
    #[serde(rename = "Painterly Illustration")]
    PainterlyIllustration,
    #[serde(rename = "Photograph")]
    Photograph,
    #[serde(rename = "Pixar")]
    Pixar,
    #[serde(rename = "Pixel Art")]
    PixelArt,
    #[serde(rename = "Postapocalyptic")]
    Postapocalyptic,
    #[serde(rename = "Sin City")]
    SinCity,
    #[serde(rename = "Soft Delicate Matte Portrait")]
    SoftDelicateMattePortrait,
    #[serde(rename = "Spooky")]
    Spooky,
    #[serde(rename = "Studio Ghibli Film Still")]
    StudioGhibliFilmStill,
    #[serde(rename = "Synthwave")]
    Synthwave,
    #[serde(rename = "Traditional Watercolor")]
    TraditionalWatercolor,
    #[serde(rename = "Van Gogh")]
    VanGogh,
    #[serde(rename = "Vibrant Matte Illustration")]
    VibrantMatteIllustration,
    #[serde(rename = "Vintage Japanese Anime")]
    VintageJapaneseAnime,
    #[serde(rename = "Woodcut")]
    Woodcut,
}
impl std::fmt::Display for PostV1AnimationBodyStyleArtStyleEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1AnimationBodyStyleArtStyleEnum::_3dRender => "3D Render",
            PostV1AnimationBodyStyleArtStyleEnum::_90sStreets => "90s Streets",
            PostV1AnimationBodyStyleArtStyleEnum::AbstractMinimalist => {
                "Abstract Minimalist"
            }
            PostV1AnimationBodyStyleArtStyleEnum::Arcane => "Arcane",
            PostV1AnimationBodyStyleArtStyleEnum::ArtDeco => "Art Deco",
            PostV1AnimationBodyStyleArtStyleEnum::BoldColoredIllustration => {
                "Bold Colored Illustration"
            }
            PostV1AnimationBodyStyleArtStyleEnum::CinematicLandscape => {
                "Cinematic Landscape"
            }
            PostV1AnimationBodyStyleArtStyleEnum::CinematicMiyazaki => {
                "Cinematic Miyazaki"
            }
            PostV1AnimationBodyStyleArtStyleEnum::Cosmic => "Cosmic",
            PostV1AnimationBodyStyleArtStyleEnum::Cubist => "Cubist",
            PostV1AnimationBodyStyleArtStyleEnum::Custom => "Custom",
            PostV1AnimationBodyStyleArtStyleEnum::Cyberpunk => "Cyberpunk",
            PostV1AnimationBodyStyleArtStyleEnum::DarkGraphicIllustration => {
                "Dark Graphic Illustration"
            }
            PostV1AnimationBodyStyleArtStyleEnum::DarkWatercolor => "Dark Watercolor",
            PostV1AnimationBodyStyleArtStyleEnum::DirectedByAi => "Directed by AI",
            PostV1AnimationBodyStyleArtStyleEnum::DoubleExposure => "Double Exposure",
            PostV1AnimationBodyStyleArtStyleEnum::FadedIllustration => {
                "Faded Illustration"
            }
            PostV1AnimationBodyStyleArtStyleEnum::Fantasy => "Fantasy",
            PostV1AnimationBodyStyleArtStyleEnum::FuturisticAnime => "Futuristic Anime",
            PostV1AnimationBodyStyleArtStyleEnum::Impressionism => "Impressionism",
            PostV1AnimationBodyStyleArtStyleEnum::InkAndWatercolorPortrait => {
                "Ink and Watercolor Portrait"
            }
            PostV1AnimationBodyStyleArtStyleEnum::Inkpunk => "Inkpunk",
            PostV1AnimationBodyStyleArtStyleEnum::IntricateAbstractLinesPortrait => {
                "Intricate Abstract Lines Portrait"
            }
            PostV1AnimationBodyStyleArtStyleEnum::JacksonPollock => "Jackson Pollock",
            PostV1AnimationBodyStyleArtStyleEnum::LandscapePainting => {
                "Landscape Painting"
            }
            PostV1AnimationBodyStyleArtStyleEnum::LowPoly => "Low Poly",
            PostV1AnimationBodyStyleArtStyleEnum::Miniatures => "Miniatures",
            PostV1AnimationBodyStyleArtStyleEnum::MinimalColdFuturism => {
                "Minimal Cold Futurism"
            }
            PostV1AnimationBodyStyleArtStyleEnum::OilPainting => "Oil Painting",
            PostV1AnimationBodyStyleArtStyleEnum::OldSchoolComic => "Old School Comic",
            PostV1AnimationBodyStyleArtStyleEnum::Overgrown => "Overgrown",
            PostV1AnimationBodyStyleArtStyleEnum::PaintedCityscape => "Painted Cityscape",
            PostV1AnimationBodyStyleArtStyleEnum::PainterlyIllustration => {
                "Painterly Illustration"
            }
            PostV1AnimationBodyStyleArtStyleEnum::Photograph => "Photograph",
            PostV1AnimationBodyStyleArtStyleEnum::Pixar => "Pixar",
            PostV1AnimationBodyStyleArtStyleEnum::PixelArt => "Pixel Art",
            PostV1AnimationBodyStyleArtStyleEnum::Postapocalyptic => "Postapocalyptic",
            PostV1AnimationBodyStyleArtStyleEnum::SinCity => "Sin City",
            PostV1AnimationBodyStyleArtStyleEnum::SoftDelicateMattePortrait => {
                "Soft Delicate Matte Portrait"
            }
            PostV1AnimationBodyStyleArtStyleEnum::Spooky => "Spooky",
            PostV1AnimationBodyStyleArtStyleEnum::StudioGhibliFilmStill => {
                "Studio Ghibli Film Still"
            }
            PostV1AnimationBodyStyleArtStyleEnum::Synthwave => "Synthwave",
            PostV1AnimationBodyStyleArtStyleEnum::TraditionalWatercolor => {
                "Traditional Watercolor"
            }
            PostV1AnimationBodyStyleArtStyleEnum::VanGogh => "Van Gogh",
            PostV1AnimationBodyStyleArtStyleEnum::VibrantMatteIllustration => {
                "Vibrant Matte Illustration"
            }
            PostV1AnimationBodyStyleArtStyleEnum::VintageJapaneseAnime => {
                "Vintage Japanese Anime"
            }
            PostV1AnimationBodyStyleArtStyleEnum::Woodcut => "Woodcut",
        };
        write!(f, "{}", str_val)
    }
}
