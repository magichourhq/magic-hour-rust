/// The voice to use for the speech. Available voices: Elon Musk, Mark Zuckerberg, Joe Rogan, Barack Obama, Morgan Freeman, Kanye West, Donald Trump, Joe Biden, Kim Kardashian, Taylor Swift, James Earl Jones, Samuel L. Jackson, Jeff Goldblum, David Attenborough, Sean Connery, Cillian Murphy, Anne Hathaway, Julia Roberts, Natalie Portman, Steve Carell, Amy Poehler, Stephen Colbert, Jimmy Fallon, David Letterman, Alex Trebek, Katy Perry, Prince, Kevin Bacon, Tom Hiddleston, Adam Driver, Alan Rickman, Alexz Johnson, Ana Gasteyer, Andrew Rannells, Arden Cho, Bear Grylls, Ben McKenzie, Ben Stiller, Ben Whishaw, Billie Joe Armstrong, Bingbing Li, Bob Barker, Booboo Stewart, Bradley Steven Perry, Bruno Mars, Caity Lotz, Cameron Boyce, Candice Accola, Carrie Underwood, Casey Affleck, Caterina Scorsone, Cedric the Entertainer, Chace Crawford, Chadwick Boseman, Charlie Day, Chris Hemsworth, Chris Martin, Christopher Mintz-Plasse, Dan Fogler, Dan Stevens, Daniel Dae Kim, Danielle Panabaker, Dave Bautista, David Schwimmer, Denis Leary, Derek Mears, Diego Luna, Donald Glover, Donnie Yen, Doutzen Kroes, Dove Cameron, Dr. Dre, Drake Bell, Elle Fanning, Ernie Hudson, Fergie, Forest Whitaker, Francia Raisa, Freddie Highmore, Gillian Jacobs, Gina Carano, Ginnifer Goodwin, Gordon Ramsay, Guy Pearce, Gwendoline Christie, Hailee Steinfeld, Howie Mandel, Hugh Jackman, Hugh Laurie, J. K. Simmons, Jack Black, Jared Leto, Jennifer Carpenter, Kesha, Kris Jenner, Kristen Bell, Lorde, Matt Smith, Marilyn Monroe, Charlie Chaplin, Albert Einstein, Abraham Lincoln, John F. Kennedy, Lucille Ball, A.R. Rahman, Aamir Khan, Ajay Devgn, Akshay Kumar, Alain Delon, Alan Alda, Alan Cumming, Amitabh Bachchan, Ang Lee, Ansel Elgort, Anthony Anderson, Anthony Mackie, Armie Hammer, Asa Butterfield, B.J. Novak, Barbara Eden, Betty White, Bill Nighy, Bill Pullman, Blake Shelton, Bonnie Wright, Brad Paisley, Brendan Gleeson, Brian Cox, Bruno Ganz, Burt Reynolds, Carrie Fisher, Charles Dance, Chiwetel Ejiofor, Chris Pine, Christina Hendricks, Christina Ricci, Cyndi Lauper, Dakota Fanning, Damian Lewis, Dan Aykroyd, Daniel Craig, David Oyelowo, David Tennant, Diane Keaton, Diane Kruger, Dick Van Dyke, Domhnall Gleeson, Dominic Cooper, Donald Sutherland, Drew Carey, Eartha Kitt, Eddie Izzard, Edward Asner, Eli Roth, Elisabeth Moss, Ellen Burstyn, Emile Hirsch, Ezra Miller, Felicity Jones, Fiona Shaw, Florence Henderson, Freida Pinto, Geena Davis, Gemma Arterton, Geri Halliwell, Glenn Close, Gloria Steinem, Greta Gerwig, Gugu Mbatha-Raw, Hans Zimmer, Harry Connick Jr., Harvey Keitel, Helena Bonham Carter, Henry Cavill, Hilary Swank, Hugh Bonneville, Idina Menzel, Imelda Staunton, Ingrid Bergman, Irrfan Khan, Isla Fisher, Iwan Rheon, Jack Lemmon, Janet Jackson, Jason Bateman, Jason Segel, Jennifer Coolidge, Johnny Galecki, Jon Favreau, Joseph Gordon-Levitt, Josh Brolin, Josh Gad, Josh Groban, Julia Louis-Dreyfus, Kristen Stewart, Kristen Wiig, Rooney Mara, Caitriona Balfe, J.J. Abrams, Zoe Saldana
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum {
    #[default]
    #[serde(rename = "A.R. Rahman")]
    ARRahman,
    #[serde(rename = "Aamir Khan")]
    AamirKhan,
    #[serde(rename = "Abraham Lincoln")]
    AbrahamLincoln,
    #[serde(rename = "Adam Driver")]
    AdamDriver,
    #[serde(rename = "Ajay Devgn")]
    AjayDevgn,
    #[serde(rename = "Akshay Kumar")]
    AkshayKumar,
    #[serde(rename = "Alain Delon")]
    AlainDelon,
    #[serde(rename = "Alan Alda")]
    AlanAlda,
    #[serde(rename = "Alan Cumming")]
    AlanCumming,
    #[serde(rename = "Alan Rickman")]
    AlanRickman,
    #[serde(rename = "Albert Einstein")]
    AlbertEinstein,
    #[serde(rename = "Alex Trebek")]
    AlexTrebek,
    #[serde(rename = "Alexz Johnson")]
    AlexzJohnson,
    #[serde(rename = "Amitabh Bachchan")]
    AmitabhBachchan,
    #[serde(rename = "Amy Poehler")]
    AmyPoehler,
    #[serde(rename = "Ana Gasteyer")]
    AnaGasteyer,
    #[serde(rename = "Andrew Rannells")]
    AndrewRannells,
    #[serde(rename = "Ang Lee")]
    AngLee,
    #[serde(rename = "Anne Hathaway")]
    AnneHathaway,
    #[serde(rename = "Ansel Elgort")]
    AnselElgort,
    #[serde(rename = "Anthony Anderson")]
    AnthonyAnderson,
    #[serde(rename = "Anthony Mackie")]
    AnthonyMackie,
    #[serde(rename = "Arden Cho")]
    ArdenCho,
    #[serde(rename = "Armie Hammer")]
    ArmieHammer,
    #[serde(rename = "Asa Butterfield")]
    AsaButterfield,
    #[serde(rename = "B.J. Novak")]
    BJNovak,
    #[serde(rename = "Barack Obama")]
    BarackObama,
    #[serde(rename = "Barbara Eden")]
    BarbaraEden,
    #[serde(rename = "Bear Grylls")]
    BearGrylls,
    #[serde(rename = "Ben McKenzie")]
    BenMcKenzie,
    #[serde(rename = "Ben Stiller")]
    BenStiller,
    #[serde(rename = "Ben Whishaw")]
    BenWhishaw,
    #[serde(rename = "Betty White")]
    BettyWhite,
    #[serde(rename = "Bill Nighy")]
    BillNighy,
    #[serde(rename = "Bill Pullman")]
    BillPullman,
    #[serde(rename = "Billie Joe Armstrong")]
    BillieJoeArmstrong,
    #[serde(rename = "Bingbing Li")]
    BingbingLi,
    #[serde(rename = "Blake Shelton")]
    BlakeShelton,
    #[serde(rename = "Bob Barker")]
    BobBarker,
    #[serde(rename = "Bonnie Wright")]
    BonnieWright,
    #[serde(rename = "Booboo Stewart")]
    BoobooStewart,
    #[serde(rename = "Brad Paisley")]
    BradPaisley,
    #[serde(rename = "Bradley Steven Perry")]
    BradleyStevenPerry,
    #[serde(rename = "Brendan Gleeson")]
    BrendanGleeson,
    #[serde(rename = "Brian Cox")]
    BrianCox,
    #[serde(rename = "Bruno Ganz")]
    BrunoGanz,
    #[serde(rename = "Bruno Mars")]
    BrunoMars,
    #[serde(rename = "Burt Reynolds")]
    BurtReynolds,
    #[serde(rename = "Caitriona Balfe")]
    CaitrionaBalfe,
    #[serde(rename = "Caity Lotz")]
    CaityLotz,
    #[serde(rename = "Cameron Boyce")]
    CameronBoyce,
    #[serde(rename = "Candice Accola")]
    CandiceAccola,
    #[serde(rename = "Carrie Fisher")]
    CarrieFisher,
    #[serde(rename = "Carrie Underwood")]
    CarrieUnderwood,
    #[serde(rename = "Casey Affleck")]
    CaseyAffleck,
    #[serde(rename = "Caterina Scorsone")]
    CaterinaScorsone,
    #[serde(rename = "Cedric the Entertainer")]
    CedricTheEntertainer,
    #[serde(rename = "Chace Crawford")]
    ChaceCrawford,
    #[serde(rename = "Chadwick Boseman")]
    ChadwickBoseman,
    #[serde(rename = "Charles Dance")]
    CharlesDance,
    #[serde(rename = "Charlie Chaplin")]
    CharlieChaplin,
    #[serde(rename = "Charlie Day")]
    CharlieDay,
    #[serde(rename = "Chiwetel Ejiofor")]
    ChiwetelEjiofor,
    #[serde(rename = "Chris Hemsworth")]
    ChrisHemsworth,
    #[serde(rename = "Chris Martin")]
    ChrisMartin,
    #[serde(rename = "Chris Pine")]
    ChrisPine,
    #[serde(rename = "Christina Hendricks")]
    ChristinaHendricks,
    #[serde(rename = "Christina Ricci")]
    ChristinaRicci,
    #[serde(rename = "Christopher Mintz-Plasse")]
    ChristopherMintzPlasse,
    #[serde(rename = "Cillian Murphy")]
    CillianMurphy,
    #[serde(rename = "Cyndi Lauper")]
    CyndiLauper,
    #[serde(rename = "Dakota Fanning")]
    DakotaFanning,
    #[serde(rename = "Damian Lewis")]
    DamianLewis,
    #[serde(rename = "Dan Aykroyd")]
    DanAykroyd,
    #[serde(rename = "Dan Fogler")]
    DanFogler,
    #[serde(rename = "Dan Stevens")]
    DanStevens,
    #[serde(rename = "Daniel Craig")]
    DanielCraig,
    #[serde(rename = "Daniel Dae Kim")]
    DanielDaeKim,
    #[serde(rename = "Danielle Panabaker")]
    DaniellePanabaker,
    #[serde(rename = "Dave Bautista")]
    DaveBautista,
    #[serde(rename = "David Attenborough")]
    DavidAttenborough,
    #[serde(rename = "David Letterman")]
    DavidLetterman,
    #[serde(rename = "David Oyelowo")]
    DavidOyelowo,
    #[serde(rename = "David Schwimmer")]
    DavidSchwimmer,
    #[serde(rename = "David Tennant")]
    DavidTennant,
    #[serde(rename = "Denis Leary")]
    DenisLeary,
    #[serde(rename = "Derek Mears")]
    DerekMears,
    #[serde(rename = "Diane Keaton")]
    DianeKeaton,
    #[serde(rename = "Diane Kruger")]
    DianeKruger,
    #[serde(rename = "Dick Van Dyke")]
    DickVanDyke,
    #[serde(rename = "Diego Luna")]
    DiegoLuna,
    #[serde(rename = "Domhnall Gleeson")]
    DomhnallGleeson,
    #[serde(rename = "Dominic Cooper")]
    DominicCooper,
    #[serde(rename = "Donald Glover")]
    DonaldGlover,
    #[serde(rename = "Donald Sutherland")]
    DonaldSutherland,
    #[serde(rename = "Donald Trump")]
    DonaldTrump,
    #[serde(rename = "Donnie Yen")]
    DonnieYen,
    #[serde(rename = "Doutzen Kroes")]
    DoutzenKroes,
    #[serde(rename = "Dove Cameron")]
    DoveCameron,
    #[serde(rename = "Dr. Dre")]
    DrDre,
    #[serde(rename = "Drake Bell")]
    DrakeBell,
    #[serde(rename = "Drew Carey")]
    DrewCarey,
    #[serde(rename = "Eartha Kitt")]
    EarthaKitt,
    #[serde(rename = "Eddie Izzard")]
    EddieIzzard,
    #[serde(rename = "Edward Asner")]
    EdwardAsner,
    #[serde(rename = "Eli Roth")]
    EliRoth,
    #[serde(rename = "Elisabeth Moss")]
    ElisabethMoss,
    #[serde(rename = "Elle Fanning")]
    ElleFanning,
    #[serde(rename = "Ellen Burstyn")]
    EllenBurstyn,
    #[serde(rename = "Elon Musk")]
    ElonMusk,
    #[serde(rename = "Emile Hirsch")]
    EmileHirsch,
    #[serde(rename = "Ernie Hudson")]
    ErnieHudson,
    #[serde(rename = "Ezra Miller")]
    EzraMiller,
    #[serde(rename = "Felicity Jones")]
    FelicityJones,
    #[serde(rename = "Fergie")]
    Fergie,
    #[serde(rename = "Fiona Shaw")]
    FionaShaw,
    #[serde(rename = "Florence Henderson")]
    FlorenceHenderson,
    #[serde(rename = "Forest Whitaker")]
    ForestWhitaker,
    #[serde(rename = "Francia Raisa")]
    FranciaRaisa,
    #[serde(rename = "Freddie Highmore")]
    FreddieHighmore,
    #[serde(rename = "Freida Pinto")]
    FreidaPinto,
    #[serde(rename = "Geena Davis")]
    GeenaDavis,
    #[serde(rename = "Gemma Arterton")]
    GemmaArterton,
    #[serde(rename = "Geri Halliwell")]
    GeriHalliwell,
    #[serde(rename = "Gillian Jacobs")]
    GillianJacobs,
    #[serde(rename = "Gina Carano")]
    GinaCarano,
    #[serde(rename = "Ginnifer Goodwin")]
    GinniferGoodwin,
    #[serde(rename = "Glenn Close")]
    GlennClose,
    #[serde(rename = "Gloria Steinem")]
    GloriaSteinem,
    #[serde(rename = "Gordon Ramsay")]
    GordonRamsay,
    #[serde(rename = "Greta Gerwig")]
    GretaGerwig,
    #[serde(rename = "Gugu Mbatha-Raw")]
    GuguMbathaRaw,
    #[serde(rename = "Guy Pearce")]
    GuyPearce,
    #[serde(rename = "Gwendoline Christie")]
    GwendolineChristie,
    #[serde(rename = "Hailee Steinfeld")]
    HaileeSteinfeld,
    #[serde(rename = "Hans Zimmer")]
    HansZimmer,
    #[serde(rename = "Harry Connick Jr.")]
    HarryConnickJr,
    #[serde(rename = "Harvey Keitel")]
    HarveyKeitel,
    #[serde(rename = "Helena Bonham Carter")]
    HelenaBonhamCarter,
    #[serde(rename = "Henry Cavill")]
    HenryCavill,
    #[serde(rename = "Hilary Swank")]
    HilarySwank,
    #[serde(rename = "Howie Mandel")]
    HowieMandel,
    #[serde(rename = "Hugh Bonneville")]
    HughBonneville,
    #[serde(rename = "Hugh Jackman")]
    HughJackman,
    #[serde(rename = "Hugh Laurie")]
    HughLaurie,
    #[serde(rename = "Idina Menzel")]
    IdinaMenzel,
    #[serde(rename = "Imelda Staunton")]
    ImeldaStaunton,
    #[serde(rename = "Ingrid Bergman")]
    IngridBergman,
    #[serde(rename = "Irrfan Khan")]
    IrrfanKhan,
    #[serde(rename = "Isla Fisher")]
    IslaFisher,
    #[serde(rename = "Iwan Rheon")]
    IwanRheon,
    #[serde(rename = "J. K. Simmons")]
    JKSimmons,
    #[serde(rename = "J.J. Abrams")]
    JJAbrams,
    #[serde(rename = "Jack Black")]
    JackBlack,
    #[serde(rename = "Jack Lemmon")]
    JackLemmon,
    #[serde(rename = "James Earl Jones")]
    JamesEarlJones,
    #[serde(rename = "Janet Jackson")]
    JanetJackson,
    #[serde(rename = "Jared Leto")]
    JaredLeto,
    #[serde(rename = "Jason Bateman")]
    JasonBateman,
    #[serde(rename = "Jason Segel")]
    JasonSegel,
    #[serde(rename = "Jeff Goldblum")]
    JeffGoldblum,
    #[serde(rename = "Jennifer Carpenter")]
    JenniferCarpenter,
    #[serde(rename = "Jennifer Coolidge")]
    JenniferCoolidge,
    #[serde(rename = "Jimmy Fallon")]
    JimmyFallon,
    #[serde(rename = "Joe Biden")]
    JoeBiden,
    #[serde(rename = "Joe Rogan")]
    JoeRogan,
    #[serde(rename = "John F. Kennedy")]
    JohnFKennedy,
    #[serde(rename = "Johnny Galecki")]
    JohnnyGalecki,
    #[serde(rename = "Jon Favreau")]
    JonFavreau,
    #[serde(rename = "Joseph Gordon-Levitt")]
    JosephGordonLevitt,
    #[serde(rename = "Josh Brolin")]
    JoshBrolin,
    #[serde(rename = "Josh Gad")]
    JoshGad,
    #[serde(rename = "Josh Groban")]
    JoshGroban,
    #[serde(rename = "Julia Louis-Dreyfus")]
    JuliaLouisDreyfus,
    #[serde(rename = "Julia Roberts")]
    JuliaRoberts,
    #[serde(rename = "Kanye West")]
    KanyeWest,
    #[serde(rename = "Katy Perry")]
    KatyPerry,
    #[serde(rename = "Kesha")]
    Kesha,
    #[serde(rename = "Kevin Bacon")]
    KevinBacon,
    #[serde(rename = "Kim Kardashian")]
    KimKardashian,
    #[serde(rename = "Kris Jenner")]
    KrisJenner,
    #[serde(rename = "Kristen Bell")]
    KristenBell,
    #[serde(rename = "Kristen Stewart")]
    KristenStewart,
    #[serde(rename = "Kristen Wiig")]
    KristenWiig,
    #[serde(rename = "Lorde")]
    Lorde,
    #[serde(rename = "Lucille Ball")]
    LucilleBall,
    #[serde(rename = "Marilyn Monroe")]
    MarilynMonroe,
    #[serde(rename = "Mark Zuckerberg")]
    MarkZuckerberg,
    #[serde(rename = "Matt Smith")]
    MattSmith,
    #[serde(rename = "Morgan Freeman")]
    MorganFreeman,
    #[serde(rename = "Natalie Portman")]
    NataliePortman,
    #[serde(rename = "Prince")]
    Prince,
    #[serde(rename = "Rooney Mara")]
    RooneyMara,
    #[serde(rename = "Samuel L. Jackson")]
    SamuelLJackson,
    #[serde(rename = "Sean Connery")]
    SeanConnery,
    #[serde(rename = "Stephen Colbert")]
    StephenColbert,
    #[serde(rename = "Steve Carell")]
    SteveCarell,
    #[serde(rename = "Taylor Swift")]
    TaylorSwift,
    #[serde(rename = "Tom Hiddleston")]
    TomHiddleston,
    #[serde(rename = "Zoe Saldana")]
    ZoeSaldana,
}
impl std::fmt::Display for V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ARRahman => "A.R. Rahman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AamirKhan => "Aamir Khan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AbrahamLincoln => {
                "Abraham Lincoln"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AdamDriver => "Adam Driver",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AjayDevgn => "Ajay Devgn",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AkshayKumar => "Akshay Kumar",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlainDelon => "Alain Delon",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlanAlda => "Alan Alda",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlanCumming => "Alan Cumming",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlanRickman => "Alan Rickman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlbertEinstein => {
                "Albert Einstein"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlexTrebek => "Alex Trebek",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlexzJohnson => {
                "Alexz Johnson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AmitabhBachchan => {
                "Amitabh Bachchan"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AmyPoehler => "Amy Poehler",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnaGasteyer => "Ana Gasteyer",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AndrewRannells => {
                "Andrew Rannells"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AngLee => "Ang Lee",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnneHathaway => {
                "Anne Hathaway"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnselElgort => "Ansel Elgort",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnthonyAnderson => {
                "Anthony Anderson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnthonyMackie => {
                "Anthony Mackie"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ArdenCho => "Arden Cho",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ArmieHammer => "Armie Hammer",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AsaButterfield => {
                "Asa Butterfield"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BJNovak => "B.J. Novak",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BarackObama => "Barack Obama",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BarbaraEden => "Barbara Eden",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BearGrylls => "Bear Grylls",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BenMcKenzie => "Ben McKenzie",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BenStiller => "Ben Stiller",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BenWhishaw => "Ben Whishaw",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BettyWhite => "Betty White",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillNighy => "Bill Nighy",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillPullman => "Bill Pullman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillieJoeArmstrong => {
                "Billie Joe Armstrong"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BingbingLi => "Bingbing Li",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BlakeShelton => {
                "Blake Shelton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BobBarker => "Bob Barker",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BonnieWright => {
                "Bonnie Wright"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BoobooStewart => {
                "Booboo Stewart"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BradPaisley => "Brad Paisley",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BradleyStevenPerry => {
                "Bradley Steven Perry"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BrendanGleeson => {
                "Brendan Gleeson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BrianCox => "Brian Cox",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BrunoGanz => "Bruno Ganz",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BrunoMars => "Bruno Mars",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BurtReynolds => {
                "Burt Reynolds"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaitrionaBalfe => {
                "Caitriona Balfe"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaityLotz => "Caity Lotz",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CameronBoyce => {
                "Cameron Boyce"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CandiceAccola => {
                "Candice Accola"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CarrieFisher => {
                "Carrie Fisher"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CarrieUnderwood => {
                "Carrie Underwood"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaseyAffleck => {
                "Casey Affleck"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaterinaScorsone => {
                "Caterina Scorsone"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CedricTheEntertainer => {
                "Cedric the Entertainer"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChaceCrawford => {
                "Chace Crawford"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChadwickBoseman => {
                "Chadwick Boseman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharlesDance => {
                "Charles Dance"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharlieChaplin => {
                "Charlie Chaplin"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharlieDay => "Charlie Day",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChiwetelEjiofor => {
                "Chiwetel Ejiofor"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisHemsworth => {
                "Chris Hemsworth"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisMartin => "Chris Martin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisPine => "Chris Pine",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChristinaHendricks => {
                "Christina Hendricks"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChristinaRicci => {
                "Christina Ricci"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChristopherMintzPlasse => {
                "Christopher Mintz-Plasse"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CillianMurphy => {
                "Cillian Murphy"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CyndiLauper => "Cyndi Lauper",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DakotaFanning => {
                "Dakota Fanning"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DamianLewis => "Damian Lewis",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanAykroyd => "Dan Aykroyd",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanFogler => "Dan Fogler",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanStevens => "Dan Stevens",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanielCraig => "Daniel Craig",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanielDaeKim => {
                "Daniel Dae Kim"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DaniellePanabaker => {
                "Danielle Panabaker"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DaveBautista => {
                "Dave Bautista"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DavidAttenborough => {
                "David Attenborough"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DavidLetterman => {
                "David Letterman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DavidOyelowo => {
                "David Oyelowo"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DavidSchwimmer => {
                "David Schwimmer"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DavidTennant => {
                "David Tennant"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DenisLeary => "Denis Leary",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DerekMears => "Derek Mears",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DianeKeaton => "Diane Keaton",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DianeKruger => "Diane Kruger",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DickVanDyke => {
                "Dick Van Dyke"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DiegoLuna => "Diego Luna",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DomhnallGleeson => {
                "Domhnall Gleeson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DominicCooper => {
                "Dominic Cooper"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonaldGlover => {
                "Donald Glover"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonaldSutherland => {
                "Donald Sutherland"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonaldTrump => "Donald Trump",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonnieYen => "Donnie Yen",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DoutzenKroes => {
                "Doutzen Kroes"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DoveCameron => "Dove Cameron",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DrDre => "Dr. Dre",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DrakeBell => "Drake Bell",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DrewCarey => "Drew Carey",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EarthaKitt => "Eartha Kitt",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EddieIzzard => "Eddie Izzard",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EdwardAsner => "Edward Asner",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EliRoth => "Eli Roth",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElisabethMoss => {
                "Elisabeth Moss"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElleFanning => "Elle Fanning",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EllenBurstyn => {
                "Ellen Burstyn"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElonMusk => "Elon Musk",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EmileHirsch => "Emile Hirsch",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ErnieHudson => "Ernie Hudson",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EzraMiller => "Ezra Miller",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FelicityJones => {
                "Felicity Jones"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Fergie => "Fergie",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FionaShaw => "Fiona Shaw",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FlorenceHenderson => {
                "Florence Henderson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ForestWhitaker => {
                "Forest Whitaker"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FranciaRaisa => {
                "Francia Raisa"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FreddieHighmore => {
                "Freddie Highmore"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FreidaPinto => "Freida Pinto",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GeenaDavis => "Geena Davis",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GemmaArterton => {
                "Gemma Arterton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GeriHalliwell => {
                "Geri Halliwell"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GillianJacobs => {
                "Gillian Jacobs"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GinaCarano => "Gina Carano",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GinniferGoodwin => {
                "Ginnifer Goodwin"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GlennClose => "Glenn Close",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GloriaSteinem => {
                "Gloria Steinem"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GordonRamsay => {
                "Gordon Ramsay"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GretaGerwig => "Greta Gerwig",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GuguMbathaRaw => {
                "Gugu Mbatha-Raw"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GuyPearce => "Guy Pearce",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GwendolineChristie => {
                "Gwendoline Christie"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HaileeSteinfeld => {
                "Hailee Steinfeld"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HansZimmer => "Hans Zimmer",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HarryConnickJr => {
                "Harry Connick Jr."
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HarveyKeitel => {
                "Harvey Keitel"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HelenaBonhamCarter => {
                "Helena Bonham Carter"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HenryCavill => "Henry Cavill",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HilarySwank => "Hilary Swank",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HowieMandel => "Howie Mandel",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HughBonneville => {
                "Hugh Bonneville"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HughJackman => "Hugh Jackman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HughLaurie => "Hugh Laurie",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IdinaMenzel => "Idina Menzel",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ImeldaStaunton => {
                "Imelda Staunton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IngridBergman => {
                "Ingrid Bergman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IrrfanKhan => "Irrfan Khan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IslaFisher => "Isla Fisher",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IwanRheon => "Iwan Rheon",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JKSimmons => "J. K. Simmons",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JJAbrams => "J.J. Abrams",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JackBlack => "Jack Black",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JackLemmon => "Jack Lemmon",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JamesEarlJones => {
                "James Earl Jones"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JanetJackson => {
                "Janet Jackson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JaredLeto => "Jared Leto",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JasonBateman => {
                "Jason Bateman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JasonSegel => "Jason Segel",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JeffGoldblum => {
                "Jeff Goldblum"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JenniferCarpenter => {
                "Jennifer Carpenter"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JenniferCoolidge => {
                "Jennifer Coolidge"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JimmyFallon => "Jimmy Fallon",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeBiden => "Joe Biden",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeRogan => "Joe Rogan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnFKennedy => {
                "John F. Kennedy"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnnyGalecki => {
                "Johnny Galecki"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JonFavreau => "Jon Favreau",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JosephGordonLevitt => {
                "Joseph Gordon-Levitt"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoshBrolin => "Josh Brolin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoshGad => "Josh Gad",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoshGroban => "Josh Groban",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JuliaLouisDreyfus => {
                "Julia Louis-Dreyfus"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JuliaRoberts => {
                "Julia Roberts"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KanyeWest => "Kanye West",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KatyPerry => "Katy Perry",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Kesha => "Kesha",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KevinBacon => "Kevin Bacon",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KimKardashian => {
                "Kim Kardashian"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KrisJenner => "Kris Jenner",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KristenBell => "Kristen Bell",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KristenStewart => {
                "Kristen Stewart"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KristenWiig => "Kristen Wiig",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Lorde => "Lorde",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LucilleBall => "Lucille Ball",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarilynMonroe => {
                "Marilyn Monroe"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarkZuckerberg => {
                "Mark Zuckerberg"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MattSmith => "Matt Smith",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MorganFreeman => {
                "Morgan Freeman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NataliePortman => {
                "Natalie Portman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Prince => "Prince",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RooneyMara => "Rooney Mara",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SamuelLJackson => {
                "Samuel L. Jackson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SeanConnery => "Sean Connery",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StephenColbert => {
                "Stephen Colbert"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SteveCarell => "Steve Carell",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TaylorSwift => "Taylor Swift",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TomHiddleston => {
                "Tom Hiddleston"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ZoeSaldana => "Zoe Saldana",
        };
        write!(f, "{}", str_val)
    }
}
