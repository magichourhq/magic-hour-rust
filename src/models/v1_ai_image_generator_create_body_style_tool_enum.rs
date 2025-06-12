/// The art style to use for image generation. Defaults to 'general' if not provided.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageGeneratorCreateBodyStyleToolEnum {
    #[default]
    #[serde(rename = "ai-anime-generator")]
    AiAnimeGenerator,
    #[serde(rename = "ai-art-generator")]
    AiArtGenerator,
    #[serde(rename = "ai-background-generator")]
    AiBackgroundGenerator,
    #[serde(rename = "ai-character-generator")]
    AiCharacterGenerator,
    #[serde(rename = "ai-face-generator")]
    AiFaceGenerator,
    #[serde(rename = "ai-fashion-generator")]
    AiFashionGenerator,
    #[serde(rename = "ai-icon-generator")]
    AiIconGenerator,
    #[serde(rename = "ai-illustration-generator")]
    AiIllustrationGenerator,
    #[serde(rename = "ai-interior-design-generator")]
    AiInteriorDesignGenerator,
    #[serde(rename = "ai-landscape-generator")]
    AiLandscapeGenerator,
    #[serde(rename = "ai-logo-generator")]
    AiLogoGenerator,
    #[serde(rename = "ai-manga-generator")]
    AiMangaGenerator,
    #[serde(rename = "ai-outfit-generator")]
    AiOutfitGenerator,
    #[serde(rename = "ai-pattern-generator")]
    AiPatternGenerator,
    #[serde(rename = "ai-photo-generator")]
    AiPhotoGenerator,
    #[serde(rename = "ai-sketch-generator")]
    AiSketchGenerator,
    #[serde(rename = "ai-tattoo-generator")]
    AiTattooGenerator,
    #[serde(rename = "album-cover-generator")]
    AlbumCoverGenerator,
    #[serde(rename = "animated-characters-generator")]
    AnimatedCharactersGenerator,
    #[serde(rename = "architecture-generator")]
    ArchitectureGenerator,
    #[serde(rename = "book-cover-generator")]
    BookCoverGenerator,
    #[serde(rename = "comic-book-generator")]
    ComicBookGenerator,
    #[serde(rename = "dark-fantasy-ai")]
    DarkFantasyAi,
    #[serde(rename = "disney-ai-generator")]
    DisneyAiGenerator,
    #[serde(rename = "dnd-ai-art-generator")]
    DndAiArtGenerator,
    #[serde(rename = "emoji-generator")]
    EmojiGenerator,
    #[serde(rename = "fantasy-map-generator")]
    FantasyMapGenerator,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "graffiti-generator")]
    GraffitiGenerator,
    #[serde(rename = "movie-poster-generator")]
    MoviePosterGenerator,
    #[serde(rename = "optical-illusion-generator")]
    OpticalIllusionGenerator,
    #[serde(rename = "pokemon-generator")]
    PokemonGenerator,
    #[serde(rename = "south-park-character-generator")]
    SouthParkCharacterGenerator,
    #[serde(rename = "superhero-generator")]
    SuperheroGenerator,
    #[serde(rename = "thumbnail-maker")]
    ThumbnailMaker,
}
impl std::fmt::Display for V1AiImageGeneratorCreateBodyStyleToolEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiAnimeGenerator => {
                "ai-anime-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiArtGenerator => {
                "ai-art-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiBackgroundGenerator => {
                "ai-background-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiCharacterGenerator => {
                "ai-character-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiFaceGenerator => {
                "ai-face-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiFashionGenerator => {
                "ai-fashion-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiIconGenerator => {
                "ai-icon-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiIllustrationGenerator => {
                "ai-illustration-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiInteriorDesignGenerator => {
                "ai-interior-design-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiLandscapeGenerator => {
                "ai-landscape-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiLogoGenerator => {
                "ai-logo-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiMangaGenerator => {
                "ai-manga-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiOutfitGenerator => {
                "ai-outfit-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiPatternGenerator => {
                "ai-pattern-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiPhotoGenerator => {
                "ai-photo-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiSketchGenerator => {
                "ai-sketch-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AiTattooGenerator => {
                "ai-tattoo-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AlbumCoverGenerator => {
                "album-cover-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::AnimatedCharactersGenerator => {
                "animated-characters-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::ArchitectureGenerator => {
                "architecture-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::BookCoverGenerator => {
                "book-cover-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::ComicBookGenerator => {
                "comic-book-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::DarkFantasyAi => "dark-fantasy-ai",
            V1AiImageGeneratorCreateBodyStyleToolEnum::DisneyAiGenerator => {
                "disney-ai-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::DndAiArtGenerator => {
                "dnd-ai-art-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::EmojiGenerator => {
                "emoji-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::FantasyMapGenerator => {
                "fantasy-map-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::General => "general",
            V1AiImageGeneratorCreateBodyStyleToolEnum::GraffitiGenerator => {
                "graffiti-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::MoviePosterGenerator => {
                "movie-poster-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::OpticalIllusionGenerator => {
                "optical-illusion-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::PokemonGenerator => {
                "pokemon-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::SouthParkCharacterGenerator => {
                "south-park-character-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::SuperheroGenerator => {
                "superhero-generator"
            }
            V1AiImageGeneratorCreateBodyStyleToolEnum::ThumbnailMaker => {
                "thumbnail-maker"
            }
        };
        write!(f, "{}", str_val)
    }
}
