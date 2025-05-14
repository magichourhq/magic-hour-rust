/// V1VideoToVideoCreateBodyStyleArtStyleEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1VideoToVideoCreateBodyStyleArtStyleEnum {
    #[default]
    #[serde(rename = "3D Render")]
    Enum3dRender,
    #[serde(rename = "Airbender")]
    Airbender,
    #[serde(rename = "Android")]
    Android,
    #[serde(rename = "Anime Warrior")]
    AnimeWarrior,
    #[serde(rename = "Armored Knight")]
    ArmoredKnight,
    #[serde(rename = "Assassin's Creed")]
    AssassinSCreed,
    #[serde(rename = "Avatar")]
    Avatar,
    #[serde(rename = "Black Spiderman")]
    BlackSpiderman,
    #[serde(rename = "Boba Fett")]
    BobaFett,
    #[serde(rename = "Celestial Skin")]
    CelestialSkin,
    #[serde(rename = "Chinese Swordsmen")]
    ChineseSwordsmen,
    #[serde(rename = "Clay")]
    Clay,
    #[serde(rename = "Comic")]
    Comic,
    #[serde(rename = "Cyberpunk")]
    Cyberpunk,
    #[serde(rename = "Cypher")]
    Cypher,
    #[serde(rename = "Dark Fantasy")]
    DarkFantasy,
    #[serde(rename = "Dragonball Z")]
    DragonballZ,
    #[serde(rename = "Future Bot")]
    FutureBot,
    #[serde(rename = "Futuristic Fantasy")]
    FuturisticFantasy,
    #[serde(rename = "GTA")]
    Gta,
    #[serde(rename = "Ghost")]
    Ghost,
    #[serde(rename = "Gundam")]
    Gundam,
    #[serde(rename = "Hologram")]
    Hologram,
    #[serde(rename = "Illustration")]
    Illustration,
    #[serde(rename = "Impressionism")]
    Impressionism,
    #[serde(rename = "Ink")]
    Ink,
    #[serde(rename = "Ink Poster")]
    InkPoster,
    #[serde(rename = "Jinx")]
    Jinx,
    #[serde(rename = "Knight")]
    Knight,
    #[serde(rename = "Lego")]
    Lego,
    #[serde(rename = "Link")]
    Link,
    #[serde(rename = "Marble")]
    Marble,
    #[serde(rename = "Mario")]
    Mario,
    #[serde(rename = "Master Chief")]
    MasterChief,
    #[serde(rename = "Mech")]
    Mech,
    #[serde(rename = "Minecraft")]
    Minecraft,
    #[serde(rename = "Mystique")]
    Mystique,
    #[serde(rename = "Naruto")]
    Naruto,
    #[serde(rename = "Neon Dream")]
    NeonDream,
    #[serde(rename = "No Art Style")]
    NoArtStyle,
    #[serde(rename = "Oil Painting")]
    OilPainting,
    #[serde(rename = "On Fire")]
    OnFire,
    #[serde(rename = "Origami")]
    Origami,
    #[serde(rename = "Pixar")]
    Pixar,
    #[serde(rename = "Pixel")]
    Pixel,
    #[serde(rename = "Power Armor")]
    PowerArmor,
    #[serde(rename = "Power Ranger")]
    PowerRanger,
    #[serde(rename = "Retro Anime")]
    RetroAnime,
    #[serde(rename = "Retro Sci-Fi")]
    RetroSciFi,
    #[serde(rename = "Samurai")]
    Samurai,
    #[serde(rename = "Samurai Bot")]
    SamuraiBot,
    #[serde(rename = "Solid Snake")]
    SolidSnake,
    #[serde(rename = "Spartan")]
    Spartan,
    #[serde(rename = "Starfield")]
    Starfield,
    #[serde(rename = "Street Fighter")]
    StreetFighter,
    #[serde(rename = "Studio Ghibli")]
    StudioGhibli,
    #[serde(rename = "Sub-Zero")]
    SubZero,
    #[serde(rename = "The Void")]
    TheVoid,
    #[serde(rename = "Tomb Raider")]
    TombRaider,
    #[serde(rename = "Underwater")]
    Underwater,
    #[serde(rename = "Van Gogh")]
    VanGogh,
    #[serde(rename = "Viking")]
    Viking,
    #[serde(rename = "Watercolor")]
    Watercolor,
    #[serde(rename = "Wu Kong")]
    WuKong,
    #[serde(rename = "Zelda")]
    Zelda,
}
impl std::fmt::Display for V1VideoToVideoCreateBodyStyleArtStyleEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Enum3dRender => "3D Render",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Airbender => "Airbender",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Android => "Android",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::AnimeWarrior => "Anime Warrior",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::ArmoredKnight => "Armored Knight",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::AssassinSCreed => {
                "Assassin's Creed"
            }
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Avatar => "Avatar",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::BlackSpiderman => {
                "Black Spiderman"
            }
            V1VideoToVideoCreateBodyStyleArtStyleEnum::BobaFett => "Boba Fett",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::CelestialSkin => "Celestial Skin",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::ChineseSwordsmen => {
                "Chinese Swordsmen"
            }
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Clay => "Clay",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Comic => "Comic",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Cyberpunk => "Cyberpunk",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Cypher => "Cypher",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::DarkFantasy => "Dark Fantasy",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::DragonballZ => "Dragonball Z",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::FutureBot => "Future Bot",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::FuturisticFantasy => {
                "Futuristic Fantasy"
            }
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Gta => "GTA",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Ghost => "Ghost",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Gundam => "Gundam",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Hologram => "Hologram",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Illustration => "Illustration",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Impressionism => "Impressionism",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Ink => "Ink",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::InkPoster => "Ink Poster",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Jinx => "Jinx",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Knight => "Knight",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Lego => "Lego",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Link => "Link",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Marble => "Marble",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Mario => "Mario",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::MasterChief => "Master Chief",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Mech => "Mech",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Minecraft => "Minecraft",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Mystique => "Mystique",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Naruto => "Naruto",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::NeonDream => "Neon Dream",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::NoArtStyle => "No Art Style",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::OilPainting => "Oil Painting",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::OnFire => "On Fire",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Origami => "Origami",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Pixar => "Pixar",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Pixel => "Pixel",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::PowerArmor => "Power Armor",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::PowerRanger => "Power Ranger",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::RetroAnime => "Retro Anime",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::RetroSciFi => "Retro Sci-Fi",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Samurai => "Samurai",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::SamuraiBot => "Samurai Bot",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::SolidSnake => "Solid Snake",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Spartan => "Spartan",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Starfield => "Starfield",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::StreetFighter => "Street Fighter",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::StudioGhibli => "Studio Ghibli",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::SubZero => "Sub-Zero",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::TheVoid => "The Void",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::TombRaider => "Tomb Raider",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Underwater => "Underwater",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::VanGogh => "Van Gogh",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Viking => "Viking",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Watercolor => "Watercolor",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::WuKong => "Wu Kong",
            V1VideoToVideoCreateBodyStyleArtStyleEnum::Zelda => "Zelda",
        };
        write!(f, "{}", str_val)
    }
}
