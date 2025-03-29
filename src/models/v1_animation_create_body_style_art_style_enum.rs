/// The art style of the final output video
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AnimationCreateBodyStyleArtStyleEnum {
    #[default]
    #[serde(rename = "3D Render")]
    Enum3dRender,
    #[serde(rename = "90s Streets")]
    Enum90sStreets,
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
impl std::fmt::Display for V1AnimationCreateBodyStyleArtStyleEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AnimationCreateBodyStyleArtStyleEnum::Enum3dRender => "3D Render",
            V1AnimationCreateBodyStyleArtStyleEnum::Enum90sStreets => "90s Streets",
            V1AnimationCreateBodyStyleArtStyleEnum::AbstractMinimalist => {
                "Abstract Minimalist"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::Arcane => "Arcane",
            V1AnimationCreateBodyStyleArtStyleEnum::ArtDeco => "Art Deco",
            V1AnimationCreateBodyStyleArtStyleEnum::BoldColoredIllustration => {
                "Bold Colored Illustration"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::CinematicLandscape => {
                "Cinematic Landscape"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::CinematicMiyazaki => {
                "Cinematic Miyazaki"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::Cosmic => "Cosmic",
            V1AnimationCreateBodyStyleArtStyleEnum::Cubist => "Cubist",
            V1AnimationCreateBodyStyleArtStyleEnum::Custom => "Custom",
            V1AnimationCreateBodyStyleArtStyleEnum::Cyberpunk => "Cyberpunk",
            V1AnimationCreateBodyStyleArtStyleEnum::DarkGraphicIllustration => {
                "Dark Graphic Illustration"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::DarkWatercolor => "Dark Watercolor",
            V1AnimationCreateBodyStyleArtStyleEnum::DirectedByAi => "Directed by AI",
            V1AnimationCreateBodyStyleArtStyleEnum::DoubleExposure => "Double Exposure",
            V1AnimationCreateBodyStyleArtStyleEnum::FadedIllustration => {
                "Faded Illustration"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::Fantasy => "Fantasy",
            V1AnimationCreateBodyStyleArtStyleEnum::FuturisticAnime => "Futuristic Anime",
            V1AnimationCreateBodyStyleArtStyleEnum::Impressionism => "Impressionism",
            V1AnimationCreateBodyStyleArtStyleEnum::InkAndWatercolorPortrait => {
                "Ink and Watercolor Portrait"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::Inkpunk => "Inkpunk",
            V1AnimationCreateBodyStyleArtStyleEnum::IntricateAbstractLinesPortrait => {
                "Intricate Abstract Lines Portrait"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::JacksonPollock => "Jackson Pollock",
            V1AnimationCreateBodyStyleArtStyleEnum::LandscapePainting => {
                "Landscape Painting"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::LowPoly => "Low Poly",
            V1AnimationCreateBodyStyleArtStyleEnum::Miniatures => "Miniatures",
            V1AnimationCreateBodyStyleArtStyleEnum::MinimalColdFuturism => {
                "Minimal Cold Futurism"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::OilPainting => "Oil Painting",
            V1AnimationCreateBodyStyleArtStyleEnum::OldSchoolComic => "Old School Comic",
            V1AnimationCreateBodyStyleArtStyleEnum::Overgrown => "Overgrown",
            V1AnimationCreateBodyStyleArtStyleEnum::PaintedCityscape => {
                "Painted Cityscape"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::PainterlyIllustration => {
                "Painterly Illustration"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::Photograph => "Photograph",
            V1AnimationCreateBodyStyleArtStyleEnum::Pixar => "Pixar",
            V1AnimationCreateBodyStyleArtStyleEnum::PixelArt => "Pixel Art",
            V1AnimationCreateBodyStyleArtStyleEnum::Postapocalyptic => "Postapocalyptic",
            V1AnimationCreateBodyStyleArtStyleEnum::SinCity => "Sin City",
            V1AnimationCreateBodyStyleArtStyleEnum::SoftDelicateMattePortrait => {
                "Soft Delicate Matte Portrait"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::Spooky => "Spooky",
            V1AnimationCreateBodyStyleArtStyleEnum::StudioGhibliFilmStill => {
                "Studio Ghibli Film Still"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::Synthwave => "Synthwave",
            V1AnimationCreateBodyStyleArtStyleEnum::TraditionalWatercolor => {
                "Traditional Watercolor"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::VanGogh => "Van Gogh",
            V1AnimationCreateBodyStyleArtStyleEnum::VibrantMatteIllustration => {
                "Vibrant Matte Illustration"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::VintageJapaneseAnime => {
                "Vintage Japanese Anime"
            }
            V1AnimationCreateBodyStyleArtStyleEnum::Woodcut => "Woodcut",
        };
        write!(f, "{}", str_val)
    }
}
