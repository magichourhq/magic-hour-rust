/// The voice to use for the speech. Use the `id` of a voice returned by `GET /v1/voices`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum {
    #[default]
    #[serde(rename = "21 Savage")]
    Enum21Savage,
    #[serde(rename = "50 Cent")]
    Enum50Cent,
    #[serde(rename = "A$AP Rocky")]
    AApRocky,
    #[serde(rename = "A.R. Rahman")]
    ARRahman,
    #[serde(rename = "Aamir Khan")]
    AamirKhan,
    #[serde(rename = "Aang (Avatar)")]
    AangAvatar,
    #[serde(rename = "Aaron Rodgers")]
    AaronRodgers,
    #[serde(rename = "Abraham Lincoln")]
    AbrahamLincoln,
    #[serde(rename = "Ada Wong")]
    AdaWong,
    #[serde(rename = "Adam Driver")]
    AdamDriver,
    #[serde(rename = "Adam Sandler")]
    AdamSandler,
    #[serde(rename = "Adin Ross")]
    AdinRoss,
    #[serde(rename = "Agent 47")]
    Agent47,
    #[serde(rename = "Airrack")]
    Airrack,
    #[serde(rename = "Ajay Devgn")]
    AjayDevgn,
    #[serde(rename = "Akshay Kumar")]
    AkshayKumar,
    #[serde(rename = "Al Gore")]
    AlGore,
    #[serde(rename = "Al Pacino")]
    AlPacino,
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
    #[serde(rename = "Albus Dumbledore")]
    AlbusDumbledore,
    #[serde(rename = "Alex Hormozi")]
    AlexHormozi,
    #[serde(rename = "Alex Pereira")]
    AlexPereira,
    #[serde(rename = "Alex Trebek")]
    AlexTrebek,
    #[serde(rename = "Alexandria Ocasio-Cortez")]
    AlexandriaOcasioCortez,
    #[serde(rename = "Alexz Johnson")]
    AlexzJohnson,
    #[serde(rename = "Ali Wong")]
    AliWong,
    #[serde(rename = "Alia Bhatt")]
    AliaBhatt,
    #[serde(rename = "All Might")]
    AllMight,
    #[serde(rename = "Aloy")]
    Aloy,
    #[serde(rename = "Altair")]
    Altair,
    #[serde(rename = "Alton Brown")]
    AltonBrown,
    #[serde(rename = "Alucard (Hellsing)")]
    AlucardHellsing,
    #[serde(rename = "Alvin (Chipmunks)")]
    AlvinChipmunks,
    #[serde(rename = "Amanda Nunes")]
    AmandaNunes,
    #[serde(rename = "Amitabh Bachchan")]
    AmitabhBachchan,
    #[serde(rename = "Amy Adams")]
    AmyAdams,
    #[serde(rename = "Amy Poehler")]
    AmyPoehler,
    #[serde(rename = "Amy Winehouse")]
    AmyWinehouse,
    #[serde(rename = "Ana Gasteyer")]
    AnaGasteyer,
    #[serde(rename = "Ana de Armas")]
    AnaDeArmas,
    #[serde(rename = "Anderson Cooper")]
    AndersonCooper,
    #[serde(rename = "Andrew Huberman")]
    AndrewHuberman,
    #[serde(rename = "Andrew Rannells")]
    AndrewRannells,
    #[serde(rename = "Andrew Rea")]
    AndrewRea,
    #[serde(rename = "Andrew Schulz")]
    AndrewSchulz,
    #[serde(rename = "Andy Dwyer")]
    AndyDwyer,
    #[serde(rename = "Ang Lee")]
    AngLee,
    #[serde(rename = "Angel Reese")]
    AngelReese,
    #[serde(rename = "Angela Martin")]
    AngelaMartin,
    #[serde(rename = "Angela Merkel")]
    AngelaMerkel,
    #[serde(rename = "Angela Yee")]
    AngelaYee,
    #[serde(rename = "Angelina Jolie")]
    AngelinaJolie,
    #[serde(rename = "Anger (Inside Out)")]
    AngerInsideOut,
    #[serde(rename = "Anitta")]
    Anitta,
    #[serde(rename = "Anne Hathaway")]
    AnneHathaway,
    #[serde(rename = "Ansel Elgort")]
    AnselElgort,
    #[serde(rename = "Ant-Man")]
    AntMan,
    #[serde(rename = "Anthony Anderson")]
    AnthonyAnderson,
    #[serde(rename = "Anthony Bourdain")]
    AnthonyBourdain,
    #[serde(rename = "Anthony Edwards")]
    AnthonyEdwards,
    #[serde(rename = "Anthony Fauci")]
    AnthonyFauci,
    #[serde(rename = "Anthony Hopkins")]
    AnthonyHopkins,
    #[serde(rename = "Anthony Joshua")]
    AnthonyJoshua,
    #[serde(rename = "Anthony Mackie")]
    AnthonyMackie,
    #[serde(rename = "Anton Chigurh")]
    AntonChigurh,
    #[serde(rename = "Antonio Banderas")]
    AntonioBanderas,
    #[serde(rename = "Anya Forger")]
    AnyaForger,
    #[serde(rename = "Anya Taylor-Joy")]
    AnyaTaylorJoy,
    #[serde(rename = "April Ludgate")]
    AprilLudgate,
    #[serde(rename = "Aragorn")]
    Aragorn,
    #[serde(rename = "Archer (Sterling)")]
    ArcherSterling,
    #[serde(rename = "Arden Cho")]
    ArdenCho,
    #[serde(rename = "Aretha Franklin")]
    ArethaFranklin,
    #[serde(rename = "Ariana Grande")]
    ArianaGrande,
    #[serde(rename = "Ariel (Little Mermaid)")]
    ArielLittleMermaid,
    #[serde(rename = "Armie Hammer")]
    ArmieHammer,
    #[serde(rename = "Arnold Schwarzenegger")]
    ArnoldSchwarzenegger,
    #[serde(rename = "Arthur Morgan")]
    ArthurMorgan,
    #[serde(rename = "Arya Stark")]
    AryaStark,
    #[serde(rename = "Asa Butterfield")]
    AsaButterfield,
    #[serde(rename = "Ash Ketchum")]
    AshKetchum,
    #[serde(rename = "Asmongold")]
    Asmongold,
    #[serde(rename = "Aubrey Plaza")]
    AubreyPlaza,
    #[serde(rename = "Austin Butler")]
    AustinButler,
    #[serde(rename = "Austin Powers")]
    AustinPowers,
    #[serde(rename = "Awkwafina")]
    Awkwafina,
    #[serde(rename = "Axl Rose")]
    AxlRose,
    #[serde(rename = "Ayo Edebiri")]
    AyoEdebiri,
    #[serde(rename = "Aziz Ansari")]
    AzizAnsari,
    #[serde(rename = "B.J. Novak")]
    BJNovak,
    #[serde(rename = "BJ Blazkowicz")]
    BjBlazkowicz,
    #[serde(rename = "BTS Jungkook")]
    BtsJungkook,
    #[serde(rename = "Babar Azam")]
    BabarAzam,
    #[serde(rename = "Baby Yoda (Grogu)")]
    BabyYodaGrogu,
    #[serde(rename = "Bad Bunny")]
    BadBunny,
    #[serde(rename = "Bakugo Katsuki")]
    BakugoKatsuki,
    #[serde(rename = "Bam Margera")]
    BamMargera,
    #[serde(rename = "Barack Obama")]
    BarackObama,
    #[serde(rename = "Barbara Eden")]
    BarbaraEden,
    #[serde(rename = "Barney Rubble")]
    BarneyRubble,
    #[serde(rename = "Barney Stinson")]
    BarneyStinson,
    #[serde(rename = "Barry Keoghan")]
    BarryKeoghan,
    #[serde(rename = "Barry White")]
    BarryWhite,
    #[serde(rename = "Bart Simpson")]
    BartSimpson,
    #[serde(rename = "Batman (Animated)")]
    BatmanAnimated,
    #[serde(rename = "Bear Grylls")]
    BearGrylls,
    #[serde(rename = "Beavis")]
    Beavis,
    #[serde(rename = "Belle (Beauty and the Beast)")]
    BelleBeautyAndTheBeast,
    #[serde(rename = "Ben McKenzie")]
    BenMcKenzie,
    #[serde(rename = "Ben Shapiro")]
    BenShapiro,
    #[serde(rename = "Ben Stiller")]
    BenStiller,
    #[serde(rename = "Ben Stokes")]
    BenStokes,
    #[serde(rename = "Ben Tennyson")]
    BenTennyson,
    #[serde(rename = "Ben Whishaw")]
    BenWhishaw,
    #[serde(rename = "Bender (Futurama)")]
    BenderFuturama,
    #[serde(rename = "Benedict Cumberbatch")]
    BenedictCumberbatch,
    #[serde(rename = "Benson Boone")]
    BensonBoone,
    #[serde(rename = "Bernie Sanders")]
    BernieSanders,
    #[serde(rename = "Bert Kreischer")]
    BertKreischer,
    #[serde(rename = "Beth Smith")]
    BethSmith,
    #[serde(rename = "Betty White")]
    BettyWhite,
    #[serde(rename = "Big Boss (MGS)")]
    BigBossMgs,
    #[serde(rename = "Big Boy")]
    BigBoy,
    #[serde(rename = "Big Mouth Nick")]
    BigMouthNick,
    #[serde(rename = "Big Smoke (GTA)")]
    BigSmokeGta,
    #[serde(rename = "Bill Burr")]
    BillBurr,
    #[serde(rename = "Bill Clinton")]
    BillClinton,
    #[serde(rename = "Bill Gates")]
    BillGates,
    #[serde(rename = "Bill Maher")]
    BillMaher,
    #[serde(rename = "Bill Nighy")]
    BillNighy,
    #[serde(rename = "Bill Nye")]
    BillNye,
    #[serde(rename = "Bill Pullman")]
    BillPullman,
    #[serde(rename = "Bill Simmons")]
    BillSimmons,
    #[serde(rename = "Billie Eilish")]
    BillieEilish,
    #[serde(rename = "Billie Joe Armstrong")]
    BillieJoeArmstrong,
    #[serde(rename = "Billy Butcher")]
    BillyButcher,
    #[serde(rename = "Billy Joel")]
    BillyJoel,
    #[serde(rename = "Bingbing Li")]
    BingbingLi,
    #[serde(rename = "Bizarrap")]
    Bizarrap,
    #[serde(rename = "Black Panther")]
    BlackPanther,
    #[serde(rename = "Black Widow")]
    BlackWidow,
    #[serde(rename = "Blake Shelton")]
    BlakeShelton,
    #[serde(rename = "Blossom (PPG)")]
    BlossomPpg,
    #[serde(rename = "Bo Burnham")]
    BoBurnham,
    #[serde(rename = "BoJack Horseman")]
    BoJackHorseman,
    #[serde(rename = "Bob Marley")]
    BobMarley,
    #[serde(rename = "Bob Odenkirk")]
    BobOdenkirk,
    #[serde(rename = "Bob Ross")]
    BobRoss,
    #[serde(rename = "Bobby Hill")]
    BobbyHill,
    #[serde(rename = "Bobby Lee")]
    BobbyLee,
    #[serde(rename = "Bonnie Wright")]
    BonnieWright,
    #[serde(rename = "Booboo Stewart")]
    BoobooStewart,
    #[serde(rename = "Boris Johnson")]
    BorisJohnson,
    #[serde(rename = "Bowser")]
    Bowser,
    #[serde(rename = "Brad Paisley")]
    BradPaisley,
    #[serde(rename = "Brad Pitt")]
    BradPitt,
    #[serde(rename = "Bradley Cooper")]
    BradleyCooper,
    #[serde(rename = "Bradley Steven Perry")]
    BradleyStevenPerry,
    #[serde(rename = "Bran Stark")]
    BranStark,
    #[serde(rename = "Bray Wyatt")]
    BrayWyatt,
    #[serde(rename = "Brendan Gleeson")]
    BrendanGleeson,
    #[serde(rename = "Bretman Rock")]
    BretmanRock,
    #[serde(rename = "Brian Cox")]
    BrianCox,
    #[serde(rename = "Bruce Springsteen")]
    BruceSpringsteen,
    #[serde(rename = "Bruce Willis")]
    BruceWillis,
    #[serde(rename = "Bruno (Encanto)")]
    BrunoEncanto,
    #[serde(rename = "Bruno Ganz")]
    BrunoGanz,
    #[serde(rename = "Bruno Mars")]
    BrunoMars,
    #[serde(rename = "Bryan Cranston")]
    BryanCranston,
    #[serde(rename = "Bubbles (PPG)")]
    BubblesPpg,
    #[serde(rename = "Bugs Bunny")]
    BugsBunny,
    #[serde(rename = "Burna Boy")]
    BurnaBoy,
    #[serde(rename = "Burt Reynolds")]
    BurtReynolds,
    #[serde(rename = "Butt-Head")]
    ButtHead,
    #[serde(rename = "Buttercup (PPG)")]
    ButtercupPpg,
    #[serde(rename = "Butters Stotch")]
    ButtersStotch,
    #[serde(rename = "Buzz Aldrin")]
    BuzzAldrin,
    #[serde(rename = "Buzz Lightyear")]
    BuzzLightyear,
    #[serde(rename = "C-3PO")]
    C3po,
    #[serde(rename = "CJ (GTA San Andreas)")]
    CjGtaSanAndreas,
    #[serde(rename = "CM Punk")]
    CmPunk,
    #[serde(rename = "Caitlin Clark")]
    CaitlinClark,
    #[serde(rename = "Caitriona Balfe")]
    CaitrionaBalfe,
    #[serde(rename = "Caity Lotz")]
    CaityLotz,
    #[serde(rename = "Calvin Harris")]
    CalvinHarris,
    #[serde(rename = "Cameron Boyce")]
    CameronBoyce,
    #[serde(rename = "Camila Cabello")]
    CamilaCabello,
    #[serde(rename = "Camilo")]
    Camilo,
    #[serde(rename = "Candace Owens")]
    CandaceOwens,
    #[serde(rename = "Candice Accola")]
    CandiceAccola,
    #[serde(rename = "Canelo Alvarez")]
    CaneloAlvarez,
    #[serde(rename = "Captain America (MCU)")]
    CaptainAmericaMcu,
    #[serde(rename = "Captain Hook (Peter Pan)")]
    CaptainHookPeterPan,
    #[serde(rename = "Captain Jack Sparrow")]
    CaptainJackSparrow,
    #[serde(rename = "Cardi B")]
    CardiB,
    #[serde(rename = "Carl Sagan")]
    CarlSagan,
    #[serde(rename = "Carlton Banks")]
    CarltonBanks,
    #[serde(rename = "Carrie Fisher")]
    CarrieFisher,
    #[serde(rename = "Carrie Underwood")]
    CarrieUnderwood,
    #[serde(rename = "Casey Affleck")]
    CaseyAffleck,
    #[serde(rename = "Casey Neistat")]
    CaseyNeistat,
    #[serde(rename = "Cate Blanchett")]
    CateBlanchett,
    #[serde(rename = "Caterina Scorsone")]
    CaterinaScorsone,
    #[serde(rename = "Cedric the Entertainer")]
    CedricTheEntertainer,
    #[serde(rename = "CeeDee Lamb")]
    CeeDeeLamb,
    #[serde(rename = "Central Cee")]
    CentralCee,
    #[serde(rename = "Cersei Lannister")]
    CerseiLannister,
    #[serde(rename = "Chace Crawford")]
    ChaceCrawford,
    #[serde(rename = "Chadwick Boseman")]
    ChadwickBoseman,
    #[serde(rename = "Chandler Bing")]
    ChandlerBing,
    #[serde(rename = "Chappell Roan")]
    ChappellRoan,
    #[serde(rename = "Charlamagne Tha God")]
    CharlamagneThaGod,
    #[serde(rename = "Charles Barkley")]
    CharlesBarkley,
    #[serde(rename = "Charles Dance")]
    CharlesDance,
    #[serde(rename = "Charles Leclerc")]
    CharlesLeclerc,
    #[serde(rename = "Charli D'Amelio")]
    CharliDAmelio,
    #[serde(rename = "Charli XCX")]
    CharliXcx,
    #[serde(rename = "Charlie Chaplin")]
    CharlieChaplin,
    #[serde(rename = "Charlie Day")]
    CharlieDay,
    #[serde(rename = "Charlie Kirk")]
    CharlieKirk,
    #[serde(rename = "Charlize Theron")]
    CharlizeTheron,
    #[serde(rename = "Chef (South Park)")]
    ChefSouthPark,
    #[serde(rename = "Chester Bennington")]
    ChesterBennington,
    #[serde(rename = "Chiwetel Ejiofor")]
    ChiwetelEjiofor,
    #[serde(rename = "Chris Brown")]
    ChrisBrown,
    #[serde(rename = "Chris Cornell")]
    ChrisCornell,
    #[serde(rename = "Chris Evans")]
    ChrisEvans,
    #[serde(rename = "Chris Hemsworth")]
    ChrisHemsworth,
    #[serde(rename = "Chris Martin")]
    ChrisMartin,
    #[serde(rename = "Chris Pine")]
    ChrisPine,
    #[serde(rename = "Chris Pratt")]
    ChrisPratt,
    #[serde(rename = "Chris Redfield")]
    ChrisRedfield,
    #[serde(rename = "Chris Rock")]
    ChrisRock,
    #[serde(rename = "Chris Stapleton")]
    ChrisStapleton,
    #[serde(rename = "Chris Tucker")]
    ChrisTucker,
    #[serde(rename = "Chris Williamson")]
    ChrisWilliamson,
    #[serde(rename = "Christian Bale")]
    ChristianBale,
    #[serde(rename = "Christina Hendricks")]
    ChristinaHendricks,
    #[serde(rename = "Christina Ricci")]
    ChristinaRicci,
    #[serde(rename = "Christopher Mintz-Plasse")]
    ChristopherMintzPlasse,
    #[serde(rename = "Christopher Walken")]
    ChristopherWalken,
    #[serde(rename = "Chucky")]
    Chucky,
    #[serde(rename = "Chun-Li")]
    ChunLi,
    #[serde(rename = "Cillian Murphy")]
    CillianMurphy,
    #[serde(rename = "Cinderella")]
    Cinderella,
    #[serde(rename = "Claptrap")]
    Claptrap,
    #[serde(rename = "Clint Eastwood")]
    ClintEastwood,
    #[serde(rename = "Cloud Strife")]
    CloudStrife,
    #[serde(rename = "Cody Ko")]
    CodyKo,
    #[serde(rename = "Cody Rhodes")]
    CodyRhodes,
    #[serde(rename = "Cogsworth (Beauty and the Beast)")]
    CogsworthBeautyAndTheBeast,
    #[serde(rename = "Coi Leray")]
    CoiLeray,
    #[serde(rename = "Colin Cowherd")]
    ColinCowherd,
    #[serde(rename = "Colleen Ballinger")]
    ColleenBallinger,
    #[serde(rename = "Commander Shepard")]
    CommanderShepard,
    #[serde(rename = "Conan Gray")]
    ConanGray,
    #[serde(rename = "Conan O'Brien")]
    ConanOBrien,
    #[serde(rename = "Connor McDavid")]
    ConnorMcDavid,
    #[serde(rename = "Conor McGregor")]
    ConorMcGregor,
    #[serde(rename = "Constance Wu")]
    ConstanceWu,
    #[serde(rename = "Corey Taylor")]
    CoreyTaylor,
    #[serde(rename = "Corpse Husband")]
    CorpseHusband,
    #[serde(rename = "Cortana")]
    Cortana,
    #[serde(rename = "CouRage")]
    CouRage,
    #[serde(rename = "Courage the Cowardly Dog")]
    CourageTheCowardlyDog,
    #[serde(rename = "Creed Bratton")]
    CreedBratton,
    #[serde(rename = "Cristiano Ronaldo")]
    CristianoRonaldo,
    #[serde(rename = "Daenerys Targaryen")]
    DaenerysTargaryen,
    #[serde(rename = "Daffy Duck")]
    DaffyDuck,
    #[serde(rename = "Daisy Ridley")]
    DaisyRidley,
    #[serde(rename = "Dakota Fanning")]
    DakotaFanning,
    #[serde(rename = "Dale Gribble")]
    DaleGribble,
    #[serde(rename = "Damian Lewis")]
    DamianLewis,
    #[serde(rename = "Damian Lillard")]
    DamianLillard,
    #[serde(rename = "Dan Aykroyd")]
    DanAykroyd,
    #[serde(rename = "Dan Fogler")]
    DanFogler,
    #[serde(rename = "Dan Le Batard")]
    DanLeBatard,
    #[serde(rename = "Dan Stevens")]
    DanStevens,
    #[serde(rename = "Dana White")]
    DanaWhite,
    #[serde(rename = "Daniel Craig")]
    DanielCraig,
    #[serde(rename = "Daniel Dae Kim")]
    DanielDaeKim,
    #[serde(rename = "Daniel Radcliffe")]
    DanielRadcliffe,
    #[serde(rename = "Daniel Ricciardo")]
    DanielRicciardo,
    #[serde(rename = "Danielle Panabaker")]
    DaniellePanabaker,
    #[serde(rename = "Danny DeVito")]
    DannyDeVito,
    #[serde(rename = "Danny Duncan")]
    DannyDuncan,
    #[serde(rename = "Danny Gonzalez")]
    DannyGonzalez,
    #[serde(rename = "Danny Phantom")]
    DannyPhantom,
    #[serde(rename = "Dante (DMC)")]
    DanteDmc,
    #[serde(rename = "Darth Vader")]
    DarthVader,
    #[serde(rename = "Dave (rapper)")]
    DaveRapper,
    #[serde(rename = "Dave Bautista")]
    DaveBautista,
    #[serde(rename = "Dave Chappelle")]
    DaveChappelle,
    #[serde(rename = "Dave Grohl")]
    DaveGrohl,
    #[serde(rename = "Dave Rubin")]
    DaveRubin,
    #[serde(rename = "David Attenborough")]
    DavidAttenborough,
    #[serde(rename = "David Beckham")]
    DavidBeckham,
    #[serde(rename = "David Bowie")]
    DavidBowie,
    #[serde(rename = "David Dobrik")]
    DavidDobrik,
    #[serde(rename = "David Goggins")]
    DavidGoggins,
    #[serde(rename = "David Guetta")]
    DavidGuetta,
    #[serde(rename = "David Letterman")]
    DavidLetterman,
    #[serde(rename = "David Oyelowo")]
    DavidOyelowo,
    #[serde(rename = "David Schwimmer")]
    DavidSchwimmer,
    #[serde(rename = "David Tennant")]
    DavidTennant,
    #[serde(rename = "Deadpool")]
    Deadpool,
    #[serde(rename = "Deepika Padukone")]
    DeepikaPadukone,
    #[serde(rename = "Deku (My Hero Academia)")]
    DekuMyHeroAcademia,
    #[serde(rename = "Demi Lovato")]
    DemiLovato,
    #[serde(rename = "Denis Leary")]
    DenisLeary,
    #[serde(rename = "Denji (Chainsaw Man)")]
    DenjiChainsawMan,
    #[serde(rename = "Denzel Washington")]
    DenzelWashington,
    #[serde(rename = "Derek Mears")]
    DerekMears,
    #[serde(rename = "Desmond Tutu")]
    DesmondTutu,
    #[serde(rename = "Destiny")]
    Destiny,
    #[serde(rename = "Dev Patel")]
    DevPatel,
    #[serde(rename = "Devin Booker")]
    DevinBooker,
    #[serde(rename = "Dexter (Dexter's Lab)")]
    DexterDexterSLab,
    #[serde(rename = "Dexter Morgan")]
    DexterMorgan,
    #[serde(rename = "Diane Keaton")]
    DianeKeaton,
    #[serde(rename = "Diane Kruger")]
    DianeKruger,
    #[serde(rename = "Diane Nguyen")]
    DianeNguyen,
    #[serde(rename = "Dick Van Dyke")]
    DickVanDyke,
    #[serde(rename = "Diego Luna")]
    DiegoLuna,
    #[serde(rename = "Diplo")]
    Diplo,
    #[serde(rename = "Discord (My Little Pony)")]
    DiscordMyLittlePony,
    #[serde(rename = "Disguised Toast")]
    DisguisedToast,
    #[serde(rename = "Doctor Strange")]
    DoctorStrange,
    #[serde(rename = "Doja Cat")]
    DojaCat,
    #[serde(rename = "Dolly Parton")]
    DollyParton,
    #[serde(rename = "Domhnall Gleeson")]
    DomhnallGleeson,
    #[serde(rename = "Dominic Cooper")]
    DominicCooper,
    #[serde(rename = "Don Draper")]
    DonDraper,
    #[serde(rename = "Don Toliver")]
    DonToliver,
    #[serde(rename = "Donald Duck")]
    DonaldDuck,
    #[serde(rename = "Donald Glover")]
    DonaldGlover,
    #[serde(rename = "Donald Sutherland")]
    DonaldSutherland,
    #[serde(rename = "Donald Trump")]
    DonaldTrump,
    #[serde(rename = "Donkey (Shrek)")]
    DonkeyShrek,
    #[serde(rename = "Donnie Yen")]
    DonnieYen,
    #[serde(rename = "Doofenshmirtz")]
    Doofenshmirtz,
    #[serde(rename = "Doom Slayer")]
    DoomSlayer,
    #[serde(rename = "Douglas Murray")]
    DouglasMurray,
    #[serde(rename = "Doutzen Kroes")]
    DoutzenKroes,
    #[serde(rename = "Dr. Dre")]
    DrDre,
    #[serde(rename = "Dr. House")]
    DrHouse,
    #[serde(rename = "Dr. Phil")]
    DrPhil,
    #[serde(rename = "DrLupo")]
    DrLupo,
    #[serde(rename = "Drake")]
    Drake,
    #[serde(rename = "Drake Bell")]
    DrakeBell,
    #[serde(rename = "Dream (Minecraft)")]
    DreamMinecraft,
    #[serde(rename = "Drew Carey")]
    DrewCarey,
    #[serde(rename = "Drew Gooden")]
    DrewGooden,
    #[serde(rename = "Dua Lipa")]
    DuaLipa,
    #[serde(rename = "Dude Perfect Tyler")]
    DudePerfectTyler,
    #[serde(rename = "Duke Nukem")]
    DukeNukem,
    #[serde(rename = "Dwayne 'The Rock' Johnson")]
    DwayneTheRockJohnson,
    #[serde(rename = "Dwight Schrute")]
    DwightSchrute,
    #[serde(rename = "Eartha Kitt")]
    EarthaKitt,
    #[serde(rename = "Ed Sheeran")]
    EdSheeran,
    #[serde(rename = "Eddie Izzard")]
    EddieIzzard,
    #[serde(rename = "Eddie Murphy")]
    EddieMurphy,
    #[serde(rename = "Edward Asner")]
    EdwardAsner,
    #[serde(rename = "Edward Elric")]
    EdwardElric,
    #[serde(rename = "Elaine Benes")]
    ElaineBenes,
    #[serde(rename = "Eli Roth")]
    EliRoth,
    #[serde(rename = "Elisabeth Moss")]
    ElisabethMoss,
    #[serde(rename = "Elle Fanning")]
    ElleFanning,
    #[serde(rename = "Ellen Burstyn")]
    EllenBurstyn,
    #[serde(rename = "Ellen DeGeneres")]
    EllenDeGeneres,
    #[serde(rename = "Ellen Ripley")]
    EllenRipley,
    #[serde(rename = "Ellie (Borderlands)")]
    EllieBorderlands,
    #[serde(rename = "Ellie Williams")]
    EllieWilliams,
    #[serde(rename = "Elmo")]
    Elmo,
    #[serde(rename = "Elon Musk")]
    ElonMusk,
    #[serde(rename = "Elsa (Frozen)")]
    ElsaFrozen,
    #[serde(rename = "Elton John")]
    EltonJohn,
    #[serde(rename = "Elvis Presley")]
    ElvisPresley,
    #[serde(rename = "Emile Hirsch")]
    EmileHirsch,
    #[serde(rename = "Eminem")]
    Eminem,
    #[serde(rename = "Emma Chamberlain")]
    EmmaChamberlain,
    #[serde(rename = "Emma Stone")]
    EmmaStone,
    #[serde(rename = "Emma Watson")]
    EmmaWatson,
    #[serde(rename = "Emmanuel Macron")]
    EmmanuelMacron,
    #[serde(rename = "Emperor Palpatine")]
    EmperorPalpatine,
    #[serde(rename = "Enrique Iglesias")]
    EnriqueIglesias,
    #[serde(rename = "Eren Jaeger")]
    ErenJaeger,
    #[serde(rename = "Eric Cartman")]
    EricCartman,
    #[serde(rename = "Eric Clapton")]
    EricClapton,
    #[serde(rename = "Erling Haaland")]
    ErlingHaaland,
    #[serde(rename = "Ernie Hudson")]
    ErnieHudson,
    #[serde(rename = "Erza Scarlet")]
    ErzaScarlet,
    #[serde(rename = "Esfand")]
    Esfand,
    #[serde(rename = "Ethan Klein")]
    EthanKlein,
    #[serde(rename = "Eugene Lee Yang")]
    EugeneLeeYang,
    #[serde(rename = "Ezio Auditore")]
    EzioAuditore,
    #[serde(rename = "Ezra Miller")]
    EzraMiller,
    #[serde(rename = "FaZe Rug")]
    FaZeRug,
    #[serde(rename = "Family Guy Quagmire")]
    FamilyGuyQuagmire,
    #[serde(rename = "Faze Clan Banks")]
    FazeClanBanks,
    #[serde(rename = "Feid")]
    Feid,
    #[serde(rename = "Felicity Jones")]
    FelicityJones,
    #[serde(rename = "Fergie")]
    Fergie,
    #[serde(rename = "Fernando Alonso")]
    FernandoAlonso,
    #[serde(rename = "Ferris Bueller")]
    FerrisBueller,
    #[serde(rename = "Finn Wolfhard")]
    FinnWolfhard,
    #[serde(rename = "Finn the Human")]
    FinnTheHuman,
    #[serde(rename = "Fiona (Shrek)")]
    FionaShrek,
    #[serde(rename = "Fiona Shaw")]
    FionaShaw,
    #[serde(rename = "Fleetwood Mac Stevie Nicks")]
    FleetwoodMacStevieNicks,
    #[serde(rename = "Flo Milli")]
    FloMilli,
    #[serde(rename = "Florence Henderson")]
    FlorenceHenderson,
    #[serde(rename = "Florence Pugh")]
    FlorencePugh,
    #[serde(rename = "Floyd Mayweather")]
    FloydMayweather,
    #[serde(rename = "Forest Whitaker")]
    ForestWhitaker,
    #[serde(rename = "Forrest Gump")]
    ForrestGump,
    #[serde(rename = "Forsen")]
    Forsen,
    #[serde(rename = "Francia Raisa")]
    FranciaRaisa,
    #[serde(rename = "Frank Sinatra")]
    FrankSinatra,
    #[serde(rename = "Frank Underwood")]
    FrankUnderwood,
    #[serde(rename = "Franklin Clinton")]
    FranklinClinton,
    #[serde(rename = "Franklin D. Roosevelt")]
    FranklinDRoosevelt,
    #[serde(rename = "Fred Flintstone")]
    FredFlintstone,
    #[serde(rename = "Fred again")]
    FredAgain,
    #[serde(rename = "Freddie Highmore")]
    FreddieHighmore,
    #[serde(rename = "Freddie Mercury")]
    FreddieMercury,
    #[serde(rename = "Freddie Prinze")]
    FreddiePrinze,
    #[serde(rename = "Freddy Krueger")]
    FreddyKrueger,
    #[serde(rename = "Freida Pinto")]
    FreidaPinto,
    #[serde(rename = "Frodo Baggins")]
    FrodoBaggins,
    #[serde(rename = "Fry (Futurama)")]
    FryFuturama,
    #[serde(rename = "Fuerza Regida")]
    FuerzaRegida,
    #[serde(rename = "Fuslie")]
    Fuslie,
    #[serde(rename = "Future")]
    Future,
    #[serde(rename = "GIR (Invader Zim)")]
    GirInvaderZim,
    #[serde(rename = "GLaDOS")]
    GLaDos,
    #[serde(rename = "Gabriel Iglesias")]
    GabrielIglesias,
    #[serde(rename = "Gal Gadot")]
    GalGadot,
    #[serde(rename = "Gandalf")]
    Gandalf,
    #[serde(rename = "Garnet (Steven Universe)")]
    GarnetStevenUniverse,
    #[serde(rename = "Garrus Vakarian")]
    GarrusVakarian,
    #[serde(rename = "Gary Oldman")]
    GaryOldman,
    #[serde(rename = "Gary Vaynerchuk")]
    GaryVaynerchuk,
    #[serde(rename = "Geena Davis")]
    GeenaDavis,
    #[serde(rename = "Gemma Arterton")]
    GemmaArterton,
    #[serde(rename = "Genji (Overwatch)")]
    GenjiOverwatch,
    #[serde(rename = "George Costanza")]
    GeorgeCostanza,
    #[serde(rename = "George H W Bush")]
    GeorgeHWBush,
    #[serde(rename = "George Jetson")]
    GeorgeJetson,
    #[serde(rename = "George R.R. Martin")]
    GeorgeRRMartin,
    #[serde(rename = "George W Bush")]
    GeorgeWBush,
    #[serde(rename = "GeorgeNotFound")]
    GeorgeNotFound,
    #[serde(rename = "Geralt of Rivia")]
    GeraltOfRivia,
    #[serde(rename = "Geri Halliwell")]
    GeriHalliwell,
    #[serde(rename = "Ghostface (Scream)")]
    GhostfaceScream,
    #[serde(rename = "Giannis Antetokounmpo")]
    GiannisAntetokounmpo,
    #[serde(rename = "Gilbert Gottfried")]
    GilbertGottfried,
    #[serde(rename = "Gillian Jacobs")]
    GillianJacobs,
    #[serde(rename = "Gina Carano")]
    GinaCarano,
    #[serde(rename = "Ginnifer Goodwin")]
    GinniferGoodwin,
    #[serde(rename = "Glen Powell")]
    GlenPowell,
    #[serde(rename = "Glenn Close")]
    GlennClose,
    #[serde(rename = "GloRilla")]
    GloRilla,
    #[serde(rename = "Gloria Steinem")]
    GloriaSteinem,
    #[serde(rename = "Gojo Satoru")]
    GojoSatoru,
    #[serde(rename = "Goku")]
    Goku,
    #[serde(rename = "Gollum Smeagol")]
    GollumSmeagol,
    #[serde(rename = "Gon Freecss")]
    GonFreecss,
    #[serde(rename = "Gordon Lightfoot")]
    GordonLightfoot,
    #[serde(rename = "Gordon Ramsay")]
    GordonRamsay,
    #[serde(rename = "Gracie Abrams")]
    GracieAbrams,
    #[serde(rename = "Graham Norton")]
    GrahamNorton,
    #[serde(rename = "Grant Cardone")]
    GrantCardone,
    #[serde(rename = "Greta Gerwig")]
    GretaGerwig,
    #[serde(rename = "Greta Thunberg")]
    GretaThunberg,
    #[serde(rename = "Griffith (Berserk)")]
    GriffithBerserk,
    #[serde(rename = "Grimes")]
    Grimes,
    #[serde(rename = "Groot")]
    Groot,
    #[serde(rename = "Gru (Despicable Me)")]
    GruDespicableMe,
    #[serde(rename = "Grupo Frontera")]
    GrupoFrontera,
    #[serde(rename = "Gugu Mbatha-Raw")]
    GuguMbathaRaw,
    #[serde(rename = "Gumball Watterson")]
    GumballWatterson,
    #[serde(rename = "Gunna")]
    Gunna,
    #[serde(rename = "Gus Fring")]
    GusFring,
    #[serde(rename = "Guts (Berserk)")]
    GutsBerserk,
    #[serde(rename = "Guy Fieri")]
    GuyFieri,
    #[serde(rename = "Guy Pearce")]
    GuyPearce,
    #[serde(rename = "Gwendoline Christie")]
    GwendolineChristie,
    #[serde(rename = "H3H3")]
    H3h3,
    #[serde(rename = "HAL 9000")]
    Hal9000,
    #[serde(rename = "Hagrid")]
    Hagrid,
    #[serde(rename = "Hailee Steinfeld")]
    HaileeSteinfeld,
    #[serde(rename = "Halsey")]
    Halsey,
    #[serde(rename = "Han Solo")]
    HanSolo,
    #[serde(rename = "Handsome Jack")]
    HandsomeJack,
    #[serde(rename = "Hank Schrader")]
    HankSchrader,
    #[serde(rename = "Hannibal Lecter")]
    HannibalLecter,
    #[serde(rename = "Hans (Frozen)")]
    HansFrozen,
    #[serde(rename = "Hans Zimmer")]
    HansZimmer,
    #[serde(rename = "Harley Quinn")]
    HarleyQuinn,
    #[serde(rename = "Harrison Ford")]
    HarrisonFord,
    #[serde(rename = "Harry Connick Jr.")]
    HarryConnickJr,
    #[serde(rename = "Harry Potter")]
    HarryPotter,
    #[serde(rename = "Harry Styles")]
    HarryStyles,
    #[serde(rename = "Harvey Keitel")]
    HarveyKeitel,
    #[serde(rename = "Hasan Minhaj")]
    HasanMinhaj,
    #[serde(rename = "Hasan Piker")]
    HasanPiker,
    #[serde(rename = "Helena Bonham Carter")]
    HelenaBonhamCarter,
    #[serde(rename = "Henry Cavill")]
    HenryCavill,
    #[serde(rename = "Henry Golding")]
    HenryGolding,
    #[serde(rename = "Hermione Granger")]
    HermioneGranger,
    #[serde(rename = "Herobrine")]
    Herobrine,
    #[serde(rename = "Hikaru Nakamura")]
    HikaruNakamura,
    #[serde(rename = "Hila Klein")]
    HilaKlein,
    #[serde(rename = "Hilary Swank")]
    HilarySwank,
    #[serde(rename = "Hillary Clinton")]
    HillaryClinton,
    #[serde(rename = "Hinata Hyuga")]
    HinataHyuga,
    #[serde(rename = "Homelander")]
    Homelander,
    #[serde(rename = "Homer Simpson")]
    HomerSimpson,
    #[serde(rename = "Howard Stern")]
    HowardStern,
    #[serde(rename = "Howie Mandel")]
    HowieMandel,
    #[serde(rename = "Hozier")]
    Hozier,
    #[serde(rename = "Hrithik Roshan")]
    HrithikRoshan,
    #[serde(rename = "Hugh Bonneville")]
    HughBonneville,
    #[serde(rename = "Hugh Jackman")]
    HughJackman,
    #[serde(rename = "Hugh Laurie")]
    HughLaurie,
    #[serde(rename = "Hulk")]
    Hulk,
    #[serde(rename = "Hulk Hogan")]
    HulkHogan,
    #[serde(rename = "IShowSpeed")]
    IShowSpeed,
    #[serde(rename = "Iago (Aladdin parrot)")]
    IagoAladdinParrot,
    #[serde(rename = "Ian McKellen")]
    IanMcKellen,
    #[serde(rename = "Ibai Llanos")]
    IbaiLlanos,
    #[serde(rename = "Ice Cube")]
    IceCube,
    #[serde(rename = "Ice Spice")]
    IceSpice,
    #[serde(rename = "Ichigo Kurosaki")]
    IchigoKurosaki,
    #[serde(rename = "Idina Menzel")]
    IdinaMenzel,
    #[serde(rename = "Idris Elba")]
    IdrisElba,
    #[serde(rename = "Ike (Fire Emblem)")]
    IkeFireEmblem,
    #[serde(rename = "Ike Broflovski")]
    IkeBroflovski,
    #[serde(rename = "Iliza Shlesinger")]
    IlizaShlesinger,
    #[serde(rename = "Imelda Staunton")]
    ImeldaStaunton,
    #[serde(rename = "Indiana Jones")]
    IndianaJones,
    #[serde(rename = "Ingrid Bergman")]
    IngridBergman,
    #[serde(rename = "Inspector Clouseau")]
    InspectorClouseau,
    #[serde(rename = "Inuyasha")]
    Inuyasha,
    #[serde(rename = "Invader Zim")]
    InvaderZim,
    #[serde(rename = "Irrfan Khan")]
    IrrfanKhan,
    #[serde(rename = "Isaac Clarke")]
    IsaacClarke,
    #[serde(rename = "Isla Fisher")]
    IslaFisher,
    #[serde(rename = "Israel Adesanya")]
    IsraelAdesanya,
    #[serde(rename = "Itachi Uchiha")]
    ItachiUchiha,
    #[serde(rename = "Itadori Yuji")]
    ItadoriYuji,
    #[serde(rename = "Iwan Rheon")]
    IwanRheon,
    #[serde(rename = "J Balvin")]
    JBalvin,
    #[serde(rename = "J. K. Simmons")]
    JKSimmons,
    #[serde(rename = "J.J. Abrams")]
    JJAbrams,
    #[serde(rename = "J.K. Rowling")]
    JKRowling,
    #[serde(rename = "JD Vance")]
    JdVance,
    #[serde(rename = "Ja Morant")]
    JaMorant,
    #[serde(rename = "Jack Black")]
    JackBlack,
    #[serde(rename = "Jack Harlow")]
    JackHarlow,
    #[serde(rename = "Jack Lemmon")]
    JackLemmon,
    #[serde(rename = "Jack Nicholson")]
    JackNicholson,
    #[serde(rename = "Jack Skellington")]
    JackSkellington,
    #[serde(rename = "Jacksepticeye")]
    Jacksepticeye,
    #[serde(rename = "Jaclyn Hill")]
    JaclynHill,
    #[serde(rename = "Jacob Elordi")]
    JacobElordi,
    #[serde(rename = "Jafar (Aladdin)")]
    JafarAladdin,
    #[serde(rename = "Jaime Lannister")]
    JaimeLannister,
    #[serde(rename = "Jake Paul")]
    JakePaul,
    #[serde(rename = "Jake the Dog")]
    JakeTheDog,
    #[serde(rename = "James Bond (Craig era)")]
    JamesBondCraigEra,
    #[serde(rename = "James Charles")]
    JamesCharles,
    #[serde(rename = "James Corden")]
    JamesCorden,
    #[serde(rename = "James Earl Jones")]
    JamesEarlJones,
    #[serde(rename = "James Harden")]
    JamesHarden,
    #[serde(rename = "James Hetfield")]
    JamesHetfield,
    #[serde(rename = "James May")]
    JamesMay,
    #[serde(rename = "Jamie Foxx")]
    JamieFoxx,
    #[serde(rename = "Jamie Lee Curtis")]
    JamieLeeCurtis,
    #[serde(rename = "Jamie Oliver")]
    JamieOliver,
    #[serde(rename = "Jane Goodall")]
    JaneGoodall,
    #[serde(rename = "Janet Jackson")]
    JanetJackson,
    #[serde(rename = "Janis Joplin")]
    JanisJoplin,
    #[serde(rename = "Jared Leto")]
    JaredLeto,
    #[serde(rename = "Jason Bateman")]
    JasonBateman,
    #[serde(rename = "Jason Momoa")]
    JasonMomoa,
    #[serde(rename = "Jason Segel")]
    JasonSegel,
    #[serde(rename = "Jason Statham")]
    JasonStatham,
    #[serde(rename = "Jason Voorhees")]
    JasonVoorhees,
    #[serde(rename = "Jay-Z")]
    JayZ,
    #[serde(rename = "Jayson Tatum")]
    JaysonTatum,
    #[serde(rename = "Jeff Bezos")]
    JeffBezos,
    #[serde(rename = "Jeff Goldblum")]
    JeffGoldblum,
    #[serde(rename = "Jeffree Star")]
    JeffreeStar,
    #[serde(rename = "Jenna Marbles")]
    JennaMarbles,
    #[serde(rename = "Jenna Ortega")]
    JennaOrtega,
    #[serde(rename = "Jennie (Blackpink)")]
    JennieBlackpink,
    #[serde(rename = "Jennifer Aniston")]
    JenniferAniston,
    #[serde(rename = "Jennifer Carpenter")]
    JenniferCarpenter,
    #[serde(rename = "Jennifer Coolidge")]
    JenniferCoolidge,
    #[serde(rename = "Jennifer Hale")]
    JenniferHale,
    #[serde(rename = "Jennifer Lawrence")]
    JenniferLawrence,
    #[serde(rename = "Jensen Huang")]
    JensenHuang,
    #[serde(rename = "Jeremy Allen White")]
    JeremyAllenWhite,
    #[serde(rename = "Jeremy Clarkson")]
    JeremyClarkson,
    #[serde(rename = "Jeremy Renner")]
    JeremyRenner,
    #[serde(rename = "Jerma985")]
    Jerma985,
    #[serde(rename = "Jerry Seinfeld")]
    JerrySeinfeld,
    #[serde(rename = "Jerry Smith")]
    JerrySmith,
    #[serde(rename = "Jesse Pinkman")]
    JessePinkman,
    #[serde(rename = "Jill Valentine")]
    JillValentine,
    #[serde(rename = "Jim Carrey")]
    JimCarrey,
    #[serde(rename = "Jim Halpert")]
    JimHalpert,
    #[serde(rename = "Jimi Hendrix")]
    JimiHendrix,
    #[serde(rename = "Jimin")]
    Jimin,
    #[serde(rename = "Jimmy Carter")]
    JimmyCarter,
    #[serde(rename = "Jimmy Fallon")]
    JimmyFallon,
    #[serde(rename = "Jimmy Kimmel")]
    JimmyKimmel,
    #[serde(rename = "Jimmy Valmer")]
    JimmyValmer,
    #[serde(rename = "Jinx (Arcane)")]
    JinxArcane,
    #[serde(rename = "Joaquin Phoenix")]
    JoaquinPhoenix,
    #[serde(rename = "Jocko Willink")]
    JockoWillink,
    #[serde(rename = "Joe Biden")]
    JoeBiden,
    #[serde(rename = "Joe Budden")]
    JoeBudden,
    #[serde(rename = "Joe Burrow")]
    JoeBurrow,
    #[serde(rename = "Joe List")]
    JoeList,
    #[serde(rename = "Joe Pesci")]
    JoePesci,
    #[serde(rename = "Joe Rogan")]
    JoeRogan,
    #[serde(rename = "Joe Root")]
    JoeRoot,
    #[serde(rename = "Joe Scott")]
    JoeScott,
    #[serde(rename = "Joel Embiid")]
    JoelEmbiid,
    #[serde(rename = "Joel Miller")]
    JoelMiller,
    #[serde(rename = "Joey Diaz")]
    JoeyDiaz,
    #[serde(rename = "Joey Tribbiani")]
    JoeyTribbiani,
    #[serde(rename = "John Boyega")]
    JohnBoyega,
    #[serde(rename = "John Cena")]
    JohnCena,
    #[serde(rename = "John F. Kennedy")]
    JohnFKennedy,
    #[serde(rename = "John Legend")]
    JohnLegend,
    #[serde(rename = "John Lennon")]
    JohnLennon,
    #[serde(rename = "John Mulaney")]
    JohnMulaney,
    #[serde(rename = "John Oliver")]
    JohnOliver,
    #[serde(rename = "John Wick")]
    JohnWick,
    #[serde(rename = "Johnny Bravo")]
    JohnnyBravo,
    #[serde(rename = "Johnny Cash")]
    JohnnyCash,
    #[serde(rename = "Johnny Depp")]
    JohnnyDepp,
    #[serde(rename = "Johnny Galecki")]
    JohnnyGalecki,
    #[serde(rename = "Johnny Knoxville")]
    JohnnyKnoxville,
    #[serde(rename = "Johnny Silverhand")]
    JohnnySilverhand,
    #[serde(rename = "Jon Bon Jovi")]
    JonBonJovi,
    #[serde(rename = "Jon Favreau")]
    JonFavreau,
    #[serde(rename = "Jon Jones")]
    JonJones,
    #[serde(rename = "Jon Snow")]
    JonSnow,
    #[serde(rename = "Jonah Hill")]
    JonahHill,
    #[serde(rename = "Jonathan Majors")]
    JonathanMajors,
    #[serde(rename = "Jordan Peterson")]
    JordanPeterson,
    #[serde(rename = "Jorge Masvidal")]
    JorgeMasvidal,
    #[serde(rename = "Joseph Gordon-Levitt")]
    JosephGordonLevitt,
    #[serde(rename = "Josh Allen")]
    JoshAllen,
    #[serde(rename = "Josh Brolin")]
    JoshBrolin,
    #[serde(rename = "Josh Gad")]
    JoshGad,
    #[serde(rename = "Josh Groban")]
    JoshGroban,
    #[serde(rename = "Joy (Inside Out)")]
    JoyInsideOut,
    #[serde(rename = "Jude Bellingham")]
    JudeBellingham,
    #[serde(rename = "Julia Louis-Dreyfus")]
    JuliaLouisDreyfus,
    #[serde(rename = "Julia Roberts")]
    JuliaRoberts,
    #[serde(rename = "Justin Bieber")]
    JustinBieber,
    #[serde(rename = "Justin Jefferson")]
    JustinJefferson,
    #[serde(rename = "Justin Timberlake")]
    JustinTimberlake,
    #[serde(rename = "Justin Trudeau")]
    JustinTrudeau,
    #[serde(rename = "KSI")]
    Ksi,
    #[serde(rename = "Kagome Higurashi")]
    KagomeHigurashi,
    #[serde(rename = "Kai Cenat")]
    KaiCenat,
    #[serde(rename = "Kakashi Hatake")]
    KakashiHatake,
    #[serde(rename = "Kamala Harris")]
    KamalaHarris,
    #[serde(rename = "Kanye West")]
    KanyeWest,
    #[serde(rename = "Karl Jacobs")]
    KarlJacobs,
    #[serde(rename = "Karol G")]
    KarolG,
    #[serde(rename = "Katy Perry")]
    KatyPerry,
    #[serde(rename = "Ke Huy Quan")]
    KeHuyQuan,
    #[serde(rename = "Keanu Reeves")]
    KeanuReeves,
    #[serde(rename = "Keir Starmer")]
    KeirStarmer,
    #[serde(rename = "Keith David")]
    KeithDavid,
    #[serde(rename = "Kendrick Lamar")]
    KendrickLamar,
    #[serde(rename = "Kermit the Frog")]
    KermitTheFrog,
    #[serde(rename = "Kesha")]
    Kesha,
    #[serde(rename = "Kevin Bacon")]
    KevinBacon,
    #[serde(rename = "Kevin Durant")]
    KevinDurant,
    #[serde(rename = "Kevin Hart")]
    KevinHart,
    #[serde(rename = "Kevin Malone")]
    KevinMalone,
    #[serde(rename = "Kevin Owens")]
    KevinOwens,
    #[serde(rename = "Khabib Nurmagomedov")]
    KhabibNurmagomedov,
    #[serde(rename = "Khaby Lame")]
    KhabyLame,
    #[serde(rename = "Khloe Kardashian")]
    KhloeKardashian,
    #[serde(rename = "Killua Zoldyck")]
    KilluaZoldyck,
    #[serde(rename = "Kim Kardashian")]
    KimKardashian,
    #[serde(rename = "King Charles III")]
    KingCharlesIii,
    #[serde(rename = "King Leonidas")]
    KingLeonidas,
    #[serde(rename = "King of the Hill Hank Hill")]
    KingOfTheHillHankHill,
    #[serde(rename = "Kirby")]
    Kirby,
    #[serde(rename = "Kobe Bryant")]
    KobeBryant,
    #[serde(rename = "Kramer (Seinfeld)")]
    KramerSeinfeld,
    #[serde(rename = "Kratos")]
    Kratos,
    #[serde(rename = "Kris Jenner")]
    KrisJenner,
    #[serde(rename = "Kristen Bell")]
    KristenBell,
    #[serde(rename = "Kristen Stewart")]
    KristenStewart,
    #[serde(rename = "Kristen Wiig")]
    KristenWiig,
    #[serde(rename = "Kristoff (Frozen)")]
    KristoffFrozen,
    #[serde(rename = "Kronk (Emperor's New Groove)")]
    KronkEmperorSNewGroove,
    #[serde(rename = "Krusty the Clown")]
    KrustyTheClown,
    #[serde(rename = "Kurt Cobain")]
    KurtCobain,
    #[serde(rename = "Kurtis Conner")]
    KurtisConner,
    #[serde(rename = "Kyle Broflovski")]
    KyleBroflovski,
    #[serde(rename = "Kylian Mbappe")]
    KylianMbappe,
    #[serde(rename = "Kylie Jenner")]
    KylieJenner,
    #[serde(rename = "Kylo Ren")]
    KyloRen,
    #[serde(rename = "Kyrie Irving")]
    KyrieIrving,
    #[serde(rename = "L (Death Note)")]
    LDeathNote,
    #[serde(rename = "Lady Gaga")]
    LadyGaga,
    #[serde(rename = "Lana Del Rey")]
    LanaDelRey,
    #[serde(rename = "Lana Kane")]
    LanaKane,
    #[serde(rename = "Lara Croft")]
    LaraCroft,
    #[serde(rename = "Latto")]
    Latto,
    #[serde(rename = "LeBron James")]
    LeBronJames,
    #[serde(rename = "League of Legends Ahri")]
    LeagueOfLegendsAhri,
    #[serde(rename = "Lee Min Ho")]
    LeeMinHo,
    #[serde(rename = "Leela (Futurama)")]
    LeelaFuturama,
    #[serde(rename = "Legolas")]
    Legolas,
    #[serde(rename = "Leon Kennedy")]
    LeonKennedy,
    #[serde(rename = "Leonardo DiCaprio")]
    LeonardoDiCaprio,
    #[serde(rename = "Leslie Knope")]
    LeslieKnope,
    #[serde(rename = "Lester Holt")]
    LesterHolt,
    #[serde(rename = "Levi Ackerman")]
    LeviAckerman,
    #[serde(rename = "Lewis Hamilton")]
    LewisHamilton,
    #[serde(rename = "Lex Fridman")]
    LexFridman,
    #[serde(rename = "Liam Neeson")]
    LiamNeeson,
    #[serde(rename = "Light Yagami")]
    LightYagami,
    #[serde(rename = "Lightning McQueen")]
    LightningMcQueen,
    #[serde(rename = "Lil Baby")]
    LilBaby,
    #[serde(rename = "Lil Wayne")]
    LilWayne,
    #[serde(rename = "Lilith (Borderlands)")]
    LilithBorderlands,
    #[serde(rename = "Lilly Singh")]
    LillySingh,
    #[serde(rename = "Link (Zelda)")]
    LinkZelda,
    #[serde(rename = "Linus (LTT)")]
    LinusLtt,
    #[serde(rename = "Lionel Messi")]
    LionelMessi,
    #[serde(rename = "Lisa (Blackpink)")]
    LisaBlackpink,
    #[serde(rename = "Lisa Simpson")]
    LisaSimpson,
    #[serde(rename = "Liza Koshy")]
    LizaKoshy,
    #[serde(rename = "Lizzo")]
    Lizzo,
    #[serde(rename = "Logan Paul")]
    LoganPaul,
    #[serde(rename = "Loid Forger")]
    LoidForger,
    #[serde(rename = "Lois Griffin")]
    LoisGriffin,
    #[serde(rename = "Loki (MCU)")]
    LokiMcu,
    #[serde(rename = "Lorde")]
    Lorde,
    #[serde(rename = "Lucille Ball")]
    LucilleBall,
    #[serde(rename = "Lucy Heartfilia")]
    LucyHeartfilia,
    #[serde(rename = "Ludwig")]
    Ludwig,
    #[serde(rename = "Luffy (One Piece)")]
    LuffyOnePiece,
    #[serde(rename = "Luigi")]
    Luigi,
    #[serde(rename = "Luke Combs")]
    LukeCombs,
    #[serde(rename = "Luke Skywalker")]
    LukeSkywalker,
    #[serde(rename = "Lumiere (Beauty and the Beast)")]
    LumiereBeautyAndTheBeast,
    #[serde(rename = "Lupita Nyong'o")]
    LupitaNyongO,
    #[serde(rename = "MS Dhoni")]
    MsDhoni,
    #[serde(rename = "Magnus Carlsen")]
    MagnusCarlsen,
    #[serde(rename = "Mahatma Gandhi")]
    MahatmaGandhi,
    #[serde(rename = "Mahershala Ali")]
    MahershalaAli,
    #[serde(rename = "Malala Yousafzai")]
    MalalaYousafzai,
    #[serde(rename = "Malcolm Gladwell")]
    MalcolmGladwell,
    #[serde(rename = "Malcolm X")]
    MalcolmX,
    #[serde(rename = "Malory Archer")]
    MaloryArcher,
    #[serde(rename = "Maluma")]
    Maluma,
    #[serde(rename = "Manuel Turizo")]
    ManuelTurizo,
    #[serde(rename = "Marc Andreessen")]
    MarcAndreessen,
    #[serde(rename = "Marc Marquez")]
    MarcMarquez,
    #[serde(rename = "Marcus Fenix")]
    MarcusFenix,
    #[serde(rename = "Margaret Thatcher")]
    MargaretThatcher,
    #[serde(rename = "Marge Simpson")]
    MargeSimpson,
    #[serde(rename = "Margot Robbie")]
    MargotRobbie,
    #[serde(rename = "Marilyn Monroe")]
    MarilynMonroe,
    #[serde(rename = "Mario")]
    Mario,
    #[serde(rename = "Marjorie Taylor Greene")]
    MarjorieTaylorGreene,
    #[serde(rename = "Mark Hamill")]
    MarkHamill,
    #[serde(rename = "Mark Rober")]
    MarkRober,
    #[serde(rename = "Mark Ruffalo")]
    MarkRuffalo,
    #[serde(rename = "Mark Wahlberg")]
    MarkWahlberg,
    #[serde(rename = "Mark Zuckerberg")]
    MarkZuckerberg,
    #[serde(rename = "Markiplier")]
    Markiplier,
    #[serde(rename = "Marques Brownlee")]
    MarquesBrownlee,
    #[serde(rename = "Marshmello")]
    Marshmello,
    #[serde(rename = "Marth (Fire Emblem)")]
    MarthFireEmblem,
    #[serde(rename = "Martha Stewart")]
    MarthaStewart,
    #[serde(rename = "Martin Luther King Jr.")]
    MartinLutherKingJr,
    #[serde(rename = "Master Chief")]
    MasterChief,
    #[serde(rename = "Mater (Cars)")]
    MaterCars,
    #[serde(rename = "Matt Rife")]
    MattRife,
    #[serde(rename = "Matt Smith")]
    MattSmith,
    #[serde(rename = "Matt Walsh")]
    MattWalsh,
    #[serde(rename = "Matthew McConaughey")]
    MatthewMcConaughey,
    #[serde(rename = "Maury (Big Mouth)")]
    MauryBigMouth,
    #[serde(rename = "Max Verstappen")]
    MaxVerstappen,
    #[serde(rename = "Maximus (Gladiator)")]
    MaximusGladiator,
    #[serde(rename = "Megan Thee Stallion")]
    MeganTheeStallion,
    #[serde(rename = "Megatron")]
    Megatron,
    #[serde(rename = "Mel Robbins")]
    MelRobbins,
    #[serde(rename = "Meliodas")]
    Meliodas,
    #[serde(rename = "Melisandre")]
    Melisandre,
    #[serde(rename = "Melissa McCarthy")]
    MelissaMcCarthy,
    #[serde(rename = "Mercy (Overwatch)")]
    MercyOverwatch,
    #[serde(rename = "Merlin (Sword in the Stone)")]
    MerlinSwordInTheStone,
    #[serde(rename = "Meryl Streep")]
    MerylStreep,
    #[serde(rename = "Metro Boomin")]
    MetroBoomin,
    #[serde(rename = "Micah Parsons")]
    MicahParsons,
    #[serde(rename = "Michael B. Jordan")]
    MichaelBJordan,
    #[serde(rename = "Michael Corleone")]
    MichaelCorleone,
    #[serde(rename = "Michael De Santa")]
    MichaelDeSanta,
    #[serde(rename = "Michael Jackson")]
    MichaelJackson,
    #[serde(rename = "Michael Knowles")]
    MichaelKnowles,
    #[serde(rename = "Michael Myers")]
    MichaelMyers,
    #[serde(rename = "Michael Phelps")]
    MichaelPhelps,
    #[serde(rename = "Michael Scott")]
    MichaelScott,
    #[serde(rename = "Michelle Obama")]
    MichelleObama,
    #[serde(rename = "Mick Jagger")]
    MickJagger,
    #[serde(rename = "Mickey Mouse")]
    MickeyMouse,
    #[serde(rename = "Mike Ehrmantraut")]
    MikeEhrmantraut,
    #[serde(rename = "Mike Pence")]
    MikePence,
    #[serde(rename = "Mike Rowe")]
    MikeRowe,
    #[serde(rename = "Mike Tyson")]
    MikeTyson,
    #[serde(rename = "Miley Cyrus")]
    MileyCyrus,
    #[serde(rename = "Millie Bobby Brown")]
    MillieBobbyBrown,
    #[serde(rename = "Mindy Kaling")]
    MindyKaling,
    #[serde(rename = "Mirabel (Encanto)")]
    MirabelEncanto,
    #[serde(rename = "Mitch McConnell")]
    MitchMcConnell,
    #[serde(rename = "Mizkif")]
    Mizkif,
    #[serde(rename = "Mob (Mob Psycho 100)")]
    MobMobPsycho100,
    #[serde(rename = "Moistcr1tikal")]
    Moistcr1tikal,
    #[serde(rename = "Mojo Jojo")]
    MojoJojo,
    #[serde(rename = "Monica Geller")]
    MonicaGeller,
    #[serde(rename = "Mordecai (Regular Show)")]
    MordecaiRegularShow,
    #[serde(rename = "Morgan Freeman")]
    MorganFreeman,
    #[serde(rename = "Morgan Wallen")]
    MorganWallen,
    #[serde(rename = "Morty Smith")]
    MortySmith,
    #[serde(rename = "Mr. Burns")]
    MrBurns,
    #[serde(rename = "Mr. Garrison")]
    MrGarrison,
    #[serde(rename = "Mr. Krabs")]
    MrKrabs,
    #[serde(rename = "MrBallen")]
    MrBallen,
    #[serde(rename = "MrBeast")]
    MrBeast,
    #[serde(rename = "Muhammad Ali")]
    MuhammadAli,
    #[serde(rename = "Mulan")]
    Mulan,
    #[serde(rename = "Mushu (Mulan)")]
    MushuMulan,
    #[serde(rename = "Myles Garrett")]
    MylesGarrett,
    #[serde(rename = "Nami (One Piece)")]
    NamiOnePiece,
    #[serde(rename = "Nancy Pelosi")]
    NancyPelosi,
    #[serde(rename = "Narendra Modi")]
    NarendraModi,
    #[serde(rename = "Naruto Uzumaki")]
    NarutoUzumaki,
    #[serde(rename = "Natalie Portman")]
    NataliePortman,
    #[serde(rename = "Nate Diaz")]
    NateDiaz,
    #[serde(rename = "Nathan Drake")]
    NathanDrake,
    #[serde(rename = "Natsu Dragneel")]
    NatsuDragneel,
    #[serde(rename = "Naval Ravikant")]
    NavalRavikant,
    #[serde(rename = "Ned Flanders")]
    NedFlanders,
    #[serde(rename = "Ned Fulmer")]
    NedFulmer,
    #[serde(rename = "Ned Stark")]
    NedStark,
    #[serde(rename = "Negan (Walking Dead)")]
    NeganWalkingDead,
    #[serde(rename = "Neil Armstrong")]
    NeilArmstrong,
    #[serde(rename = "Neil Gaiman")]
    NeilGaiman,
    #[serde(rename = "Neil deGrasse Tyson")]
    NeilDeGrasseTyson,
    #[serde(rename = "Nelson Mandela")]
    NelsonMandela,
    #[serde(rename = "Neymar")]
    Neymar,
    #[serde(rename = "Nezuko Kamado")]
    NezukoKamado,
    #[serde(rename = "Niall Horan")]
    NiallHoran,
    #[serde(rename = "Nick Fury")]
    NickFury,
    #[serde(rename = "Nicki Minaj")]
    NickiMinaj,
    #[serde(rename = "Nicolas Cage")]
    NicolasCage,
    #[serde(rename = "Nicole Kidman")]
    NicoleKidman,
    #[serde(rename = "Nikki Glaser")]
    NikkiGlaser,
    #[serde(rename = "NikkieTutorials")]
    NikkieTutorials,
    #[serde(rename = "Niko Bellic")]
    NikoBellic,
    #[serde(rename = "Nikola Jokic")]
    NikolaJokic,
    #[serde(rename = "Ninja")]
    Ninja,
    #[serde(rename = "Noah Kahan")]
    NoahKahan,
    #[serde(rename = "Noah Schnapp")]
    NoahSchnapp,
    #[serde(rename = "Nolan North")]
    NolanNorth,
    #[serde(rename = "Norman Bates")]
    NormanBates,
    #[serde(rename = "Northernlion")]
    Northernlion,
    #[serde(rename = "Notorious B.I.G.")]
    NotoriousBIG,
    #[serde(rename = "Novak Djokovic")]
    NovakDjokovic,
    #[serde(rename = "Obi-Wan Kenobi")]
    ObiWanKenobi,
    #[serde(rename = "Octavia Spencer")]
    OctaviaSpencer,
    #[serde(rename = "Olivia Rodrigo")]
    OliviaRodrigo,
    #[serde(rename = "Oprah Winfrey")]
    OprahWinfrey,
    #[serde(rename = "Optimus Prime")]
    OptimusPrime,
    #[serde(rename = "Oscar Isaac")]
    OscarIsaac,
    #[serde(rename = "Owen Wilson")]
    OwenWilson,
    #[serde(rename = "Ozuna")]
    Ozuna,
    #[serde(rename = "Ozzy Osbourne")]
    OzzyOsbourne,
    #[serde(rename = "PC Principal")]
    PcPrincipal,
    #[serde(rename = "Paris Hilton")]
    ParisHilton,
    #[serde(rename = "Park Seo Joon")]
    ParkSeoJoon,
    #[serde(rename = "Pat McAfee")]
    PatMcAfee,
    #[serde(rename = "Patrick Bateman")]
    PatrickBateman,
    #[serde(rename = "Patrick Mahomes")]
    PatrickMahomes,
    #[serde(rename = "Patrick Star")]
    PatrickStar,
    #[serde(rename = "Patrick Stewart")]
    PatrickStewart,
    #[serde(rename = "Patrick Warburton")]
    PatrickWarburton,
    #[serde(rename = "Paul Giamatti")]
    PaulGiamatti,
    #[serde(rename = "Paul McCartney")]
    PaulMcCartney,
    #[serde(rename = "Paul Mescal")]
    PaulMescal,
    #[serde(rename = "Pedro Pascal")]
    PedroPascal,
    #[serde(rename = "Penelope Cruz")]
    PenelopeCruz,
    #[serde(rename = "Pennywise")]
    Pennywise,
    #[serde(rename = "Peso Pluma")]
    PesoPluma,
    #[serde(rename = "Pestily")]
    Pestily,
    #[serde(rename = "Peter Attia")]
    PeterAttia,
    #[serde(rename = "Peter Griffin")]
    PeterGriffin,
    #[serde(rename = "Peter Thiel")]
    PeterThiel,
    #[serde(rename = "Petyr Baelish")]
    PetyrBaelish,
    #[serde(rename = "PewDiePie")]
    PewDiePie,
    #[serde(rename = "Peyton Manning")]
    PeytonManning,
    #[serde(rename = "Philip DeFranco")]
    PhilipDeFranco,
    #[serde(rename = "Phineas Flynn")]
    PhineasFlynn,
    #[serde(rename = "Phoebe Buffay")]
    PhoebeBuffay,
    #[serde(rename = "Picard")]
    Picard,
    #[serde(rename = "Piers Morgan")]
    PiersMorgan,
    #[serde(rename = "Pikachu")]
    Pikachu,
    #[serde(rename = "Pit (Kid Icarus)")]
    PitKidIcarus,
    #[serde(rename = "Plankton")]
    Plankton,
    #[serde(rename = "Playboi Carti")]
    PlayboiCarti,
    #[serde(rename = "PointCrow")]
    PointCrow,
    #[serde(rename = "Pokimane")]
    Pokimane,
    #[serde(rename = "Popeye")]
    Popeye,
    #[serde(rename = "Porky Pig")]
    PorkyPig,
    #[serde(rename = "Post Malone")]
    PostMalone,
    #[serde(rename = "Power (Chainsaw Man)")]
    PowerChainsawMan,
    #[serde(rename = "Prince")]
    Prince,
    #[serde(rename = "Prince Harry")]
    PrinceHarry,
    #[serde(rename = "Prince William")]
    PrinceWilliam,
    #[serde(rename = "Princess Carolyn")]
    PrincessCarolyn,
    #[serde(rename = "Princess Diana")]
    PrincessDiana,
    #[serde(rename = "Princess Leia")]
    PrincessLeia,
    #[serde(rename = "Priyanka Chopra")]
    PriyankaChopra,
    #[serde(rename = "Professor Farnsworth")]
    ProfessorFarnsworth,
    #[serde(rename = "Quackity")]
    Quackity,
    #[serde(rename = "Queen Elizabeth II")]
    QueenElizabethIi,
    #[serde(rename = "Quentin Tarantino")]
    QuentinTarantino,
    #[serde(rename = "Quin69")]
    Quin69,
    #[serde(rename = "RM (BTS)")]
    RmBts,
    #[serde(rename = "Rachel Green")]
    RachelGreen,
    #[serde(rename = "Rachel Maddow")]
    RachelMaddow,
    #[serde(rename = "Rachel Zegler")]
    RachelZegler,
    #[serde(rename = "Rafael Nadal")]
    RafaelNadal,
    #[serde(rename = "Rafiki (Lion King)")]
    RafikiLionKing,
    #[serde(rename = "Raiden (MGS)")]
    RaidenMgs,
    #[serde(rename = "Rainbow Dash")]
    RainbowDash,
    #[serde(rename = "Rambo")]
    Rambo,
    #[serde(rename = "Ranboo")]
    Ranboo,
    #[serde(rename = "Randall Park")]
    RandallPark,
    #[serde(rename = "Randy Marsh")]
    RandyMarsh,
    #[serde(rename = "Randy Orton")]
    RandyOrton,
    #[serde(rename = "Ranveer Singh")]
    RanveerSingh,
    #[serde(rename = "Rapunzel (Tangled)")]
    RapunzelTangled,
    #[serde(rename = "Rauw Alejandro")]
    RauwAlejandro,
    #[serde(rename = "Ray Charles")]
    RayCharles,
    #[serde(rename = "Reaper (Overwatch)")]
    ReaperOverwatch,
    #[serde(rename = "Reese Witherspoon")]
    ReeseWitherspoon,
    #[serde(rename = "Regina King")]
    ReginaKing,
    #[serde(rename = "Reigen Arataka")]
    ReigenArataka,
    #[serde(rename = "Reinhardt (Overwatch)")]
    ReinhardtOverwatch,
    #[serde(rename = "Remy (Ratatouille)")]
    RemyRatatouille,
    #[serde(rename = "Rey Mysterio")]
    ReyMysterio,
    #[serde(rename = "Rhett and Link")]
    RhettAndLink,
    #[serde(rename = "Ric Flair")]
    RicFlair,
    #[serde(rename = "Rich Eisen")]
    RichEisen,
    #[serde(rename = "Richard Feynman")]
    RichardFeynman,
    #[serde(rename = "Richard Hammond")]
    RichardHammond,
    #[serde(rename = "Rick Grimes")]
    RickGrimes,
    #[serde(rename = "Rick Sanchez")]
    RickSanchez,
    #[serde(rename = "Ricky Gervais")]
    RickyGervais,
    #[serde(rename = "Ricky Martin")]
    RickyMartin,
    #[serde(rename = "Rigby (Regular Show)")]
    RigbyRegularShow,
    #[serde(rename = "Rihanna")]
    Rihanna,
    #[serde(rename = "Riku (Kingdom Hearts)")]
    RikuKingdomHearts,
    #[serde(rename = "Rishi Sunak")]
    RishiSunak,
    #[serde(rename = "Riz Ahmed")]
    RizAhmed,
    #[serde(rename = "Robert De Niro")]
    RobertDeNiro,
    #[serde(rename = "Robert Downey Jr.")]
    RobertDowneyJr,
    #[serde(rename = "Robert Plant")]
    RobertPlant,
    #[serde(rename = "Robin Williams")]
    RobinWilliams,
    #[serde(rename = "Rocket Raccoon")]
    RocketRaccoon,
    #[serde(rename = "Rocky Balboa")]
    RockyBalboa,
    #[serde(rename = "Roger Federer")]
    RogerFederer,
    #[serde(rename = "Rohit Sharma")]
    RohitSharma,
    #[serde(rename = "Roman Reigns")]
    RomanReigns,
    #[serde(rename = "Ron DeSantis")]
    RonDeSantis,
    #[serde(rename = "Ron Swanson")]
    RonSwanson,
    #[serde(rename = "Ron Weasley")]
    RonWeasley,
    #[serde(rename = "Ronald Reagan")]
    RonaldReagan,
    #[serde(rename = "Ronda Rousey")]
    RondaRousey,
    #[serde(rename = "Rooney Mara")]
    RooneyMara,
    #[serde(rename = "Rosalia")]
    Rosalia,
    #[serde(rename = "Rose (Blackpink)")]
    RoseBlackpink,
    #[serde(rename = "Ross Geller")]
    RossGeller,
    #[serde(rename = "Rubius")]
    Rubius,
    #[serde(rename = "Rupert Grint")]
    RupertGrint,
    #[serde(rename = "Russell Brand")]
    RussellBrand,
    #[serde(rename = "Russell Westbrook")]
    RussellWestbrook,
    #[serde(rename = "Ryan Gosling")]
    RyanGosling,
    #[serde(rename = "Ryan Reynolds")]
    RyanReynolds,
    #[serde(rename = "Ryan Seacrest")]
    RyanSeacrest,
    #[serde(rename = "Ryu (Street Fighter)")]
    RyuStreetFighter,
    #[serde(rename = "Ryu Hayabusa")]
    RyuHayabusa,
    #[serde(rename = "SSSniperWolf")]
    SsSniperWolf,
    #[serde(rename = "SZA")]
    Sza,
    #[serde(rename = "Sabrina Carpenter")]
    SabrinaCarpenter,
    #[serde(rename = "Sachin Tendulkar")]
    SachinTendulkar,
    #[serde(rename = "Sadness (Inside Out)")]
    SadnessInsideOut,
    #[serde(rename = "Safiya Nygaard")]
    SafiyaNygaard,
    #[serde(rename = "Saitama (One Punch Man)")]
    SaitamaOnePunchMan,
    #[serde(rename = "Salma Hayek")]
    SalmaHayek,
    #[serde(rename = "Salman Khan")]
    SalmanKhan,
    #[serde(rename = "Sam Altman")]
    SamAltman,
    #[serde(rename = "Sam Fender")]
    SamFender,
    #[serde(rename = "Sam Harris")]
    SamHarris,
    #[serde(rename = "Sam Porter Bridges")]
    SamPorterBridges,
    #[serde(rename = "Sam Smith")]
    SamSmith,
    #[serde(rename = "Samuel L. Jackson")]
    SamuelLJackson,
    #[serde(rename = "Samus Aran")]
    SamusAran,
    #[serde(rename = "Samwise Gamgee")]
    SamwiseGamgee,
    #[serde(rename = "Sandra Bullock")]
    SandraBullock,
    #[serde(rename = "Sandy Cheeks")]
    SandyCheeks,
    #[serde(rename = "Sansa Stark")]
    SansaStark,
    #[serde(rename = "Sapnap")]
    Sapnap,
    #[serde(rename = "Sarah Connor")]
    SarahConnor,
    #[serde(rename = "Sasuke Uchiha")]
    SasukeUchiha,
    #[serde(rename = "Satya Nadella")]
    SatyaNadella,
    #[serde(rename = "Saul Goodman")]
    SaulGoodman,
    #[serde(rename = "Scarlett Johansson")]
    ScarlettJohansson,
    #[serde(rename = "Scooby-Doo")]
    ScoobyDoo,
    #[serde(rename = "Scorpion (Mortal Kombat)")]
    ScorpionMortalKombat,
    #[serde(rename = "Scuttle (Little Mermaid)")]
    ScuttleLittleMermaid,
    #[serde(rename = "Sean Connery")]
    SeanConnery,
    #[serde(rename = "Sean Hannity")]
    SeanHannity,
    #[serde(rename = "Sebastian (Little Mermaid)")]
    SebastianLittleMermaid,
    #[serde(rename = "Sebastian Maniscalco")]
    SebastianManiscalco,
    #[serde(rename = "Selena Gomez")]
    SelenaGomez,
    #[serde(rename = "Sephiroth")]
    Sephiroth,
    #[serde(rename = "Serena Williams")]
    SerenaWilliams,
    #[serde(rename = "Seth MacFarlane")]
    SethMacFarlane,
    #[serde(rename = "Seth Meyers")]
    SethMeyers,
    #[serde(rename = "Seth Rogen")]
    SethRogen,
    #[serde(rename = "Seth Rollins")]
    SethRollins,
    #[serde(rename = "Severus Snape")]
    SeverusSnape,
    #[serde(rename = "Sexyy Red")]
    SexyyRed,
    #[serde(rename = "Sha'Carri Richardson")]
    ShaCarriRichardson,
    #[serde(rename = "Shaggy Rogers")]
    ShaggyRogers,
    #[serde(rename = "Shah Rukh Khan")]
    ShahRukhKhan,
    #[serde(rename = "Shane Dawson")]
    ShaneDawson,
    #[serde(rename = "Shane Gillis")]
    ShaneGillis,
    #[serde(rename = "Shannon Sharpe")]
    ShannonSharpe,
    #[serde(rename = "Shaquille O'Neal")]
    ShaquilleONeal,
    #[serde(rename = "Shawn Mendes")]
    ShawnMendes,
    #[serde(rename = "Sheldon Cooper")]
    SheldonCooper,
    #[serde(rename = "Sherlock Holmes")]
    SherlockHolmes,
    #[serde(rename = "Sheryl Sandberg")]
    SherylSandberg,
    #[serde(rename = "Shohei Ohtani")]
    ShoheiOhtani,
    #[serde(rename = "Shrek")]
    Shrek,
    #[serde(rename = "Shroud")]
    Shroud,
    #[serde(rename = "Sia")]
    Sia,
    #[serde(rename = "Sidney Crosby")]
    SidneyCrosby,
    #[serde(rename = "Simon Sinek")]
    SimonSinek,
    #[serde(rename = "Simone Biles")]
    SimoneBiles,
    #[serde(rename = "Simu Liu")]
    SimuLiu,
    #[serde(rename = "Skeletor (He-Man)")]
    SkeletorHeMan,
    #[serde(rename = "Skip Bayless")]
    SkipBayless,
    #[serde(rename = "Skrillex")]
    Skrillex,
    #[serde(rename = "Skyler White")]
    SkylerWhite,
    #[serde(rename = "Slash (musician)")]
    SlashMusician,
    #[serde(rename = "Snoop Dogg")]
    SnoopDogg,
    #[serde(rename = "Snow White")]
    SnowWhite,
    #[serde(rename = "Sodapoppin")]
    Sodapoppin,
    #[serde(rename = "Sofia Vergara")]
    SofiaVergara,
    #[serde(rename = "Solid Snake")]
    SolidSnake,
    #[serde(rename = "Song Joong Ki")]
    SongJoongKi,
    #[serde(rename = "Sonic the Hedgehog")]
    SonicTheHedgehog,
    #[serde(rename = "Sora (Kingdom Hearts)")]
    SoraKingdomHearts,
    #[serde(rename = "SpiderMan (Tom Holland ver.)")]
    SpiderManTomHollandVer,
    #[serde(rename = "Spike Spiegel")]
    SpikeSpiegel,
    #[serde(rename = "SpongeBob SquarePants")]
    SpongeBobSquarePants,
    #[serde(rename = "Spyro")]
    Spyro,
    #[serde(rename = "Squidward Tentacles")]
    SquidwardTentacles,
    #[serde(rename = "Stan Marsh")]
    StanMarsh,
    #[serde(rename = "Stanley Hudson")]
    StanleyHudson,
    #[serde(rename = "Stephen A Smith")]
    StephenASmith,
    #[serde(rename = "Stephen Colbert")]
    StephenColbert,
    #[serde(rename = "Stephen Curry")]
    StephenCurry,
    #[serde(rename = "Stephen Hawking")]
    StephenHawking,
    #[serde(rename = "Stephen King")]
    StephenKing,
    #[serde(rename = "Steve (Minecraft)")]
    SteveMinecraft,
    #[serde(rename = "Steve Blum")]
    SteveBlum,
    #[serde(rename = "Steve Carell")]
    SteveCarell,
    #[serde(rename = "Steve Harvey")]
    SteveHarvey,
    #[serde(rename = "Steve Irwin")]
    SteveIrwin,
    #[serde(rename = "Steve Jobs")]
    SteveJobs,
    #[serde(rename = "Steve-O")]
    SteveO,
    #[serde(rename = "Steven Crowder")]
    StevenCrowder,
    #[serde(rename = "Steven Universe")]
    StevenUniverse,
    #[serde(rename = "Stevie Wonder")]
    StevieWonder,
    #[serde(rename = "Stewie Griffin")]
    StewieGriffin,
    #[serde(rename = "Stone Cold Steve Austin")]
    StoneColdSteveAustin,
    #[serde(rename = "Stormzy")]
    Stormzy,
    #[serde(rename = "Sub-Zero")]
    SubZero,
    #[serde(rename = "Summer Smith")]
    SummerSmith,
    #[serde(rename = "Summit1g")]
    Summit1g,
    #[serde(rename = "Sundar Pichai")]
    SundarPichai,
    #[serde(rename = "Superman")]
    Superman,
    #[serde(rename = "Sydney Sweeney")]
    SydneySweeney,
    #[serde(rename = "Sykkuno")]
    Sykkuno,
    #[serde(rename = "Sylvester Stallone")]
    SylvesterStallone,
    #[serde(rename = "Sylvester the Cat")]
    SylvesterTheCat,
    #[serde(rename = "Tana Mongeau")]
    TanaMongeau,
    #[serde(rename = "Tanjiro Kamado")]
    TanjiroKamado,
    #[serde(rename = "Tara Strong")]
    TaraStrong,
    #[serde(rename = "Taraji P. Henson")]
    TarajiPHenson,
    #[serde(rename = "Tate McRae")]
    TateMcRae,
    #[serde(rename = "Taylor Swift")]
    TaylorSwift,
    #[serde(rename = "Technoblade")]
    Technoblade,
    #[serde(rename = "Tems")]
    Tems,
    #[serde(rename = "Terminator")]
    Terminator,
    #[serde(rename = "Tfue")]
    Tfue,
    #[serde(rename = "Thanos")]
    Thanos,
    #[serde(rename = "The Dude (Big Lebowski)")]
    TheDudeBigLebowski,
    #[serde(rename = "The Grinch")]
    TheGrinch,
    #[serde(rename = "The Hound")]
    TheHound,
    #[serde(rename = "The Joker (Animated)")]
    TheJokerAnimated,
    #[serde(rename = "The Mandalorian")]
    TheMandalorian,
    #[serde(rename = "The Rock")]
    TheRock,
    #[serde(rename = "The Undertaker")]
    TheUndertaker,
    #[serde(rename = "The Weeknd")]
    TheWeeknd,
    #[serde(rename = "Theo Von")]
    TheoVon,
    #[serde(rename = "Thor (MCU)")]
    ThorMcu,
    #[serde(rename = "Tiger Woods")]
    TigerWoods,
    #[serde(rename = "Tim Cook")]
    TimCook,
    #[serde(rename = "Tim Dillon")]
    TimDillon,
    #[serde(rename = "Timmy Turner")]
    TimmyTurner,
    #[serde(rename = "Timothee Chalamet")]
    TimotheeChalamet,
    #[serde(rename = "Timthetatman")]
    Timthetatman,
    #[serde(rename = "Toby Flenderson")]
    TobyFlenderson,
    #[serde(rename = "Todd Chavez")]
    ToddChavez,
    #[serde(rename = "Tom Brady")]
    TomBrady,
    #[serde(rename = "Tom Cruise")]
    TomCruise,
    #[serde(rename = "Tom Hanks")]
    TomHanks,
    #[serde(rename = "Tom Hardy")]
    TomHardy,
    #[serde(rename = "Tom Haverford")]
    TomHaverford,
    #[serde(rename = "Tom Hiddleston")]
    TomHiddleston,
    #[serde(rename = "Tom Holland")]
    TomHolland,
    #[serde(rename = "Tom Kenny")]
    TomKenny,
    #[serde(rename = "Tom Segura")]
    TomSegura,
    #[serde(rename = "Tommy Shelby")]
    TommyShelby,
    #[serde(rename = "Tony Hawk")]
    TonyHawk,
    #[serde(rename = "Tony Montana (Scarface)")]
    TonyMontanaScarface,
    #[serde(rename = "Tony Soprano")]
    TonySoprano,
    #[serde(rename = "Tony Stark")]
    TonyStark,
    #[serde(rename = "Towelie")]
    Towelie,
    #[serde(rename = "Tracer (Overwatch)")]
    TracerOverwatch,
    #[serde(rename = "Trainwreckstv")]
    Trainwreckstv,
    #[serde(rename = "Travis Bickle")]
    TravisBickle,
    #[serde(rename = "Travis Kelce")]
    TravisKelce,
    #[serde(rename = "Travis Pastrana")]
    TravisPastrana,
    #[serde(rename = "Travis Scott")]
    TravisScott,
    #[serde(rename = "Trevor Noah")]
    TrevorNoah,
    #[serde(rename = "Trevor Philips")]
    TrevorPhilips,
    #[serde(rename = "Trevor Wallace")]
    TrevorWallace,
    #[serde(rename = "Triple H")]
    TripleH,
    #[serde(rename = "Trisha Paytas")]
    TrishaPaytas,
    #[serde(rename = "Troy Baker")]
    TroyBaker,
    #[serde(rename = "Troye Sivan")]
    TroyeSivan,
    #[serde(rename = "Tubbo")]
    Tubbo,
    #[serde(rename = "Tucker Carlson")]
    TuckerCarlson,
    #[serde(rename = "Tupac Shakur")]
    TupacShakur,
    #[serde(rename = "Tweety Bird")]
    TweetyBird,
    #[serde(rename = "Twilight Sparkle")]
    TwilightSparkle,
    #[serde(rename = "Tyler Durden")]
    TylerDurden,
    #[serde(rename = "Tyler, The Creator")]
    TylerTheCreator,
    #[serde(rename = "Tyrion Lannister")]
    TyrionLannister,
    #[serde(rename = "Tyson Fury")]
    TysonFury,
    #[serde(rename = "Ursula (Little Mermaid)")]
    UrsulaLittleMermaid,
    #[serde(rename = "Usain Bolt")]
    UsainBolt,
    #[serde(rename = "Usher")]
    Usher,
    #[serde(rename = "V (BTS)")]
    VBts,
    #[serde(rename = "V (Cyberpunk)")]
    VCyberpunk,
    #[serde(rename = "Valentino Rossi")]
    ValentinoRossi,
    #[serde(rename = "Valkyrae")]
    Valkyrae,
    #[serde(rename = "Valorant Jett")]
    ValorantJett,
    #[serde(rename = "Vanellope von Schweetz")]
    VanellopeVonSchweetz,
    #[serde(rename = "Varys")]
    Varys,
    #[serde(rename = "Vash the Stampede")]
    VashTheStampede,
    #[serde(rename = "Vegeta")]
    Vegeta,
    #[serde(rename = "Vegetta777")]
    Vegetta777,
    #[serde(rename = "Venom")]
    Venom,
    #[serde(rename = "Venus Williams")]
    VenusWilliams,
    #[serde(rename = "Vergil (DMC)")]
    VergilDmc,
    #[serde(rename = "Vi (Arcane)")]
    ViArcane,
    #[serde(rename = "Victor Wembanyama")]
    VictorWembanyama,
    #[serde(rename = "Vin Diesel")]
    VinDiesel,
    #[serde(rename = "Vinesauce Vinny")]
    VinesauceVinny,
    #[serde(rename = "Vinicius Junior")]
    ViniciusJunior,
    #[serde(rename = "Viola Davis")]
    ViolaDavis,
    #[serde(rename = "Virat Kohli")]
    ViratKohli,
    #[serde(rename = "Vito Corleone")]
    VitoCorleone,
    #[serde(rename = "Vivek Ramaswamy")]
    VivekRamaswamy,
    #[serde(rename = "Vladimir Putin")]
    VladimirPutin,
    #[serde(rename = "Voldemort")]
    Voldemort,
    #[serde(rename = "Volodymyr Zelenskyy")]
    VolodymyrZelenskyy,
    #[serde(rename = "Walter Bishop (Fringe)")]
    WalterBishopFringe,
    #[serde(rename = "Walter White")]
    WalterWhite,
    #[serde(rename = "Warren Buffett")]
    WarrenBuffett,
    #[serde(rename = "Wendy Testaburger")]
    WendyTestaburger,
    #[serde(rename = "Werner Herzog")]
    WernerHerzog,
    #[serde(rename = "Whitney Cummings")]
    WhitneyCummings,
    #[serde(rename = "Whitney Houston")]
    WhitneyHouston,
    #[serde(rename = "Widowmaker (Overwatch)")]
    WidowmakerOverwatch,
    #[serde(rename = "Wilbur Soot")]
    WilburSoot,
    #[serde(rename = "Will Ferrell")]
    WillFerrell,
    #[serde(rename = "Will Smith")]
    WillSmith,
    #[serde(rename = "Willem Dafoe")]
    WillemDafoe,
    #[serde(rename = "William Wallace")]
    WilliamWallace,
    #[serde(rename = "Willie Nelson")]
    WillieNelson,
    #[serde(rename = "Willy Wonka")]
    WillyWonka,
    #[serde(rename = "Winnie the Pooh")]
    WinnieThePooh,
    #[serde(rename = "Winston Churchill")]
    WinstonChurchill,
    #[serde(rename = "Wizkid")]
    Wizkid,
    #[serde(rename = "Wolf Blitzer")]
    WolfBlitzer,
    #[serde(rename = "Wolverine")]
    Wolverine,
    #[serde(rename = "Wonder Woman")]
    WonderWoman,
    #[serde(rename = "Woody (Toy Story)")]
    WoodyToyStory,
    #[serde(rename = "Wreck-It Ralph")]
    WreckItRalph,
    #[serde(rename = "Xi Jinping")]
    XiJinping,
    #[serde(rename = "Yasuo (LoL)")]
    YasuoLoL,
    #[serde(rename = "Yoda")]
    Yoda,
    #[serde(rename = "Yor Forger")]
    YorForger,
    #[serde(rename = "Yoshi")]
    Yoshi,
    #[serde(rename = "Yuri Gagarin")]
    YuriGagarin,
    #[serde(rename = "Yusuke Urameshi")]
    YusukeUrameshi,
    #[serde(rename = "Yzma (Emperor's New Groove)")]
    YzmaEmperorSNewGroove,
    #[serde(rename = "Zach Bryan")]
    ZachBryan,
    #[serde(rename = "Zach King")]
    ZachKing,
    #[serde(rename = "Zach Kornfeld")]
    ZachKornfeld,
    #[serde(rename = "Zane Lowe")]
    ZaneLowe,
    #[serde(rename = "Zapp Brannigan")]
    ZappBrannigan,
    #[serde(rename = "Zazu (Lion King)")]
    ZazuLionKing,
    #[serde(rename = "Zedd")]
    Zedd,
    #[serde(rename = "Zendaya")]
    Zendaya,
    #[serde(rename = "Zenitsu Agatsuma")]
    ZenitsuAgatsuma,
    #[serde(rename = "Zoe Saldana")]
    ZoeSaldana,
    #[serde(rename = "Zoro (One Piece)")]
    ZoroOnePiece,
    #[serde(rename = "Zuko (Avatar)")]
    ZukoAvatar,
    #[serde(rename = "cv-0001")]
    Cv0001,
    #[serde(rename = "cv-0002")]
    Cv0002,
    #[serde(rename = "cv-0003")]
    Cv0003,
    #[serde(rename = "cv-0004")]
    Cv0004,
    #[serde(rename = "cv-0005")]
    Cv0005,
    #[serde(rename = "cv-0006")]
    Cv0006,
    #[serde(rename = "cv-0007")]
    Cv0007,
    #[serde(rename = "cv-0008")]
    Cv0008,
    #[serde(rename = "cv-0009")]
    Cv0009,
    #[serde(rename = "cv-0010")]
    Cv0010,
    #[serde(rename = "cv-0011")]
    Cv0011,
    #[serde(rename = "cv-0012")]
    Cv0012,
    #[serde(rename = "cv-0013")]
    Cv0013,
    #[serde(rename = "cv-0014")]
    Cv0014,
    #[serde(rename = "cv-0015")]
    Cv0015,
    #[serde(rename = "cv-0016")]
    Cv0016,
    #[serde(rename = "cv-0017")]
    Cv0017,
    #[serde(rename = "cv-0018")]
    Cv0018,
    #[serde(rename = "cv-0019")]
    Cv0019,
    #[serde(rename = "cv-0020")]
    Cv0020,
    #[serde(rename = "cv-0021")]
    Cv0021,
    #[serde(rename = "cv-0022")]
    Cv0022,
    #[serde(rename = "cv-0023")]
    Cv0023,
    #[serde(rename = "cv-0024")]
    Cv0024,
    #[serde(rename = "cv-0025")]
    Cv0025,
    #[serde(rename = "cv-0026")]
    Cv0026,
    #[serde(rename = "cv-0028")]
    Cv0028,
    #[serde(rename = "cv-0029")]
    Cv0029,
    #[serde(rename = "cv-0030")]
    Cv0030,
    #[serde(rename = "cv-0031")]
    Cv0031,
    #[serde(rename = "cv-0032")]
    Cv0032,
    #[serde(rename = "cv-0033")]
    Cv0033,
    #[serde(rename = "cv-0034")]
    Cv0034,
    #[serde(rename = "cv-0035")]
    Cv0035,
    #[serde(rename = "cv-0036")]
    Cv0036,
    #[serde(rename = "cv-0037")]
    Cv0037,
    #[serde(rename = "cv-0038")]
    Cv0038,
    #[serde(rename = "cv-0039")]
    Cv0039,
    #[serde(rename = "cv-0040")]
    Cv0040,
    #[serde(rename = "cv-0041")]
    Cv0041,
    #[serde(rename = "cv-0042")]
    Cv0042,
    #[serde(rename = "cv-0043")]
    Cv0043,
    #[serde(rename = "cv-0044")]
    Cv0044,
    #[serde(rename = "cv-0045")]
    Cv0045,
    #[serde(rename = "cv-0046")]
    Cv0046,
    #[serde(rename = "cv-0047")]
    Cv0047,
    #[serde(rename = "cv-0048")]
    Cv0048,
    #[serde(rename = "cv-0049")]
    Cv0049,
    #[serde(rename = "cv-0050")]
    Cv0050,
    #[serde(rename = "cv-0051")]
    Cv0051,
    #[serde(rename = "cv-0052")]
    Cv0052,
    #[serde(rename = "cv-0053")]
    Cv0053,
    #[serde(rename = "cv-0054")]
    Cv0054,
    #[serde(rename = "cv-0055")]
    Cv0055,
    #[serde(rename = "cv-0056")]
    Cv0056,
    #[serde(rename = "cv-0057")]
    Cv0057,
    #[serde(rename = "cv-0058")]
    Cv0058,
    #[serde(rename = "cv-0059")]
    Cv0059,
    #[serde(rename = "cv-0060")]
    Cv0060,
    #[serde(rename = "cv-0061")]
    Cv0061,
    #[serde(rename = "cv-0062")]
    Cv0062,
    #[serde(rename = "cv-0063")]
    Cv0063,
    #[serde(rename = "cv-0064")]
    Cv0064,
    #[serde(rename = "cv-0065")]
    Cv0065,
    #[serde(rename = "cv-0066")]
    Cv0066,
    #[serde(rename = "cv-0067")]
    Cv0067,
    #[serde(rename = "cv-0068")]
    Cv0068,
    #[serde(rename = "cv-0069")]
    Cv0069,
    #[serde(rename = "cv-0070")]
    Cv0070,
    #[serde(rename = "cv-0071")]
    Cv0071,
    #[serde(rename = "cv-0072")]
    Cv0072,
    #[serde(rename = "cv-0073")]
    Cv0073,
    #[serde(rename = "cv-0074")]
    Cv0074,
    #[serde(rename = "cv-0075")]
    Cv0075,
    #[serde(rename = "cv-0076")]
    Cv0076,
    #[serde(rename = "cv-0077")]
    Cv0077,
    #[serde(rename = "cv-0078")]
    Cv0078,
    #[serde(rename = "cv-0079")]
    Cv0079,
    #[serde(rename = "cv-0080")]
    Cv0080,
    #[serde(rename = "cv-0081")]
    Cv0081,
    #[serde(rename = "cv-0082")]
    Cv0082,
    #[serde(rename = "cv-0083")]
    Cv0083,
    #[serde(rename = "cv-0084")]
    Cv0084,
    #[serde(rename = "cv-0085")]
    Cv0085,
    #[serde(rename = "cv-0086")]
    Cv0086,
    #[serde(rename = "cv-0087")]
    Cv0087,
    #[serde(rename = "cv-0088")]
    Cv0088,
    #[serde(rename = "cv-0089")]
    Cv0089,
    #[serde(rename = "cv-0090")]
    Cv0090,
    #[serde(rename = "cv-0091")]
    Cv0091,
    #[serde(rename = "cv-0092")]
    Cv0092,
    #[serde(rename = "cv-0093")]
    Cv0093,
    #[serde(rename = "cv-0094")]
    Cv0094,
    #[serde(rename = "cv-0095")]
    Cv0095,
    #[serde(rename = "cv-0096")]
    Cv0096,
    #[serde(rename = "cv-0097")]
    Cv0097,
    #[serde(rename = "cv-0098")]
    Cv0098,
    #[serde(rename = "cv-0099")]
    Cv0099,
    #[serde(rename = "cv-0100")]
    Cv0100,
    #[serde(rename = "cv-0101")]
    Cv0101,
    #[serde(rename = "cv-0102")]
    Cv0102,
    #[serde(rename = "cv-0103")]
    Cv0103,
    #[serde(rename = "cv-0104")]
    Cv0104,
    #[serde(rename = "cv-0105")]
    Cv0105,
    #[serde(rename = "cv-0106")]
    Cv0106,
    #[serde(rename = "cv-0107")]
    Cv0107,
    #[serde(rename = "cv-0108")]
    Cv0108,
    #[serde(rename = "cv-0109")]
    Cv0109,
    #[serde(rename = "cv-0110")]
    Cv0110,
    #[serde(rename = "cv-0111")]
    Cv0111,
    #[serde(rename = "cv-0112")]
    Cv0112,
    #[serde(rename = "cv-0113")]
    Cv0113,
    #[serde(rename = "cv-0114")]
    Cv0114,
    #[serde(rename = "cv-0115")]
    Cv0115,
    #[serde(rename = "cv-0116")]
    Cv0116,
    #[serde(rename = "cv-0117")]
    Cv0117,
    #[serde(rename = "cv-0118")]
    Cv0118,
    #[serde(rename = "cv-0119")]
    Cv0119,
    #[serde(rename = "cv-0120")]
    Cv0120,
    #[serde(rename = "cv-0121")]
    Cv0121,
    #[serde(rename = "cv-0122")]
    Cv0122,
    #[serde(rename = "cv-0123")]
    Cv0123,
    #[serde(rename = "cv-0124")]
    Cv0124,
    #[serde(rename = "cv-0125")]
    Cv0125,
    #[serde(rename = "cv-0126")]
    Cv0126,
    #[serde(rename = "cv-0127")]
    Cv0127,
    #[serde(rename = "cv-0128")]
    Cv0128,
    #[serde(rename = "cv-0129")]
    Cv0129,
    #[serde(rename = "cv-0130")]
    Cv0130,
    #[serde(rename = "cv-0131")]
    Cv0131,
    #[serde(rename = "cv-0132")]
    Cv0132,
    #[serde(rename = "cv-0133")]
    Cv0133,
    #[serde(rename = "cv-0134")]
    Cv0134,
    #[serde(rename = "cv-0135")]
    Cv0135,
    #[serde(rename = "cv-0136")]
    Cv0136,
    #[serde(rename = "cv-0137")]
    Cv0137,
    #[serde(rename = "cv-0138")]
    Cv0138,
    #[serde(rename = "cv-0139")]
    Cv0139,
    #[serde(rename = "cv-0140")]
    Cv0140,
    #[serde(rename = "cv-0141")]
    Cv0141,
    #[serde(rename = "cv-0142")]
    Cv0142,
    #[serde(rename = "cv-0143")]
    Cv0143,
    #[serde(rename = "cv-0144")]
    Cv0144,
    #[serde(rename = "cv-0145")]
    Cv0145,
    #[serde(rename = "cv-0146")]
    Cv0146,
    #[serde(rename = "cv-0147")]
    Cv0147,
    #[serde(rename = "cv-0148")]
    Cv0148,
    #[serde(rename = "cv-0149")]
    Cv0149,
    #[serde(rename = "cv-0150")]
    Cv0150,
    #[serde(rename = "cv-0151")]
    Cv0151,
    #[serde(rename = "cv-0152")]
    Cv0152,
    #[serde(rename = "cv-0153")]
    Cv0153,
    #[serde(rename = "cv-0154")]
    Cv0154,
    #[serde(rename = "cv-0155")]
    Cv0155,
    #[serde(rename = "cv-0156")]
    Cv0156,
    #[serde(rename = "cv-0157")]
    Cv0157,
    #[serde(rename = "cv-0158")]
    Cv0158,
    #[serde(rename = "cv-0159")]
    Cv0159,
    #[serde(rename = "cv-0160")]
    Cv0160,
    #[serde(rename = "cv-0161")]
    Cv0161,
    #[serde(rename = "cv-0162")]
    Cv0162,
    #[serde(rename = "cv-0163")]
    Cv0163,
    #[serde(rename = "cv-0164")]
    Cv0164,
    #[serde(rename = "cv-0165")]
    Cv0165,
    #[serde(rename = "cv-0166")]
    Cv0166,
    #[serde(rename = "cv-0167")]
    Cv0167,
    #[serde(rename = "cv-0168")]
    Cv0168,
    #[serde(rename = "cv-0169")]
    Cv0169,
    #[serde(rename = "cv-0170")]
    Cv0170,
    #[serde(rename = "cv-0171")]
    Cv0171,
    #[serde(rename = "cv-0172")]
    Cv0172,
    #[serde(rename = "cv-0173")]
    Cv0173,
    #[serde(rename = "cv-0174")]
    Cv0174,
    #[serde(rename = "cv-0175")]
    Cv0175,
    #[serde(rename = "cv-0176")]
    Cv0176,
    #[serde(rename = "cv-0177")]
    Cv0177,
    #[serde(rename = "cv-0178")]
    Cv0178,
    #[serde(rename = "cv-0179")]
    Cv0179,
    #[serde(rename = "cv-0180")]
    Cv0180,
    #[serde(rename = "cv-0181")]
    Cv0181,
    #[serde(rename = "cv-0182")]
    Cv0182,
    #[serde(rename = "cv-0183")]
    Cv0183,
    #[serde(rename = "cv-0184")]
    Cv0184,
    #[serde(rename = "cv-0185")]
    Cv0185,
    #[serde(rename = "cv-0186")]
    Cv0186,
    #[serde(rename = "cv-0187")]
    Cv0187,
    #[serde(rename = "cv-0188")]
    Cv0188,
    #[serde(rename = "cv-0189")]
    Cv0189,
    #[serde(rename = "cv-0190")]
    Cv0190,
    #[serde(rename = "cv-0191")]
    Cv0191,
    #[serde(rename = "cv-0192")]
    Cv0192,
    #[serde(rename = "cv-0193")]
    Cv0193,
    #[serde(rename = "cv-0194")]
    Cv0194,
    #[serde(rename = "cv-0195")]
    Cv0195,
    #[serde(rename = "cv-0196")]
    Cv0196,
    #[serde(rename = "cv-0197")]
    Cv0197,
    #[serde(rename = "cv-0198")]
    Cv0198,
    #[serde(rename = "cv-0199")]
    Cv0199,
    #[serde(rename = "cv-0200")]
    Cv0200,
    #[serde(rename = "cv-0201")]
    Cv0201,
    #[serde(rename = "cv-0202")]
    Cv0202,
    #[serde(rename = "cv-0203")]
    Cv0203,
    #[serde(rename = "cv-0205")]
    Cv0205,
    #[serde(rename = "cv-0206")]
    Cv0206,
    #[serde(rename = "cv-0207")]
    Cv0207,
    #[serde(rename = "cv-0208")]
    Cv0208,
    #[serde(rename = "cv-0209")]
    Cv0209,
    #[serde(rename = "cv-0210")]
    Cv0210,
    #[serde(rename = "cv-0211")]
    Cv0211,
    #[serde(rename = "cv-0212")]
    Cv0212,
    #[serde(rename = "cv-0213")]
    Cv0213,
    #[serde(rename = "cv-0214")]
    Cv0214,
    #[serde(rename = "cv-0215")]
    Cv0215,
    #[serde(rename = "cv-0216")]
    Cv0216,
    #[serde(rename = "cv-0217")]
    Cv0217,
    #[serde(rename = "cv-0218")]
    Cv0218,
    #[serde(rename = "cv-0219")]
    Cv0219,
    #[serde(rename = "cv-0220")]
    Cv0220,
    #[serde(rename = "cv-0221")]
    Cv0221,
    #[serde(rename = "cv-0222")]
    Cv0222,
    #[serde(rename = "cv-0223")]
    Cv0223,
    #[serde(rename = "cv-0224")]
    Cv0224,
    #[serde(rename = "cv-0225")]
    Cv0225,
    #[serde(rename = "cv-0226")]
    Cv0226,
    #[serde(rename = "cv-0227")]
    Cv0227,
    #[serde(rename = "cv-0228")]
    Cv0228,
    #[serde(rename = "cv-0229")]
    Cv0229,
    #[serde(rename = "cv-0230")]
    Cv0230,
    #[serde(rename = "cv-0231")]
    Cv0231,
    #[serde(rename = "cv-0232")]
    Cv0232,
    #[serde(rename = "cv-0233")]
    Cv0233,
    #[serde(rename = "cv-0234")]
    Cv0234,
    #[serde(rename = "cv-0235")]
    Cv0235,
    #[serde(rename = "cv-0236")]
    Cv0236,
    #[serde(rename = "cv-0237")]
    Cv0237,
    #[serde(rename = "cv-0238")]
    Cv0238,
    #[serde(rename = "cv-0239")]
    Cv0239,
    #[serde(rename = "cv-0240")]
    Cv0240,
    #[serde(rename = "cv-0241")]
    Cv0241,
    #[serde(rename = "cv-0242")]
    Cv0242,
    #[serde(rename = "cv-0243")]
    Cv0243,
    #[serde(rename = "cv-0244")]
    Cv0244,
    #[serde(rename = "cv-0245")]
    Cv0245,
    #[serde(rename = "cv-0246")]
    Cv0246,
    #[serde(rename = "cv-0247")]
    Cv0247,
    #[serde(rename = "cv-0248")]
    Cv0248,
    #[serde(rename = "cv-0249")]
    Cv0249,
    #[serde(rename = "cv-0250")]
    Cv0250,
    #[serde(rename = "cv-0251")]
    Cv0251,
    #[serde(rename = "cv-0252")]
    Cv0252,
    #[serde(rename = "cv-0253")]
    Cv0253,
    #[serde(rename = "cv-0254")]
    Cv0254,
    #[serde(rename = "cv-0255")]
    Cv0255,
    #[serde(rename = "cv-0256")]
    Cv0256,
    #[serde(rename = "cv-0257")]
    Cv0257,
    #[serde(rename = "cv-0258")]
    Cv0258,
    #[serde(rename = "cv-0259")]
    Cv0259,
    #[serde(rename = "cv-0260")]
    Cv0260,
    #[serde(rename = "cv-0261")]
    Cv0261,
    #[serde(rename = "cv-0262")]
    Cv0262,
    #[serde(rename = "cv-0263")]
    Cv0263,
    #[serde(rename = "cv-0264")]
    Cv0264,
    #[serde(rename = "cv-0265")]
    Cv0265,
    #[serde(rename = "cv-0266")]
    Cv0266,
    #[serde(rename = "cv-0267")]
    Cv0267,
    #[serde(rename = "cv-0268")]
    Cv0268,
    #[serde(rename = "cv-0269")]
    Cv0269,
    #[serde(rename = "cv-0270")]
    Cv0270,
    #[serde(rename = "cv-0271")]
    Cv0271,
    #[serde(rename = "cv-0272")]
    Cv0272,
    #[serde(rename = "cv-0273")]
    Cv0273,
    #[serde(rename = "cv-0274")]
    Cv0274,
    #[serde(rename = "cv-0275")]
    Cv0275,
    #[serde(rename = "cv-0276")]
    Cv0276,
    #[serde(rename = "cv-0277")]
    Cv0277,
    #[serde(rename = "cv-0278")]
    Cv0278,
    #[serde(rename = "cv-0279")]
    Cv0279,
    #[serde(rename = "cv-0280")]
    Cv0280,
    #[serde(rename = "cv-0281")]
    Cv0281,
    #[serde(rename = "cv-0282")]
    Cv0282,
    #[serde(rename = "cv-0283")]
    Cv0283,
    #[serde(rename = "cv-0284")]
    Cv0284,
    #[serde(rename = "cv-0285")]
    Cv0285,
    #[serde(rename = "cv-0286")]
    Cv0286,
    #[serde(rename = "cv-0287")]
    Cv0287,
    #[serde(rename = "cv-0288")]
    Cv0288,
    #[serde(rename = "cv-0289")]
    Cv0289,
    #[serde(rename = "cv-0290")]
    Cv0290,
    #[serde(rename = "cv-0291")]
    Cv0291,
    #[serde(rename = "cv-0292")]
    Cv0292,
    #[serde(rename = "cv-0293")]
    Cv0293,
    #[serde(rename = "cv-0294")]
    Cv0294,
    #[serde(rename = "cv-0295")]
    Cv0295,
    #[serde(rename = "cv-0296")]
    Cv0296,
    #[serde(rename = "cv-0297")]
    Cv0297,
    #[serde(rename = "cv-0298")]
    Cv0298,
    #[serde(rename = "cv-0299")]
    Cv0299,
    #[serde(rename = "cv-0300")]
    Cv0300,
    #[serde(rename = "cv-0301")]
    Cv0301,
    #[serde(rename = "cv-0302")]
    Cv0302,
    #[serde(rename = "cv-0303")]
    Cv0303,
    #[serde(rename = "cv-0304")]
    Cv0304,
    #[serde(rename = "cv-0305")]
    Cv0305,
    #[serde(rename = "cv-0306")]
    Cv0306,
    #[serde(rename = "cv-0307")]
    Cv0307,
    #[serde(rename = "cv-0308")]
    Cv0308,
    #[serde(rename = "cv-0309")]
    Cv0309,
    #[serde(rename = "cv-0310")]
    Cv0310,
    #[serde(rename = "cv-0311")]
    Cv0311,
    #[serde(rename = "cv-0312")]
    Cv0312,
    #[serde(rename = "cv-0313")]
    Cv0313,
    #[serde(rename = "cv-0314")]
    Cv0314,
    #[serde(rename = "cv-0315")]
    Cv0315,
    #[serde(rename = "cv-0316")]
    Cv0316,
    #[serde(rename = "cv-0317")]
    Cv0317,
    #[serde(rename = "cv-0318")]
    Cv0318,
    #[serde(rename = "cv-0319")]
    Cv0319,
    #[serde(rename = "cv-0320")]
    Cv0320,
    #[serde(rename = "cv-0321")]
    Cv0321,
    #[serde(rename = "cv-0322")]
    Cv0322,
    #[serde(rename = "cv-0323")]
    Cv0323,
    #[serde(rename = "cv-0324")]
    Cv0324,
    #[serde(rename = "cv-0325")]
    Cv0325,
    #[serde(rename = "cv-0326")]
    Cv0326,
    #[serde(rename = "cv-0327")]
    Cv0327,
    #[serde(rename = "cv-0328")]
    Cv0328,
    #[serde(rename = "cv-0329")]
    Cv0329,
    #[serde(rename = "cv-0330")]
    Cv0330,
    #[serde(rename = "cv-0331")]
    Cv0331,
    #[serde(rename = "cv-0332")]
    Cv0332,
    #[serde(rename = "cv-0333")]
    Cv0333,
    #[serde(rename = "cv-0334")]
    Cv0334,
    #[serde(rename = "cv-0335")]
    Cv0335,
    #[serde(rename = "cv-0336")]
    Cv0336,
    #[serde(rename = "cv-0337")]
    Cv0337,
    #[serde(rename = "cv-0338")]
    Cv0338,
    #[serde(rename = "cv-0339")]
    Cv0339,
    #[serde(rename = "cv-0340")]
    Cv0340,
    #[serde(rename = "cv-0341")]
    Cv0341,
    #[serde(rename = "cv-0342")]
    Cv0342,
    #[serde(rename = "cv-0343")]
    Cv0343,
    #[serde(rename = "cv-0344")]
    Cv0344,
    #[serde(rename = "cv-0345")]
    Cv0345,
    #[serde(rename = "cv-0346")]
    Cv0346,
    #[serde(rename = "cv-0347")]
    Cv0347,
    #[serde(rename = "cv-0348")]
    Cv0348,
    #[serde(rename = "cv-0349")]
    Cv0349,
    #[serde(rename = "cv-0350")]
    Cv0350,
    #[serde(rename = "cv-0351")]
    Cv0351,
    #[serde(rename = "cv-0352")]
    Cv0352,
    #[serde(rename = "cv-0353")]
    Cv0353,
    #[serde(rename = "cv-0354")]
    Cv0354,
    #[serde(rename = "cv-0355")]
    Cv0355,
    #[serde(rename = "cv-0356")]
    Cv0356,
    #[serde(rename = "cv-0357")]
    Cv0357,
    #[serde(rename = "cv-0358")]
    Cv0358,
    #[serde(rename = "cv-0359")]
    Cv0359,
    #[serde(rename = "cv-0360")]
    Cv0360,
    #[serde(rename = "cv-0361")]
    Cv0361,
    #[serde(rename = "cv-0362")]
    Cv0362,
    #[serde(rename = "cv-0363")]
    Cv0363,
    #[serde(rename = "cv-0364")]
    Cv0364,
    #[serde(rename = "cv-0365")]
    Cv0365,
    #[serde(rename = "cv-0366")]
    Cv0366,
    #[serde(rename = "cv-0367")]
    Cv0367,
    #[serde(rename = "cv-0368")]
    Cv0368,
    #[serde(rename = "cv-0369")]
    Cv0369,
    #[serde(rename = "cv-0370")]
    Cv0370,
    #[serde(rename = "cv-0371")]
    Cv0371,
    #[serde(rename = "cv-0372")]
    Cv0372,
    #[serde(rename = "cv-0373")]
    Cv0373,
    #[serde(rename = "cv-0374")]
    Cv0374,
    #[serde(rename = "cv-0375")]
    Cv0375,
    #[serde(rename = "cv-0376")]
    Cv0376,
    #[serde(rename = "cv-0377")]
    Cv0377,
    #[serde(rename = "cv-0378")]
    Cv0378,
    #[serde(rename = "cv-0379")]
    Cv0379,
    #[serde(rename = "cv-0380")]
    Cv0380,
    #[serde(rename = "cv-0381")]
    Cv0381,
    #[serde(rename = "cv-0382")]
    Cv0382,
    #[serde(rename = "cv-0383")]
    Cv0383,
    #[serde(rename = "cv-0384")]
    Cv0384,
    #[serde(rename = "cv-0385")]
    Cv0385,
    #[serde(rename = "cv-0386")]
    Cv0386,
    #[serde(rename = "cv-0387")]
    Cv0387,
    #[serde(rename = "cv-0388")]
    Cv0388,
    #[serde(rename = "cv-0389")]
    Cv0389,
    #[serde(rename = "cv-0390")]
    Cv0390,
    #[serde(rename = "cv-0391")]
    Cv0391,
    #[serde(rename = "cv-0392")]
    Cv0392,
    #[serde(rename = "cv-0393")]
    Cv0393,
    #[serde(rename = "cv-0394")]
    Cv0394,
    #[serde(rename = "cv-0395")]
    Cv0395,
    #[serde(rename = "cv-0396")]
    Cv0396,
    #[serde(rename = "cv-0397")]
    Cv0397,
    #[serde(rename = "cv-0398")]
    Cv0398,
    #[serde(rename = "cv-0399")]
    Cv0399,
    #[serde(rename = "cv-0400")]
    Cv0400,
    #[serde(rename = "cv-0401")]
    Cv0401,
    #[serde(rename = "cv-0402")]
    Cv0402,
    #[serde(rename = "cv-0403")]
    Cv0403,
    #[serde(rename = "cv-0404")]
    Cv0404,
    #[serde(rename = "cv-0405")]
    Cv0405,
    #[serde(rename = "cv-0406")]
    Cv0406,
    #[serde(rename = "cv-0407")]
    Cv0407,
    #[serde(rename = "cv-0408")]
    Cv0408,
    #[serde(rename = "cv-0409")]
    Cv0409,
    #[serde(rename = "cv-0410")]
    Cv0410,
    #[serde(rename = "cv-0411")]
    Cv0411,
    #[serde(rename = "cv-0412")]
    Cv0412,
    #[serde(rename = "cv-0413")]
    Cv0413,
    #[serde(rename = "cv-0414")]
    Cv0414,
    #[serde(rename = "cv-0415")]
    Cv0415,
    #[serde(rename = "cv-0416")]
    Cv0416,
    #[serde(rename = "cv-0417")]
    Cv0417,
    #[serde(rename = "cv-0418")]
    Cv0418,
    #[serde(rename = "cv-0419")]
    Cv0419,
    #[serde(rename = "cv-0420")]
    Cv0420,
    #[serde(rename = "cv-0421")]
    Cv0421,
    #[serde(rename = "cv-0422")]
    Cv0422,
    #[serde(rename = "cv-0423")]
    Cv0423,
    #[serde(rename = "cv-0424")]
    Cv0424,
    #[serde(rename = "cv-0425")]
    Cv0425,
    #[serde(rename = "cv-0426")]
    Cv0426,
    #[serde(rename = "cv-0427")]
    Cv0427,
    #[serde(rename = "cv-0428")]
    Cv0428,
    #[serde(rename = "cv-0429")]
    Cv0429,
    #[serde(rename = "cv-0430")]
    Cv0430,
    #[serde(rename = "cv-0431")]
    Cv0431,
    #[serde(rename = "cv-0432")]
    Cv0432,
    #[serde(rename = "cv-0433")]
    Cv0433,
    #[serde(rename = "cv-0434")]
    Cv0434,
    #[serde(rename = "cv-0435")]
    Cv0435,
    #[serde(rename = "cv-0436")]
    Cv0436,
    #[serde(rename = "cv-0437")]
    Cv0437,
    #[serde(rename = "cv-0438")]
    Cv0438,
    #[serde(rename = "cv-0439")]
    Cv0439,
    #[serde(rename = "cv-0440")]
    Cv0440,
    #[serde(rename = "cv-0441")]
    Cv0441,
    #[serde(rename = "cv-0442")]
    Cv0442,
    #[serde(rename = "cv-0443")]
    Cv0443,
    #[serde(rename = "cv-0444")]
    Cv0444,
    #[serde(rename = "cv-0445")]
    Cv0445,
    #[serde(rename = "cv-0446")]
    Cv0446,
    #[serde(rename = "cv-0447")]
    Cv0447,
    #[serde(rename = "cv-0448")]
    Cv0448,
    #[serde(rename = "cv-0449")]
    Cv0449,
    #[serde(rename = "cv-0450")]
    Cv0450,
    #[serde(rename = "cv-0451")]
    Cv0451,
    #[serde(rename = "cv-0452")]
    Cv0452,
    #[serde(rename = "cv-0453")]
    Cv0453,
    #[serde(rename = "cv-0454")]
    Cv0454,
    #[serde(rename = "cv-0455")]
    Cv0455,
    #[serde(rename = "cv-0456")]
    Cv0456,
    #[serde(rename = "cv-0457")]
    Cv0457,
    #[serde(rename = "cv-0458")]
    Cv0458,
    #[serde(rename = "cv-0459")]
    Cv0459,
    #[serde(rename = "cv-0460")]
    Cv0460,
    #[serde(rename = "cv-0461")]
    Cv0461,
    #[serde(rename = "cv-0462")]
    Cv0462,
    #[serde(rename = "cv-0463")]
    Cv0463,
    #[serde(rename = "cv-0464")]
    Cv0464,
    #[serde(rename = "cv-0465")]
    Cv0465,
    #[serde(rename = "cv-0466")]
    Cv0466,
    #[serde(rename = "cv-0467")]
    Cv0467,
    #[serde(rename = "cv-0469")]
    Cv0469,
    #[serde(rename = "cv-0470")]
    Cv0470,
    #[serde(rename = "cv-0471")]
    Cv0471,
    #[serde(rename = "cv-0472")]
    Cv0472,
    #[serde(rename = "cv-0473")]
    Cv0473,
    #[serde(rename = "cv-0474")]
    Cv0474,
    #[serde(rename = "cv-0475")]
    Cv0475,
    #[serde(rename = "cv-0476")]
    Cv0476,
    #[serde(rename = "cv-0477")]
    Cv0477,
    #[serde(rename = "cv-0478")]
    Cv0478,
    #[serde(rename = "cv-0479")]
    Cv0479,
    #[serde(rename = "cv-0480")]
    Cv0480,
    #[serde(rename = "cv-0481")]
    Cv0481,
    #[serde(rename = "cv-0482")]
    Cv0482,
    #[serde(rename = "cv-0483")]
    Cv0483,
    #[serde(rename = "cv-0484")]
    Cv0484,
    #[serde(rename = "cv-0485")]
    Cv0485,
    #[serde(rename = "cv-0486")]
    Cv0486,
    #[serde(rename = "cv-0487")]
    Cv0487,
    #[serde(rename = "cv-0488")]
    Cv0488,
    #[serde(rename = "cv-0489")]
    Cv0489,
    #[serde(rename = "xQc")]
    XQc,
}
impl std::fmt::Display for V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Enum21Savage => "21 Savage",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Enum50Cent => "50 Cent",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AApRocky => "A$AP Rocky",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ARRahman => "A.R. Rahman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AamirKhan => "Aamir Khan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AangAvatar => "Aang (Avatar)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AaronRodgers => {
                "Aaron Rodgers"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AbrahamLincoln => {
                "Abraham Lincoln"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AdaWong => "Ada Wong",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AdamDriver => "Adam Driver",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AdamSandler => "Adam Sandler",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AdinRoss => "Adin Ross",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Agent47 => "Agent 47",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Airrack => "Airrack",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AjayDevgn => "Ajay Devgn",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AkshayKumar => "Akshay Kumar",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlGore => "Al Gore",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlPacino => "Al Pacino",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlainDelon => "Alain Delon",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlanAlda => "Alan Alda",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlanCumming => "Alan Cumming",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlanRickman => "Alan Rickman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlbertEinstein => {
                "Albert Einstein"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlbusDumbledore => {
                "Albus Dumbledore"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlexHormozi => "Alex Hormozi",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlexPereira => "Alex Pereira",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlexTrebek => "Alex Trebek",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlexandriaOcasioCortez => {
                "Alexandria Ocasio-Cortez"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlexzJohnson => {
                "Alexz Johnson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AliWong => "Ali Wong",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AliaBhatt => "Alia Bhatt",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AllMight => "All Might",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Aloy => "Aloy",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Altair => "Altair",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AltonBrown => "Alton Brown",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlucardHellsing => {
                "Alucard (Hellsing)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AlvinChipmunks => {
                "Alvin (Chipmunks)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AmandaNunes => "Amanda Nunes",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AmitabhBachchan => {
                "Amitabh Bachchan"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AmyAdams => "Amy Adams",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AmyPoehler => "Amy Poehler",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AmyWinehouse => {
                "Amy Winehouse"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnaGasteyer => "Ana Gasteyer",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnaDeArmas => "Ana de Armas",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AndersonCooper => {
                "Anderson Cooper"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AndrewHuberman => {
                "Andrew Huberman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AndrewRannells => {
                "Andrew Rannells"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AndrewRea => "Andrew Rea",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AndrewSchulz => {
                "Andrew Schulz"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AndyDwyer => "Andy Dwyer",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AngLee => "Ang Lee",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AngelReese => "Angel Reese",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AngelaMartin => {
                "Angela Martin"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AngelaMerkel => {
                "Angela Merkel"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AngelaYee => "Angela Yee",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AngelinaJolie => {
                "Angelina Jolie"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AngerInsideOut => {
                "Anger (Inside Out)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Anitta => "Anitta",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnneHathaway => {
                "Anne Hathaway"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnselElgort => "Ansel Elgort",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AntMan => "Ant-Man",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnthonyAnderson => {
                "Anthony Anderson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnthonyBourdain => {
                "Anthony Bourdain"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnthonyEdwards => {
                "Anthony Edwards"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnthonyFauci => {
                "Anthony Fauci"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnthonyHopkins => {
                "Anthony Hopkins"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnthonyJoshua => {
                "Anthony Joshua"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnthonyMackie => {
                "Anthony Mackie"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AntonChigurh => {
                "Anton Chigurh"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AntonioBanderas => {
                "Antonio Banderas"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnyaForger => "Anya Forger",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AnyaTaylorJoy => {
                "Anya Taylor-Joy"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AprilLudgate => {
                "April Ludgate"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Aragorn => "Aragorn",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ArcherSterling => {
                "Archer (Sterling)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ArdenCho => "Arden Cho",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ArethaFranklin => {
                "Aretha Franklin"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ArianaGrande => {
                "Ariana Grande"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ArielLittleMermaid => {
                "Ariel (Little Mermaid)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ArmieHammer => "Armie Hammer",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ArnoldSchwarzenegger => {
                "Arnold Schwarzenegger"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ArthurMorgan => {
                "Arthur Morgan"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AryaStark => "Arya Stark",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AsaButterfield => {
                "Asa Butterfield"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AshKetchum => "Ash Ketchum",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Asmongold => "Asmongold",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AubreyPlaza => "Aubrey Plaza",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AustinButler => {
                "Austin Butler"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AustinPowers => {
                "Austin Powers"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Awkwafina => "Awkwafina",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AxlRose => "Axl Rose",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AyoEdebiri => "Ayo Edebiri",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::AzizAnsari => "Aziz Ansari",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BJNovak => "B.J. Novak",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BjBlazkowicz => {
                "BJ Blazkowicz"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BtsJungkook => "BTS Jungkook",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BabarAzam => "Babar Azam",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BabyYodaGrogu => {
                "Baby Yoda (Grogu)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BadBunny => "Bad Bunny",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BakugoKatsuki => {
                "Bakugo Katsuki"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BamMargera => "Bam Margera",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BarackObama => "Barack Obama",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BarbaraEden => "Barbara Eden",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BarneyRubble => {
                "Barney Rubble"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BarneyStinson => {
                "Barney Stinson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BarryKeoghan => {
                "Barry Keoghan"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BarryWhite => "Barry White",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BartSimpson => "Bart Simpson",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BatmanAnimated => {
                "Batman (Animated)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BearGrylls => "Bear Grylls",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Beavis => "Beavis",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BelleBeautyAndTheBeast => {
                "Belle (Beauty and the Beast)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BenMcKenzie => "Ben McKenzie",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BenShapiro => "Ben Shapiro",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BenStiller => "Ben Stiller",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BenStokes => "Ben Stokes",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BenTennyson => "Ben Tennyson",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BenWhishaw => "Ben Whishaw",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BenderFuturama => {
                "Bender (Futurama)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BenedictCumberbatch => {
                "Benedict Cumberbatch"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BensonBoone => "Benson Boone",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BernieSanders => {
                "Bernie Sanders"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BertKreischer => {
                "Bert Kreischer"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BethSmith => "Beth Smith",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BettyWhite => "Betty White",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BigBossMgs => {
                "Big Boss (MGS)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BigBoy => "Big Boy",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BigMouthNick => {
                "Big Mouth Nick"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BigSmokeGta => {
                "Big Smoke (GTA)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillBurr => "Bill Burr",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillClinton => "Bill Clinton",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillGates => "Bill Gates",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillMaher => "Bill Maher",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillNighy => "Bill Nighy",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillNye => "Bill Nye",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillPullman => "Bill Pullman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillSimmons => "Bill Simmons",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillieEilish => {
                "Billie Eilish"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillieJoeArmstrong => {
                "Billie Joe Armstrong"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillyButcher => {
                "Billy Butcher"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BillyJoel => "Billy Joel",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BingbingLi => "Bingbing Li",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Bizarrap => "Bizarrap",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BlackPanther => {
                "Black Panther"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BlackWidow => "Black Widow",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BlakeShelton => {
                "Blake Shelton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BlossomPpg => "Blossom (PPG)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BoBurnham => "Bo Burnham",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BoJackHorseman => {
                "BoJack Horseman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BobMarley => "Bob Marley",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BobOdenkirk => "Bob Odenkirk",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BobRoss => "Bob Ross",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BobbyHill => "Bobby Hill",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BobbyLee => "Bobby Lee",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BonnieWright => {
                "Bonnie Wright"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BoobooStewart => {
                "Booboo Stewart"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BorisJohnson => {
                "Boris Johnson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Bowser => "Bowser",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BradPaisley => "Brad Paisley",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BradPitt => "Brad Pitt",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BradleyCooper => {
                "Bradley Cooper"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BradleyStevenPerry => {
                "Bradley Steven Perry"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BranStark => "Bran Stark",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BrayWyatt => "Bray Wyatt",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BrendanGleeson => {
                "Brendan Gleeson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BretmanRock => "Bretman Rock",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BrianCox => "Brian Cox",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BruceSpringsteen => {
                "Bruce Springsteen"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BruceWillis => "Bruce Willis",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BrunoEncanto => {
                "Bruno (Encanto)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BrunoGanz => "Bruno Ganz",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BrunoMars => "Bruno Mars",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BryanCranston => {
                "Bryan Cranston"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BubblesPpg => "Bubbles (PPG)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BugsBunny => "Bugs Bunny",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BurnaBoy => "Burna Boy",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BurtReynolds => {
                "Burt Reynolds"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ButtHead => "Butt-Head",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ButtercupPpg => {
                "Buttercup (PPG)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ButtersStotch => {
                "Butters Stotch"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BuzzAldrin => "Buzz Aldrin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BuzzLightyear => {
                "Buzz Lightyear"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::C3po => "C-3PO",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CjGtaSanAndreas => {
                "CJ (GTA San Andreas)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CmPunk => "CM Punk",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaitlinClark => {
                "Caitlin Clark"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaitrionaBalfe => {
                "Caitriona Balfe"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaityLotz => "Caity Lotz",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CalvinHarris => {
                "Calvin Harris"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CameronBoyce => {
                "Cameron Boyce"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CamilaCabello => {
                "Camila Cabello"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Camilo => "Camilo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CandaceOwens => {
                "Candace Owens"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CandiceAccola => {
                "Candice Accola"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaneloAlvarez => {
                "Canelo Alvarez"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaptainAmericaMcu => {
                "Captain America (MCU)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaptainHookPeterPan => {
                "Captain Hook (Peter Pan)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaptainJackSparrow => {
                "Captain Jack Sparrow"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CardiB => "Cardi B",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CarlSagan => "Carl Sagan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CarltonBanks => {
                "Carlton Banks"
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
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaseyNeistat => {
                "Casey Neistat"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CateBlanchett => {
                "Cate Blanchett"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CaterinaScorsone => {
                "Caterina Scorsone"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CedricTheEntertainer => {
                "Cedric the Entertainer"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CeeDeeLamb => "CeeDee Lamb",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CentralCee => "Central Cee",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CerseiLannister => {
                "Cersei Lannister"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChaceCrawford => {
                "Chace Crawford"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChadwickBoseman => {
                "Chadwick Boseman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChandlerBing => {
                "Chandler Bing"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChappellRoan => {
                "Chappell Roan"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharlamagneThaGod => {
                "Charlamagne Tha God"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharlesBarkley => {
                "Charles Barkley"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharlesDance => {
                "Charles Dance"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharlesLeclerc => {
                "Charles Leclerc"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharliDAmelio => {
                "Charli D'Amelio"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharliXcx => "Charli XCX",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharlieChaplin => {
                "Charlie Chaplin"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharlieDay => "Charlie Day",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharlieKirk => "Charlie Kirk",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CharlizeTheron => {
                "Charlize Theron"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChefSouthPark => {
                "Chef (South Park)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChesterBennington => {
                "Chester Bennington"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChiwetelEjiofor => {
                "Chiwetel Ejiofor"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisBrown => "Chris Brown",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisCornell => {
                "Chris Cornell"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisEvans => "Chris Evans",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisHemsworth => {
                "Chris Hemsworth"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisMartin => "Chris Martin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisPine => "Chris Pine",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisPratt => "Chris Pratt",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisRedfield => {
                "Chris Redfield"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisRock => "Chris Rock",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisStapleton => {
                "Chris Stapleton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisTucker => "Chris Tucker",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChrisWilliamson => {
                "Chris Williamson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChristianBale => {
                "Christian Bale"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChristinaHendricks => {
                "Christina Hendricks"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChristinaRicci => {
                "Christina Ricci"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChristopherMintzPlasse => {
                "Christopher Mintz-Plasse"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChristopherWalken => {
                "Christopher Walken"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Chucky => "Chucky",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ChunLi => "Chun-Li",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CillianMurphy => {
                "Cillian Murphy"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cinderella => "Cinderella",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Claptrap => "Claptrap",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ClintEastwood => {
                "Clint Eastwood"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CloudStrife => "Cloud Strife",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CodyKo => "Cody Ko",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CodyRhodes => "Cody Rhodes",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CogsworthBeautyAndTheBeast => {
                "Cogsworth (Beauty and the Beast)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CoiLeray => "Coi Leray",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ColinCowherd => {
                "Colin Cowherd"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ColleenBallinger => {
                "Colleen Ballinger"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CommanderShepard => {
                "Commander Shepard"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ConanGray => "Conan Gray",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ConanOBrien => {
                "Conan O'Brien"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ConnorMcDavid => {
                "Connor McDavid"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ConorMcGregor => {
                "Conor McGregor"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ConstanceWu => "Constance Wu",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CoreyTaylor => "Corey Taylor",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CorpseHusband => {
                "Corpse Husband"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cortana => "Cortana",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CouRage => "CouRage",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CourageTheCowardlyDog => {
                "Courage the Cowardly Dog"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CreedBratton => {
                "Creed Bratton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::CristianoRonaldo => {
                "Cristiano Ronaldo"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DaenerysTargaryen => {
                "Daenerys Targaryen"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DaffyDuck => "Daffy Duck",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DaisyRidley => "Daisy Ridley",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DakotaFanning => {
                "Dakota Fanning"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DaleGribble => "Dale Gribble",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DamianLewis => "Damian Lewis",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DamianLillard => {
                "Damian Lillard"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanAykroyd => "Dan Aykroyd",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanFogler => "Dan Fogler",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanLeBatard => {
                "Dan Le Batard"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanStevens => "Dan Stevens",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanaWhite => "Dana White",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanielCraig => "Daniel Craig",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanielDaeKim => {
                "Daniel Dae Kim"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanielRadcliffe => {
                "Daniel Radcliffe"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanielRicciardo => {
                "Daniel Ricciardo"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DaniellePanabaker => {
                "Danielle Panabaker"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DannyDeVito => "Danny DeVito",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DannyDuncan => "Danny Duncan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DannyGonzalez => {
                "Danny Gonzalez"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DannyPhantom => {
                "Danny Phantom"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DanteDmc => "Dante (DMC)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DarthVader => "Darth Vader",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DaveRapper => "Dave (rapper)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DaveBautista => {
                "Dave Bautista"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DaveChappelle => {
                "Dave Chappelle"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DaveGrohl => "Dave Grohl",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DaveRubin => "Dave Rubin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DavidAttenborough => {
                "David Attenborough"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DavidBeckham => {
                "David Beckham"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DavidBowie => "David Bowie",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DavidDobrik => "David Dobrik",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DavidGoggins => {
                "David Goggins"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DavidGuetta => "David Guetta",
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
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Deadpool => "Deadpool",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DeepikaPadukone => {
                "Deepika Padukone"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DekuMyHeroAcademia => {
                "Deku (My Hero Academia)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DemiLovato => "Demi Lovato",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DenisLeary => "Denis Leary",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DenjiChainsawMan => {
                "Denji (Chainsaw Man)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DenzelWashington => {
                "Denzel Washington"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DerekMears => "Derek Mears",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DesmondTutu => "Desmond Tutu",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Destiny => "Destiny",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DevPatel => "Dev Patel",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DevinBooker => "Devin Booker",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DexterDexterSLab => {
                "Dexter (Dexter's Lab)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DexterMorgan => {
                "Dexter Morgan"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DianeKeaton => "Diane Keaton",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DianeKruger => "Diane Kruger",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DianeNguyen => "Diane Nguyen",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DickVanDyke => {
                "Dick Van Dyke"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DiegoLuna => "Diego Luna",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Diplo => "Diplo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DiscordMyLittlePony => {
                "Discord (My Little Pony)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DisguisedToast => {
                "Disguised Toast"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DoctorStrange => {
                "Doctor Strange"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DojaCat => "Doja Cat",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DollyParton => "Dolly Parton",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DomhnallGleeson => {
                "Domhnall Gleeson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DominicCooper => {
                "Dominic Cooper"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonDraper => "Don Draper",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonToliver => "Don Toliver",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonaldDuck => "Donald Duck",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonaldGlover => {
                "Donald Glover"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonaldSutherland => {
                "Donald Sutherland"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonaldTrump => "Donald Trump",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonkeyShrek => {
                "Donkey (Shrek)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonnieYen => "Donnie Yen",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Doofenshmirtz => {
                "Doofenshmirtz"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DoomSlayer => "Doom Slayer",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DouglasMurray => {
                "Douglas Murray"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DoutzenKroes => {
                "Doutzen Kroes"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DrDre => "Dr. Dre",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DrHouse => "Dr. House",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DrPhil => "Dr. Phil",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DrLupo => "DrLupo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Drake => "Drake",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DrakeBell => "Drake Bell",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DreamMinecraft => {
                "Dream (Minecraft)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DrewCarey => "Drew Carey",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DrewGooden => "Drew Gooden",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DuaLipa => "Dua Lipa",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DudePerfectTyler => {
                "Dude Perfect Tyler"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DukeNukem => "Duke Nukem",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DwayneTheRockJohnson => {
                "Dwayne 'The Rock' Johnson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DwightSchrute => {
                "Dwight Schrute"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EarthaKitt => "Eartha Kitt",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EdSheeran => "Ed Sheeran",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EddieIzzard => "Eddie Izzard",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EddieMurphy => "Eddie Murphy",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EdwardAsner => "Edward Asner",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EdwardElric => "Edward Elric",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElaineBenes => "Elaine Benes",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EliRoth => "Eli Roth",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElisabethMoss => {
                "Elisabeth Moss"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElleFanning => "Elle Fanning",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EllenBurstyn => {
                "Ellen Burstyn"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EllenDeGeneres => {
                "Ellen DeGeneres"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EllenRipley => "Ellen Ripley",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EllieBorderlands => {
                "Ellie (Borderlands)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EllieWilliams => {
                "Ellie Williams"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Elmo => "Elmo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElonMusk => "Elon Musk",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElsaFrozen => "Elsa (Frozen)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EltonJohn => "Elton John",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElvisPresley => {
                "Elvis Presley"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EmileHirsch => "Emile Hirsch",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Eminem => "Eminem",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EmmaChamberlain => {
                "Emma Chamberlain"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EmmaStone => "Emma Stone",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EmmaWatson => "Emma Watson",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EmmanuelMacron => {
                "Emmanuel Macron"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EmperorPalpatine => {
                "Emperor Palpatine"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EnriqueIglesias => {
                "Enrique Iglesias"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ErenJaeger => "Eren Jaeger",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EricCartman => "Eric Cartman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EricClapton => "Eric Clapton",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ErlingHaaland => {
                "Erling Haaland"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ErnieHudson => "Ernie Hudson",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ErzaScarlet => "Erza Scarlet",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Esfand => "Esfand",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EthanKlein => "Ethan Klein",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EugeneLeeYang => {
                "Eugene Lee Yang"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EzioAuditore => {
                "Ezio Auditore"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::EzraMiller => "Ezra Miller",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FaZeRug => "FaZe Rug",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FamilyGuyQuagmire => {
                "Family Guy Quagmire"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FazeClanBanks => {
                "Faze Clan Banks"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Feid => "Feid",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FelicityJones => {
                "Felicity Jones"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Fergie => "Fergie",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FernandoAlonso => {
                "Fernando Alonso"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FerrisBueller => {
                "Ferris Bueller"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FinnWolfhard => {
                "Finn Wolfhard"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FinnTheHuman => {
                "Finn the Human"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FionaShrek => "Fiona (Shrek)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FionaShaw => "Fiona Shaw",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FleetwoodMacStevieNicks => {
                "Fleetwood Mac Stevie Nicks"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FloMilli => "Flo Milli",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FlorenceHenderson => {
                "Florence Henderson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FlorencePugh => {
                "Florence Pugh"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FloydMayweather => {
                "Floyd Mayweather"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ForestWhitaker => {
                "Forest Whitaker"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ForrestGump => "Forrest Gump",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Forsen => "Forsen",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FranciaRaisa => {
                "Francia Raisa"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FrankSinatra => {
                "Frank Sinatra"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FrankUnderwood => {
                "Frank Underwood"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FranklinClinton => {
                "Franklin Clinton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FranklinDRoosevelt => {
                "Franklin D. Roosevelt"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FredFlintstone => {
                "Fred Flintstone"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FredAgain => "Fred again",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FreddieHighmore => {
                "Freddie Highmore"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FreddieMercury => {
                "Freddie Mercury"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FreddiePrinze => {
                "Freddie Prinze"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FreddyKrueger => {
                "Freddy Krueger"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FreidaPinto => "Freida Pinto",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FrodoBaggins => {
                "Frodo Baggins"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FryFuturama => {
                "Fry (Futurama)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::FuerzaRegida => {
                "Fuerza Regida"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Fuslie => "Fuslie",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Future => "Future",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GirInvaderZim => {
                "GIR (Invader Zim)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GLaDos => "GLaDOS",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GabrielIglesias => {
                "Gabriel Iglesias"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GalGadot => "Gal Gadot",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Gandalf => "Gandalf",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GarnetStevenUniverse => {
                "Garnet (Steven Universe)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GarrusVakarian => {
                "Garrus Vakarian"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GaryOldman => "Gary Oldman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GaryVaynerchuk => {
                "Gary Vaynerchuk"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GeenaDavis => "Geena Davis",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GemmaArterton => {
                "Gemma Arterton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GenjiOverwatch => {
                "Genji (Overwatch)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GeorgeCostanza => {
                "George Costanza"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GeorgeHWBush => {
                "George H W Bush"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GeorgeJetson => {
                "George Jetson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GeorgeRRMartin => {
                "George R.R. Martin"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GeorgeWBush => {
                "George W Bush"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GeorgeNotFound => {
                "GeorgeNotFound"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GeraltOfRivia => {
                "Geralt of Rivia"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GeriHalliwell => {
                "Geri Halliwell"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GhostfaceScream => {
                "Ghostface (Scream)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GiannisAntetokounmpo => {
                "Giannis Antetokounmpo"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GilbertGottfried => {
                "Gilbert Gottfried"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GillianJacobs => {
                "Gillian Jacobs"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GinaCarano => "Gina Carano",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GinniferGoodwin => {
                "Ginnifer Goodwin"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GlenPowell => "Glen Powell",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GlennClose => "Glenn Close",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GloRilla => "GloRilla",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GloriaSteinem => {
                "Gloria Steinem"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GojoSatoru => "Gojo Satoru",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Goku => "Goku",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GollumSmeagol => {
                "Gollum Smeagol"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GonFreecss => "Gon Freecss",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GordonLightfoot => {
                "Gordon Lightfoot"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GordonRamsay => {
                "Gordon Ramsay"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GracieAbrams => {
                "Gracie Abrams"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GrahamNorton => {
                "Graham Norton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GrantCardone => {
                "Grant Cardone"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GretaGerwig => "Greta Gerwig",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GretaThunberg => {
                "Greta Thunberg"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GriffithBerserk => {
                "Griffith (Berserk)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Grimes => "Grimes",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Groot => "Groot",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GruDespicableMe => {
                "Gru (Despicable Me)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GrupoFrontera => {
                "Grupo Frontera"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GuguMbathaRaw => {
                "Gugu Mbatha-Raw"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GumballWatterson => {
                "Gumball Watterson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Gunna => "Gunna",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GusFring => "Gus Fring",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GutsBerserk => {
                "Guts (Berserk)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GuyFieri => "Guy Fieri",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GuyPearce => "Guy Pearce",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::GwendolineChristie => {
                "Gwendoline Christie"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::H3h3 => "H3H3",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Hal9000 => "HAL 9000",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Hagrid => "Hagrid",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HaileeSteinfeld => {
                "Hailee Steinfeld"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Halsey => "Halsey",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HanSolo => "Han Solo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HandsomeJack => {
                "Handsome Jack"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HankSchrader => {
                "Hank Schrader"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HannibalLecter => {
                "Hannibal Lecter"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HansFrozen => "Hans (Frozen)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HansZimmer => "Hans Zimmer",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HarleyQuinn => "Harley Quinn",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HarrisonFord => {
                "Harrison Ford"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HarryConnickJr => {
                "Harry Connick Jr."
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HarryPotter => "Harry Potter",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HarryStyles => "Harry Styles",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HarveyKeitel => {
                "Harvey Keitel"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HasanMinhaj => "Hasan Minhaj",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HasanPiker => "Hasan Piker",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HelenaBonhamCarter => {
                "Helena Bonham Carter"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HenryCavill => "Henry Cavill",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HenryGolding => {
                "Henry Golding"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HermioneGranger => {
                "Hermione Granger"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Herobrine => "Herobrine",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HikaruNakamura => {
                "Hikaru Nakamura"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HilaKlein => "Hila Klein",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HilarySwank => "Hilary Swank",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HillaryClinton => {
                "Hillary Clinton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HinataHyuga => "Hinata Hyuga",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Homelander => "Homelander",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HomerSimpson => {
                "Homer Simpson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HowardStern => "Howard Stern",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HowieMandel => "Howie Mandel",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Hozier => "Hozier",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HrithikRoshan => {
                "Hrithik Roshan"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HughBonneville => {
                "Hugh Bonneville"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HughJackman => "Hugh Jackman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HughLaurie => "Hugh Laurie",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Hulk => "Hulk",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::HulkHogan => "Hulk Hogan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IShowSpeed => "IShowSpeed",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IagoAladdinParrot => {
                "Iago (Aladdin parrot)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IanMcKellen => "Ian McKellen",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IbaiLlanos => "Ibai Llanos",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IceCube => "Ice Cube",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IceSpice => "Ice Spice",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IchigoKurosaki => {
                "Ichigo Kurosaki"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IdinaMenzel => "Idina Menzel",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IdrisElba => "Idris Elba",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IkeFireEmblem => {
                "Ike (Fire Emblem)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IkeBroflovski => {
                "Ike Broflovski"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IlizaShlesinger => {
                "Iliza Shlesinger"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ImeldaStaunton => {
                "Imelda Staunton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IndianaJones => {
                "Indiana Jones"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IngridBergman => {
                "Ingrid Bergman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::InspectorClouseau => {
                "Inspector Clouseau"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Inuyasha => "Inuyasha",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::InvaderZim => "Invader Zim",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IrrfanKhan => "Irrfan Khan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IsaacClarke => "Isaac Clarke",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IslaFisher => "Isla Fisher",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IsraelAdesanya => {
                "Israel Adesanya"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ItachiUchiha => {
                "Itachi Uchiha"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ItadoriYuji => "Itadori Yuji",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::IwanRheon => "Iwan Rheon",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JBalvin => "J Balvin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JKSimmons => "J. K. Simmons",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JJAbrams => "J.J. Abrams",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JKRowling => "J.K. Rowling",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JdVance => "JD Vance",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JaMorant => "Ja Morant",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JackBlack => "Jack Black",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JackHarlow => "Jack Harlow",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JackLemmon => "Jack Lemmon",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JackNicholson => {
                "Jack Nicholson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JackSkellington => {
                "Jack Skellington"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Jacksepticeye => {
                "Jacksepticeye"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JaclynHill => "Jaclyn Hill",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JacobElordi => "Jacob Elordi",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JafarAladdin => {
                "Jafar (Aladdin)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JaimeLannister => {
                "Jaime Lannister"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JakePaul => "Jake Paul",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JakeTheDog => "Jake the Dog",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JamesBondCraigEra => {
                "James Bond (Craig era)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JamesCharles => {
                "James Charles"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JamesCorden => "James Corden",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JamesEarlJones => {
                "James Earl Jones"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JamesHarden => "James Harden",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JamesHetfield => {
                "James Hetfield"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JamesMay => "James May",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JamieFoxx => "Jamie Foxx",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JamieLeeCurtis => {
                "Jamie Lee Curtis"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JamieOliver => "Jamie Oliver",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JaneGoodall => "Jane Goodall",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JanetJackson => {
                "Janet Jackson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JanisJoplin => "Janis Joplin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JaredLeto => "Jared Leto",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JasonBateman => {
                "Jason Bateman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JasonMomoa => "Jason Momoa",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JasonSegel => "Jason Segel",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JasonStatham => {
                "Jason Statham"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JasonVoorhees => {
                "Jason Voorhees"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JayZ => "Jay-Z",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JaysonTatum => "Jayson Tatum",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JeffBezos => "Jeff Bezos",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JeffGoldblum => {
                "Jeff Goldblum"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JeffreeStar => "Jeffree Star",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JennaMarbles => {
                "Jenna Marbles"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JennaOrtega => "Jenna Ortega",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JennieBlackpink => {
                "Jennie (Blackpink)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JenniferAniston => {
                "Jennifer Aniston"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JenniferCarpenter => {
                "Jennifer Carpenter"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JenniferCoolidge => {
                "Jennifer Coolidge"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JenniferHale => {
                "Jennifer Hale"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JenniferLawrence => {
                "Jennifer Lawrence"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JensenHuang => "Jensen Huang",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JeremyAllenWhite => {
                "Jeremy Allen White"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JeremyClarkson => {
                "Jeremy Clarkson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JeremyRenner => {
                "Jeremy Renner"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Jerma985 => "Jerma985",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JerrySeinfeld => {
                "Jerry Seinfeld"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JerrySmith => "Jerry Smith",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JessePinkman => {
                "Jesse Pinkman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JillValentine => {
                "Jill Valentine"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JimCarrey => "Jim Carrey",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JimHalpert => "Jim Halpert",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JimiHendrix => "Jimi Hendrix",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Jimin => "Jimin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JimmyCarter => "Jimmy Carter",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JimmyFallon => "Jimmy Fallon",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JimmyKimmel => "Jimmy Kimmel",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JimmyValmer => "Jimmy Valmer",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JinxArcane => "Jinx (Arcane)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoaquinPhoenix => {
                "Joaquin Phoenix"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JockoWillink => {
                "Jocko Willink"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeBiden => "Joe Biden",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeBudden => "Joe Budden",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeBurrow => "Joe Burrow",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeList => "Joe List",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoePesci => "Joe Pesci",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeRogan => "Joe Rogan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeRoot => "Joe Root",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeScott => "Joe Scott",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoelEmbiid => "Joel Embiid",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoelMiller => "Joel Miller",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeyDiaz => "Joey Diaz",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeyTribbiani => {
                "Joey Tribbiani"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnBoyega => "John Boyega",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnCena => "John Cena",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnFKennedy => {
                "John F. Kennedy"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnLegend => "John Legend",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnLennon => "John Lennon",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnMulaney => "John Mulaney",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnOliver => "John Oliver",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnWick => "John Wick",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnnyBravo => "Johnny Bravo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnnyCash => "Johnny Cash",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnnyDepp => "Johnny Depp",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnnyGalecki => {
                "Johnny Galecki"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnnyKnoxville => {
                "Johnny Knoxville"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JohnnySilverhand => {
                "Johnny Silverhand"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JonBonJovi => "Jon Bon Jovi",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JonFavreau => "Jon Favreau",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JonJones => "Jon Jones",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JonSnow => "Jon Snow",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JonahHill => "Jonah Hill",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JonathanMajors => {
                "Jonathan Majors"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JordanPeterson => {
                "Jordan Peterson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JorgeMasvidal => {
                "Jorge Masvidal"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JosephGordonLevitt => {
                "Joseph Gordon-Levitt"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoshAllen => "Josh Allen",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoshBrolin => "Josh Brolin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoshGad => "Josh Gad",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoshGroban => "Josh Groban",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoyInsideOut => {
                "Joy (Inside Out)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JudeBellingham => {
                "Jude Bellingham"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JuliaLouisDreyfus => {
                "Julia Louis-Dreyfus"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JuliaRoberts => {
                "Julia Roberts"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JustinBieber => {
                "Justin Bieber"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JustinJefferson => {
                "Justin Jefferson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JustinTimberlake => {
                "Justin Timberlake"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JustinTrudeau => {
                "Justin Trudeau"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Ksi => "KSI",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KagomeHigurashi => {
                "Kagome Higurashi"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KaiCenat => "Kai Cenat",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KakashiHatake => {
                "Kakashi Hatake"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KamalaHarris => {
                "Kamala Harris"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KanyeWest => "Kanye West",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KarlJacobs => "Karl Jacobs",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KarolG => "Karol G",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KatyPerry => "Katy Perry",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KeHuyQuan => "Ke Huy Quan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KeanuReeves => "Keanu Reeves",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KeirStarmer => "Keir Starmer",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KeithDavid => "Keith David",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KendrickLamar => {
                "Kendrick Lamar"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KermitTheFrog => {
                "Kermit the Frog"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Kesha => "Kesha",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KevinBacon => "Kevin Bacon",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KevinDurant => "Kevin Durant",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KevinHart => "Kevin Hart",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KevinMalone => "Kevin Malone",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KevinOwens => "Kevin Owens",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KhabibNurmagomedov => {
                "Khabib Nurmagomedov"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KhabyLame => "Khaby Lame",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KhloeKardashian => {
                "Khloe Kardashian"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KilluaZoldyck => {
                "Killua Zoldyck"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KimKardashian => {
                "Kim Kardashian"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KingCharlesIii => {
                "King Charles III"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KingLeonidas => {
                "King Leonidas"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KingOfTheHillHankHill => {
                "King of the Hill Hank Hill"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Kirby => "Kirby",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KobeBryant => "Kobe Bryant",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KramerSeinfeld => {
                "Kramer (Seinfeld)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Kratos => "Kratos",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KrisJenner => "Kris Jenner",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KristenBell => "Kristen Bell",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KristenStewart => {
                "Kristen Stewart"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KristenWiig => "Kristen Wiig",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KristoffFrozen => {
                "Kristoff (Frozen)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KronkEmperorSNewGroove => {
                "Kronk (Emperor's New Groove)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KrustyTheClown => {
                "Krusty the Clown"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KurtCobain => "Kurt Cobain",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KurtisConner => {
                "Kurtis Conner"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KyleBroflovski => {
                "Kyle Broflovski"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KylianMbappe => {
                "Kylian Mbappe"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KylieJenner => "Kylie Jenner",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KyloRen => "Kylo Ren",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KyrieIrving => "Kyrie Irving",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LDeathNote => {
                "L (Death Note)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LadyGaga => "Lady Gaga",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LanaDelRey => "Lana Del Rey",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LanaKane => "Lana Kane",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LaraCroft => "Lara Croft",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Latto => "Latto",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LeBronJames => "LeBron James",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LeagueOfLegendsAhri => {
                "League of Legends Ahri"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LeeMinHo => "Lee Min Ho",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LeelaFuturama => {
                "Leela (Futurama)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Legolas => "Legolas",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LeonKennedy => "Leon Kennedy",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LeonardoDiCaprio => {
                "Leonardo DiCaprio"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LeslieKnope => "Leslie Knope",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LesterHolt => "Lester Holt",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LeviAckerman => {
                "Levi Ackerman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LewisHamilton => {
                "Lewis Hamilton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LexFridman => "Lex Fridman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LiamNeeson => "Liam Neeson",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LightYagami => "Light Yagami",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LightningMcQueen => {
                "Lightning McQueen"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LilBaby => "Lil Baby",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LilWayne => "Lil Wayne",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LilithBorderlands => {
                "Lilith (Borderlands)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LillySingh => "Lilly Singh",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LinkZelda => "Link (Zelda)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LinusLtt => "Linus (LTT)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LionelMessi => "Lionel Messi",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LisaBlackpink => {
                "Lisa (Blackpink)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LisaSimpson => "Lisa Simpson",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LizaKoshy => "Liza Koshy",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Lizzo => "Lizzo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LoganPaul => "Logan Paul",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LoidForger => "Loid Forger",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LoisGriffin => "Lois Griffin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LokiMcu => "Loki (MCU)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Lorde => "Lorde",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LucilleBall => "Lucille Ball",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LucyHeartfilia => {
                "Lucy Heartfilia"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Ludwig => "Ludwig",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LuffyOnePiece => {
                "Luffy (One Piece)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Luigi => "Luigi",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LukeCombs => "Luke Combs",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LukeSkywalker => {
                "Luke Skywalker"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LumiereBeautyAndTheBeast => {
                "Lumiere (Beauty and the Beast)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::LupitaNyongO => {
                "Lupita Nyong'o"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MsDhoni => "MS Dhoni",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MagnusCarlsen => {
                "Magnus Carlsen"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MahatmaGandhi => {
                "Mahatma Gandhi"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MahershalaAli => {
                "Mahershala Ali"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MalalaYousafzai => {
                "Malala Yousafzai"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MalcolmGladwell => {
                "Malcolm Gladwell"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MalcolmX => "Malcolm X",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MaloryArcher => {
                "Malory Archer"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Maluma => "Maluma",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ManuelTurizo => {
                "Manuel Turizo"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarcAndreessen => {
                "Marc Andreessen"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarcMarquez => "Marc Marquez",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarcusFenix => "Marcus Fenix",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MargaretThatcher => {
                "Margaret Thatcher"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MargeSimpson => {
                "Marge Simpson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MargotRobbie => {
                "Margot Robbie"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarilynMonroe => {
                "Marilyn Monroe"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Mario => "Mario",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarjorieTaylorGreene => {
                "Marjorie Taylor Greene"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarkHamill => "Mark Hamill",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarkRober => "Mark Rober",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarkRuffalo => "Mark Ruffalo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarkWahlberg => {
                "Mark Wahlberg"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarkZuckerberg => {
                "Mark Zuckerberg"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Markiplier => "Markiplier",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarquesBrownlee => {
                "Marques Brownlee"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Marshmello => "Marshmello",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarthFireEmblem => {
                "Marth (Fire Emblem)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarthaStewart => {
                "Martha Stewart"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MartinLutherKingJr => {
                "Martin Luther King Jr."
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MasterChief => "Master Chief",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MaterCars => "Mater (Cars)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MattRife => "Matt Rife",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MattSmith => "Matt Smith",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MattWalsh => "Matt Walsh",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MatthewMcConaughey => {
                "Matthew McConaughey"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MauryBigMouth => {
                "Maury (Big Mouth)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MaxVerstappen => {
                "Max Verstappen"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MaximusGladiator => {
                "Maximus (Gladiator)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MeganTheeStallion => {
                "Megan Thee Stallion"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Megatron => "Megatron",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MelRobbins => "Mel Robbins",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Meliodas => "Meliodas",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Melisandre => "Melisandre",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MelissaMcCarthy => {
                "Melissa McCarthy"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MercyOverwatch => {
                "Mercy (Overwatch)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MerlinSwordInTheStone => {
                "Merlin (Sword in the Stone)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MerylStreep => "Meryl Streep",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MetroBoomin => "Metro Boomin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MicahParsons => {
                "Micah Parsons"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MichaelBJordan => {
                "Michael B. Jordan"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MichaelCorleone => {
                "Michael Corleone"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MichaelDeSanta => {
                "Michael De Santa"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MichaelJackson => {
                "Michael Jackson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MichaelKnowles => {
                "Michael Knowles"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MichaelMyers => {
                "Michael Myers"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MichaelPhelps => {
                "Michael Phelps"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MichaelScott => {
                "Michael Scott"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MichelleObama => {
                "Michelle Obama"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MickJagger => "Mick Jagger",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MickeyMouse => "Mickey Mouse",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MikeEhrmantraut => {
                "Mike Ehrmantraut"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MikePence => "Mike Pence",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MikeRowe => "Mike Rowe",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MikeTyson => "Mike Tyson",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MileyCyrus => "Miley Cyrus",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MillieBobbyBrown => {
                "Millie Bobby Brown"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MindyKaling => "Mindy Kaling",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MirabelEncanto => {
                "Mirabel (Encanto)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MitchMcConnell => {
                "Mitch McConnell"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Mizkif => "Mizkif",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MobMobPsycho100 => {
                "Mob (Mob Psycho 100)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Moistcr1tikal => {
                "Moistcr1tikal"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MojoJojo => "Mojo Jojo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MonicaGeller => {
                "Monica Geller"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MordecaiRegularShow => {
                "Mordecai (Regular Show)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MorganFreeman => {
                "Morgan Freeman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MorganWallen => {
                "Morgan Wallen"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MortySmith => "Morty Smith",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MrBurns => "Mr. Burns",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MrGarrison => "Mr. Garrison",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MrKrabs => "Mr. Krabs",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MrBallen => "MrBallen",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MrBeast => "MrBeast",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MuhammadAli => "Muhammad Ali",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Mulan => "Mulan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MushuMulan => "Mushu (Mulan)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MylesGarrett => {
                "Myles Garrett"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NamiOnePiece => {
                "Nami (One Piece)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NancyPelosi => "Nancy Pelosi",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NarendraModi => {
                "Narendra Modi"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NarutoUzumaki => {
                "Naruto Uzumaki"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NataliePortman => {
                "Natalie Portman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NateDiaz => "Nate Diaz",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NathanDrake => "Nathan Drake",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NatsuDragneel => {
                "Natsu Dragneel"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NavalRavikant => {
                "Naval Ravikant"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NedFlanders => "Ned Flanders",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NedFulmer => "Ned Fulmer",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NedStark => "Ned Stark",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NeganWalkingDead => {
                "Negan (Walking Dead)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NeilArmstrong => {
                "Neil Armstrong"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NeilGaiman => "Neil Gaiman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NeilDeGrasseTyson => {
                "Neil deGrasse Tyson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NelsonMandela => {
                "Nelson Mandela"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Neymar => "Neymar",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NezukoKamado => {
                "Nezuko Kamado"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NiallHoran => "Niall Horan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NickFury => "Nick Fury",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NickiMinaj => "Nicki Minaj",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NicolasCage => "Nicolas Cage",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NicoleKidman => {
                "Nicole Kidman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NikkiGlaser => "Nikki Glaser",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NikkieTutorials => {
                "NikkieTutorials"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NikoBellic => "Niko Bellic",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NikolaJokic => "Nikola Jokic",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Ninja => "Ninja",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NoahKahan => "Noah Kahan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NoahSchnapp => "Noah Schnapp",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NolanNorth => "Nolan North",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NormanBates => "Norman Bates",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Northernlion => {
                "Northernlion"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NotoriousBIG => {
                "Notorious B.I.G."
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::NovakDjokovic => {
                "Novak Djokovic"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ObiWanKenobi => {
                "Obi-Wan Kenobi"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::OctaviaSpencer => {
                "Octavia Spencer"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::OliviaRodrigo => {
                "Olivia Rodrigo"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::OprahWinfrey => {
                "Oprah Winfrey"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::OptimusPrime => {
                "Optimus Prime"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::OscarIsaac => "Oscar Isaac",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::OwenWilson => "Owen Wilson",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Ozuna => "Ozuna",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::OzzyOsbourne => {
                "Ozzy Osbourne"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PcPrincipal => "PC Principal",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ParisHilton => "Paris Hilton",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ParkSeoJoon => {
                "Park Seo Joon"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PatMcAfee => "Pat McAfee",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PatrickBateman => {
                "Patrick Bateman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PatrickMahomes => {
                "Patrick Mahomes"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PatrickStar => "Patrick Star",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PatrickStewart => {
                "Patrick Stewart"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PatrickWarburton => {
                "Patrick Warburton"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PaulGiamatti => {
                "Paul Giamatti"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PaulMcCartney => {
                "Paul McCartney"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PaulMescal => "Paul Mescal",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PedroPascal => "Pedro Pascal",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PenelopeCruz => {
                "Penelope Cruz"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Pennywise => "Pennywise",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PesoPluma => "Peso Pluma",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Pestily => "Pestily",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PeterAttia => "Peter Attia",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PeterGriffin => {
                "Peter Griffin"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PeterThiel => "Peter Thiel",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PetyrBaelish => {
                "Petyr Baelish"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PewDiePie => "PewDiePie",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PeytonManning => {
                "Peyton Manning"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PhilipDeFranco => {
                "Philip DeFranco"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PhineasFlynn => {
                "Phineas Flynn"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PhoebeBuffay => {
                "Phoebe Buffay"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Picard => "Picard",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PiersMorgan => "Piers Morgan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Pikachu => "Pikachu",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PitKidIcarus => {
                "Pit (Kid Icarus)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Plankton => "Plankton",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PlayboiCarti => {
                "Playboi Carti"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PointCrow => "PointCrow",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Pokimane => "Pokimane",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Popeye => "Popeye",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PorkyPig => "Porky Pig",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PostMalone => "Post Malone",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PowerChainsawMan => {
                "Power (Chainsaw Man)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Prince => "Prince",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PrinceHarry => "Prince Harry",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PrinceWilliam => {
                "Prince William"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PrincessCarolyn => {
                "Princess Carolyn"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PrincessDiana => {
                "Princess Diana"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PrincessLeia => {
                "Princess Leia"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::PriyankaChopra => {
                "Priyanka Chopra"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ProfessorFarnsworth => {
                "Professor Farnsworth"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Quackity => "Quackity",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::QueenElizabethIi => {
                "Queen Elizabeth II"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::QuentinTarantino => {
                "Quentin Tarantino"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Quin69 => "Quin69",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RmBts => "RM (BTS)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RachelGreen => "Rachel Green",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RachelMaddow => {
                "Rachel Maddow"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RachelZegler => {
                "Rachel Zegler"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RafaelNadal => "Rafael Nadal",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RafikiLionKing => {
                "Rafiki (Lion King)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RaidenMgs => "Raiden (MGS)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RainbowDash => "Rainbow Dash",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Rambo => "Rambo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Ranboo => "Ranboo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RandallPark => "Randall Park",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RandyMarsh => "Randy Marsh",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RandyOrton => "Randy Orton",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RanveerSingh => {
                "Ranveer Singh"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RapunzelTangled => {
                "Rapunzel (Tangled)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RauwAlejandro => {
                "Rauw Alejandro"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RayCharles => "Ray Charles",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ReaperOverwatch => {
                "Reaper (Overwatch)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ReeseWitherspoon => {
                "Reese Witherspoon"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ReginaKing => "Regina King",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ReigenArataka => {
                "Reigen Arataka"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ReinhardtOverwatch => {
                "Reinhardt (Overwatch)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RemyRatatouille => {
                "Remy (Ratatouille)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ReyMysterio => "Rey Mysterio",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RhettAndLink => {
                "Rhett and Link"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RicFlair => "Ric Flair",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RichEisen => "Rich Eisen",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RichardFeynman => {
                "Richard Feynman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RichardHammond => {
                "Richard Hammond"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RickGrimes => "Rick Grimes",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RickSanchez => "Rick Sanchez",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RickyGervais => {
                "Ricky Gervais"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RickyMartin => "Ricky Martin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RigbyRegularShow => {
                "Rigby (Regular Show)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Rihanna => "Rihanna",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RikuKingdomHearts => {
                "Riku (Kingdom Hearts)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RishiSunak => "Rishi Sunak",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RizAhmed => "Riz Ahmed",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RobertDeNiro => {
                "Robert De Niro"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RobertDowneyJr => {
                "Robert Downey Jr."
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RobertPlant => "Robert Plant",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RobinWilliams => {
                "Robin Williams"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RocketRaccoon => {
                "Rocket Raccoon"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RockyBalboa => "Rocky Balboa",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RogerFederer => {
                "Roger Federer"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RohitSharma => "Rohit Sharma",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RomanReigns => "Roman Reigns",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RonDeSantis => "Ron DeSantis",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RonSwanson => "Ron Swanson",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RonWeasley => "Ron Weasley",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RonaldReagan => {
                "Ronald Reagan"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RondaRousey => "Ronda Rousey",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RooneyMara => "Rooney Mara",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Rosalia => "Rosalia",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RoseBlackpink => {
                "Rose (Blackpink)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RossGeller => "Ross Geller",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Rubius => "Rubius",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RupertGrint => "Rupert Grint",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RussellBrand => {
                "Russell Brand"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RussellWestbrook => {
                "Russell Westbrook"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RyanGosling => "Ryan Gosling",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RyanReynolds => {
                "Ryan Reynolds"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RyanSeacrest => {
                "Ryan Seacrest"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RyuStreetFighter => {
                "Ryu (Street Fighter)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::RyuHayabusa => "Ryu Hayabusa",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SsSniperWolf => {
                "SSSniperWolf"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Sza => "SZA",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SabrinaCarpenter => {
                "Sabrina Carpenter"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SachinTendulkar => {
                "Sachin Tendulkar"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SadnessInsideOut => {
                "Sadness (Inside Out)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SafiyaNygaard => {
                "Safiya Nygaard"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SaitamaOnePunchMan => {
                "Saitama (One Punch Man)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SalmaHayek => "Salma Hayek",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SalmanKhan => "Salman Khan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SamAltman => "Sam Altman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SamFender => "Sam Fender",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SamHarris => "Sam Harris",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SamPorterBridges => {
                "Sam Porter Bridges"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SamSmith => "Sam Smith",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SamuelLJackson => {
                "Samuel L. Jackson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SamusAran => "Samus Aran",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SamwiseGamgee => {
                "Samwise Gamgee"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SandraBullock => {
                "Sandra Bullock"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SandyCheeks => "Sandy Cheeks",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SansaStark => "Sansa Stark",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Sapnap => "Sapnap",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SarahConnor => "Sarah Connor",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SasukeUchiha => {
                "Sasuke Uchiha"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SatyaNadella => {
                "Satya Nadella"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SaulGoodman => "Saul Goodman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ScarlettJohansson => {
                "Scarlett Johansson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ScoobyDoo => "Scooby-Doo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ScorpionMortalKombat => {
                "Scorpion (Mortal Kombat)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ScuttleLittleMermaid => {
                "Scuttle (Little Mermaid)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SeanConnery => "Sean Connery",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SeanHannity => "Sean Hannity",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SebastianLittleMermaid => {
                "Sebastian (Little Mermaid)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SebastianManiscalco => {
                "Sebastian Maniscalco"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SelenaGomez => "Selena Gomez",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Sephiroth => "Sephiroth",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SerenaWilliams => {
                "Serena Williams"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SethMacFarlane => {
                "Seth MacFarlane"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SethMeyers => "Seth Meyers",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SethRogen => "Seth Rogen",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SethRollins => "Seth Rollins",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SeverusSnape => {
                "Severus Snape"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SexyyRed => "Sexyy Red",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ShaCarriRichardson => {
                "Sha'Carri Richardson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ShaggyRogers => {
                "Shaggy Rogers"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ShahRukhKhan => {
                "Shah Rukh Khan"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ShaneDawson => "Shane Dawson",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ShaneGillis => "Shane Gillis",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ShannonSharpe => {
                "Shannon Sharpe"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ShaquilleONeal => {
                "Shaquille O'Neal"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ShawnMendes => "Shawn Mendes",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SheldonCooper => {
                "Sheldon Cooper"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SherlockHolmes => {
                "Sherlock Holmes"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SherylSandberg => {
                "Sheryl Sandberg"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ShoheiOhtani => {
                "Shohei Ohtani"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Shrek => "Shrek",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Shroud => "Shroud",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Sia => "Sia",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SidneyCrosby => {
                "Sidney Crosby"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SimonSinek => "Simon Sinek",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SimoneBiles => "Simone Biles",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SimuLiu => "Simu Liu",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SkeletorHeMan => {
                "Skeletor (He-Man)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SkipBayless => "Skip Bayless",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Skrillex => "Skrillex",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SkylerWhite => "Skyler White",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SlashMusician => {
                "Slash (musician)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SnoopDogg => "Snoop Dogg",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SnowWhite => "Snow White",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Sodapoppin => "Sodapoppin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SofiaVergara => {
                "Sofia Vergara"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SolidSnake => "Solid Snake",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SongJoongKi => {
                "Song Joong Ki"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SonicTheHedgehog => {
                "Sonic the Hedgehog"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SoraKingdomHearts => {
                "Sora (Kingdom Hearts)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SpiderManTomHollandVer => {
                "SpiderMan (Tom Holland ver.)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SpikeSpiegel => {
                "Spike Spiegel"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SpongeBobSquarePants => {
                "SpongeBob SquarePants"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Spyro => "Spyro",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SquidwardTentacles => {
                "Squidward Tentacles"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StanMarsh => "Stan Marsh",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StanleyHudson => {
                "Stanley Hudson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StephenASmith => {
                "Stephen A Smith"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StephenColbert => {
                "Stephen Colbert"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StephenCurry => {
                "Stephen Curry"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StephenHawking => {
                "Stephen Hawking"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StephenKing => "Stephen King",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SteveMinecraft => {
                "Steve (Minecraft)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SteveBlum => "Steve Blum",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SteveCarell => "Steve Carell",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SteveHarvey => "Steve Harvey",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SteveIrwin => "Steve Irwin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SteveJobs => "Steve Jobs",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SteveO => "Steve-O",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StevenCrowder => {
                "Steven Crowder"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StevenUniverse => {
                "Steven Universe"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StevieWonder => {
                "Stevie Wonder"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StewieGriffin => {
                "Stewie Griffin"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::StoneColdSteveAustin => {
                "Stone Cold Steve Austin"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Stormzy => "Stormzy",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SubZero => "Sub-Zero",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SummerSmith => "Summer Smith",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Summit1g => "Summit1g",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SundarPichai => {
                "Sundar Pichai"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Superman => "Superman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SydneySweeney => {
                "Sydney Sweeney"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Sykkuno => "Sykkuno",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SylvesterStallone => {
                "Sylvester Stallone"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::SylvesterTheCat => {
                "Sylvester the Cat"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TanaMongeau => "Tana Mongeau",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TanjiroKamado => {
                "Tanjiro Kamado"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TaraStrong => "Tara Strong",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TarajiPHenson => {
                "Taraji P. Henson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TateMcRae => "Tate McRae",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TaylorSwift => "Taylor Swift",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Technoblade => "Technoblade",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Tems => "Tems",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Terminator => "Terminator",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Tfue => "Tfue",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Thanos => "Thanos",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TheDudeBigLebowski => {
                "The Dude (Big Lebowski)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TheGrinch => "The Grinch",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TheHound => "The Hound",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TheJokerAnimated => {
                "The Joker (Animated)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TheMandalorian => {
                "The Mandalorian"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TheRock => "The Rock",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TheUndertaker => {
                "The Undertaker"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TheWeeknd => "The Weeknd",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TheoVon => "Theo Von",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ThorMcu => "Thor (MCU)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TigerWoods => "Tiger Woods",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TimCook => "Tim Cook",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TimDillon => "Tim Dillon",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TimmyTurner => "Timmy Turner",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TimotheeChalamet => {
                "Timothee Chalamet"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Timthetatman => {
                "Timthetatman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TobyFlenderson => {
                "Toby Flenderson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ToddChavez => "Todd Chavez",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TomBrady => "Tom Brady",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TomCruise => "Tom Cruise",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TomHanks => "Tom Hanks",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TomHardy => "Tom Hardy",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TomHaverford => {
                "Tom Haverford"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TomHiddleston => {
                "Tom Hiddleston"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TomHolland => "Tom Holland",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TomKenny => "Tom Kenny",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TomSegura => "Tom Segura",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TommyShelby => "Tommy Shelby",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TonyHawk => "Tony Hawk",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TonyMontanaScarface => {
                "Tony Montana (Scarface)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TonySoprano => "Tony Soprano",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TonyStark => "Tony Stark",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Towelie => "Towelie",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TracerOverwatch => {
                "Tracer (Overwatch)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Trainwreckstv => {
                "Trainwreckstv"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TravisBickle => {
                "Travis Bickle"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TravisKelce => "Travis Kelce",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TravisPastrana => {
                "Travis Pastrana"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TravisScott => "Travis Scott",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TrevorNoah => "Trevor Noah",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TrevorPhilips => {
                "Trevor Philips"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TrevorWallace => {
                "Trevor Wallace"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TripleH => "Triple H",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TrishaPaytas => {
                "Trisha Paytas"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TroyBaker => "Troy Baker",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TroyeSivan => "Troye Sivan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Tubbo => "Tubbo",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TuckerCarlson => {
                "Tucker Carlson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TupacShakur => "Tupac Shakur",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TweetyBird => "Tweety Bird",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TwilightSparkle => {
                "Twilight Sparkle"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TylerDurden => "Tyler Durden",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TylerTheCreator => {
                "Tyler, The Creator"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TyrionLannister => {
                "Tyrion Lannister"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TysonFury => "Tyson Fury",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::UrsulaLittleMermaid => {
                "Ursula (Little Mermaid)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::UsainBolt => "Usain Bolt",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Usher => "Usher",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VBts => "V (BTS)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VCyberpunk => "V (Cyberpunk)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ValentinoRossi => {
                "Valentino Rossi"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Valkyrae => "Valkyrae",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ValorantJett => {
                "Valorant Jett"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VanellopeVonSchweetz => {
                "Vanellope von Schweetz"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Varys => "Varys",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VashTheStampede => {
                "Vash the Stampede"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Vegeta => "Vegeta",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Vegetta777 => "Vegetta777",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Venom => "Venom",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VenusWilliams => {
                "Venus Williams"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VergilDmc => "Vergil (DMC)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ViArcane => "Vi (Arcane)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VictorWembanyama => {
                "Victor Wembanyama"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VinDiesel => "Vin Diesel",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VinesauceVinny => {
                "Vinesauce Vinny"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ViniciusJunior => {
                "Vinicius Junior"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ViolaDavis => "Viola Davis",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ViratKohli => "Virat Kohli",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VitoCorleone => {
                "Vito Corleone"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VivekRamaswamy => {
                "Vivek Ramaswamy"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VladimirPutin => {
                "Vladimir Putin"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Voldemort => "Voldemort",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::VolodymyrZelenskyy => {
                "Volodymyr Zelenskyy"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WalterBishopFringe => {
                "Walter Bishop (Fringe)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WalterWhite => "Walter White",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WarrenBuffett => {
                "Warren Buffett"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WendyTestaburger => {
                "Wendy Testaburger"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WernerHerzog => {
                "Werner Herzog"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WhitneyCummings => {
                "Whitney Cummings"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WhitneyHouston => {
                "Whitney Houston"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WidowmakerOverwatch => {
                "Widowmaker (Overwatch)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WilburSoot => "Wilbur Soot",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WillFerrell => "Will Ferrell",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WillSmith => "Will Smith",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WillemDafoe => "Willem Dafoe",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WilliamWallace => {
                "William Wallace"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WillieNelson => {
                "Willie Nelson"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WillyWonka => "Willy Wonka",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WinnieThePooh => {
                "Winnie the Pooh"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WinstonChurchill => {
                "Winston Churchill"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Wizkid => "Wizkid",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WolfBlitzer => "Wolf Blitzer",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Wolverine => "Wolverine",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WonderWoman => "Wonder Woman",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WoodyToyStory => {
                "Woody (Toy Story)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::WreckItRalph => {
                "Wreck-It Ralph"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::XiJinping => "Xi Jinping",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::YasuoLoL => "Yasuo (LoL)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Yoda => "Yoda",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::YorForger => "Yor Forger",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Yoshi => "Yoshi",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::YuriGagarin => "Yuri Gagarin",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::YusukeUrameshi => {
                "Yusuke Urameshi"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::YzmaEmperorSNewGroove => {
                "Yzma (Emperor's New Groove)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ZachBryan => "Zach Bryan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ZachKing => "Zach King",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ZachKornfeld => {
                "Zach Kornfeld"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ZaneLowe => "Zane Lowe",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ZappBrannigan => {
                "Zapp Brannigan"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ZazuLionKing => {
                "Zazu (Lion King)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Zedd => "Zedd",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Zendaya => "Zendaya",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ZenitsuAgatsuma => {
                "Zenitsu Agatsuma"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ZoeSaldana => "Zoe Saldana",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ZoroOnePiece => {
                "Zoro (One Piece)"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ZukoAvatar => "Zuko (Avatar)",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0001 => "cv-0001",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0002 => "cv-0002",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0003 => "cv-0003",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0004 => "cv-0004",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0005 => "cv-0005",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0006 => "cv-0006",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0007 => "cv-0007",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0008 => "cv-0008",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0009 => "cv-0009",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0010 => "cv-0010",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0011 => "cv-0011",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0012 => "cv-0012",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0013 => "cv-0013",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0014 => "cv-0014",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0015 => "cv-0015",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0016 => "cv-0016",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0017 => "cv-0017",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0018 => "cv-0018",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0019 => "cv-0019",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0020 => "cv-0020",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0021 => "cv-0021",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0022 => "cv-0022",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0023 => "cv-0023",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0024 => "cv-0024",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0025 => "cv-0025",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0026 => "cv-0026",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0028 => "cv-0028",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0029 => "cv-0029",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0030 => "cv-0030",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0031 => "cv-0031",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0032 => "cv-0032",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0033 => "cv-0033",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0034 => "cv-0034",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0035 => "cv-0035",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0036 => "cv-0036",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0037 => "cv-0037",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0038 => "cv-0038",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0039 => "cv-0039",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0040 => "cv-0040",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0041 => "cv-0041",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0042 => "cv-0042",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0043 => "cv-0043",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0044 => "cv-0044",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0045 => "cv-0045",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0046 => "cv-0046",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0047 => "cv-0047",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0048 => "cv-0048",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0049 => "cv-0049",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0050 => "cv-0050",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0051 => "cv-0051",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0052 => "cv-0052",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0053 => "cv-0053",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0054 => "cv-0054",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0055 => "cv-0055",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0056 => "cv-0056",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0057 => "cv-0057",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0058 => "cv-0058",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0059 => "cv-0059",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0060 => "cv-0060",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0061 => "cv-0061",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0062 => "cv-0062",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0063 => "cv-0063",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0064 => "cv-0064",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0065 => "cv-0065",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0066 => "cv-0066",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0067 => "cv-0067",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0068 => "cv-0068",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0069 => "cv-0069",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0070 => "cv-0070",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0071 => "cv-0071",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0072 => "cv-0072",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0073 => "cv-0073",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0074 => "cv-0074",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0075 => "cv-0075",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0076 => "cv-0076",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0077 => "cv-0077",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0078 => "cv-0078",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0079 => "cv-0079",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0080 => "cv-0080",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0081 => "cv-0081",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0082 => "cv-0082",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0083 => "cv-0083",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0084 => "cv-0084",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0085 => "cv-0085",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0086 => "cv-0086",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0087 => "cv-0087",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0088 => "cv-0088",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0089 => "cv-0089",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0090 => "cv-0090",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0091 => "cv-0091",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0092 => "cv-0092",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0093 => "cv-0093",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0094 => "cv-0094",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0095 => "cv-0095",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0096 => "cv-0096",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0097 => "cv-0097",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0098 => "cv-0098",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0099 => "cv-0099",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0100 => "cv-0100",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0101 => "cv-0101",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0102 => "cv-0102",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0103 => "cv-0103",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0104 => "cv-0104",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0105 => "cv-0105",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0106 => "cv-0106",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0107 => "cv-0107",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0108 => "cv-0108",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0109 => "cv-0109",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0110 => "cv-0110",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0111 => "cv-0111",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0112 => "cv-0112",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0113 => "cv-0113",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0114 => "cv-0114",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0115 => "cv-0115",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0116 => "cv-0116",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0117 => "cv-0117",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0118 => "cv-0118",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0119 => "cv-0119",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0120 => "cv-0120",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0121 => "cv-0121",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0122 => "cv-0122",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0123 => "cv-0123",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0124 => "cv-0124",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0125 => "cv-0125",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0126 => "cv-0126",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0127 => "cv-0127",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0128 => "cv-0128",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0129 => "cv-0129",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0130 => "cv-0130",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0131 => "cv-0131",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0132 => "cv-0132",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0133 => "cv-0133",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0134 => "cv-0134",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0135 => "cv-0135",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0136 => "cv-0136",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0137 => "cv-0137",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0138 => "cv-0138",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0139 => "cv-0139",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0140 => "cv-0140",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0141 => "cv-0141",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0142 => "cv-0142",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0143 => "cv-0143",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0144 => "cv-0144",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0145 => "cv-0145",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0146 => "cv-0146",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0147 => "cv-0147",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0148 => "cv-0148",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0149 => "cv-0149",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0150 => "cv-0150",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0151 => "cv-0151",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0152 => "cv-0152",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0153 => "cv-0153",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0154 => "cv-0154",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0155 => "cv-0155",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0156 => "cv-0156",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0157 => "cv-0157",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0158 => "cv-0158",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0159 => "cv-0159",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0160 => "cv-0160",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0161 => "cv-0161",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0162 => "cv-0162",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0163 => "cv-0163",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0164 => "cv-0164",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0165 => "cv-0165",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0166 => "cv-0166",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0167 => "cv-0167",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0168 => "cv-0168",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0169 => "cv-0169",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0170 => "cv-0170",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0171 => "cv-0171",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0172 => "cv-0172",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0173 => "cv-0173",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0174 => "cv-0174",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0175 => "cv-0175",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0176 => "cv-0176",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0177 => "cv-0177",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0178 => "cv-0178",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0179 => "cv-0179",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0180 => "cv-0180",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0181 => "cv-0181",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0182 => "cv-0182",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0183 => "cv-0183",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0184 => "cv-0184",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0185 => "cv-0185",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0186 => "cv-0186",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0187 => "cv-0187",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0188 => "cv-0188",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0189 => "cv-0189",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0190 => "cv-0190",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0191 => "cv-0191",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0192 => "cv-0192",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0193 => "cv-0193",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0194 => "cv-0194",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0195 => "cv-0195",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0196 => "cv-0196",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0197 => "cv-0197",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0198 => "cv-0198",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0199 => "cv-0199",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0200 => "cv-0200",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0201 => "cv-0201",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0202 => "cv-0202",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0203 => "cv-0203",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0205 => "cv-0205",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0206 => "cv-0206",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0207 => "cv-0207",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0208 => "cv-0208",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0209 => "cv-0209",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0210 => "cv-0210",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0211 => "cv-0211",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0212 => "cv-0212",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0213 => "cv-0213",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0214 => "cv-0214",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0215 => "cv-0215",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0216 => "cv-0216",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0217 => "cv-0217",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0218 => "cv-0218",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0219 => "cv-0219",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0220 => "cv-0220",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0221 => "cv-0221",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0222 => "cv-0222",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0223 => "cv-0223",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0224 => "cv-0224",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0225 => "cv-0225",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0226 => "cv-0226",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0227 => "cv-0227",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0228 => "cv-0228",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0229 => "cv-0229",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0230 => "cv-0230",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0231 => "cv-0231",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0232 => "cv-0232",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0233 => "cv-0233",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0234 => "cv-0234",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0235 => "cv-0235",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0236 => "cv-0236",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0237 => "cv-0237",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0238 => "cv-0238",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0239 => "cv-0239",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0240 => "cv-0240",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0241 => "cv-0241",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0242 => "cv-0242",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0243 => "cv-0243",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0244 => "cv-0244",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0245 => "cv-0245",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0246 => "cv-0246",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0247 => "cv-0247",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0248 => "cv-0248",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0249 => "cv-0249",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0250 => "cv-0250",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0251 => "cv-0251",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0252 => "cv-0252",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0253 => "cv-0253",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0254 => "cv-0254",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0255 => "cv-0255",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0256 => "cv-0256",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0257 => "cv-0257",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0258 => "cv-0258",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0259 => "cv-0259",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0260 => "cv-0260",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0261 => "cv-0261",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0262 => "cv-0262",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0263 => "cv-0263",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0264 => "cv-0264",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0265 => "cv-0265",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0266 => "cv-0266",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0267 => "cv-0267",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0268 => "cv-0268",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0269 => "cv-0269",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0270 => "cv-0270",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0271 => "cv-0271",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0272 => "cv-0272",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0273 => "cv-0273",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0274 => "cv-0274",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0275 => "cv-0275",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0276 => "cv-0276",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0277 => "cv-0277",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0278 => "cv-0278",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0279 => "cv-0279",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0280 => "cv-0280",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0281 => "cv-0281",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0282 => "cv-0282",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0283 => "cv-0283",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0284 => "cv-0284",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0285 => "cv-0285",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0286 => "cv-0286",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0287 => "cv-0287",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0288 => "cv-0288",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0289 => "cv-0289",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0290 => "cv-0290",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0291 => "cv-0291",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0292 => "cv-0292",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0293 => "cv-0293",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0294 => "cv-0294",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0295 => "cv-0295",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0296 => "cv-0296",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0297 => "cv-0297",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0298 => "cv-0298",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0299 => "cv-0299",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0300 => "cv-0300",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0301 => "cv-0301",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0302 => "cv-0302",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0303 => "cv-0303",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0304 => "cv-0304",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0305 => "cv-0305",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0306 => "cv-0306",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0307 => "cv-0307",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0308 => "cv-0308",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0309 => "cv-0309",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0310 => "cv-0310",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0311 => "cv-0311",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0312 => "cv-0312",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0313 => "cv-0313",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0314 => "cv-0314",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0315 => "cv-0315",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0316 => "cv-0316",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0317 => "cv-0317",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0318 => "cv-0318",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0319 => "cv-0319",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0320 => "cv-0320",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0321 => "cv-0321",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0322 => "cv-0322",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0323 => "cv-0323",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0324 => "cv-0324",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0325 => "cv-0325",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0326 => "cv-0326",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0327 => "cv-0327",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0328 => "cv-0328",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0329 => "cv-0329",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0330 => "cv-0330",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0331 => "cv-0331",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0332 => "cv-0332",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0333 => "cv-0333",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0334 => "cv-0334",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0335 => "cv-0335",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0336 => "cv-0336",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0337 => "cv-0337",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0338 => "cv-0338",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0339 => "cv-0339",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0340 => "cv-0340",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0341 => "cv-0341",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0342 => "cv-0342",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0343 => "cv-0343",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0344 => "cv-0344",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0345 => "cv-0345",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0346 => "cv-0346",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0347 => "cv-0347",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0348 => "cv-0348",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0349 => "cv-0349",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0350 => "cv-0350",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0351 => "cv-0351",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0352 => "cv-0352",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0353 => "cv-0353",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0354 => "cv-0354",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0355 => "cv-0355",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0356 => "cv-0356",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0357 => "cv-0357",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0358 => "cv-0358",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0359 => "cv-0359",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0360 => "cv-0360",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0361 => "cv-0361",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0362 => "cv-0362",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0363 => "cv-0363",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0364 => "cv-0364",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0365 => "cv-0365",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0366 => "cv-0366",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0367 => "cv-0367",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0368 => "cv-0368",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0369 => "cv-0369",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0370 => "cv-0370",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0371 => "cv-0371",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0372 => "cv-0372",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0373 => "cv-0373",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0374 => "cv-0374",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0375 => "cv-0375",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0376 => "cv-0376",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0377 => "cv-0377",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0378 => "cv-0378",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0379 => "cv-0379",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0380 => "cv-0380",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0381 => "cv-0381",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0382 => "cv-0382",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0383 => "cv-0383",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0384 => "cv-0384",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0385 => "cv-0385",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0386 => "cv-0386",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0387 => "cv-0387",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0388 => "cv-0388",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0389 => "cv-0389",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0390 => "cv-0390",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0391 => "cv-0391",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0392 => "cv-0392",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0393 => "cv-0393",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0394 => "cv-0394",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0395 => "cv-0395",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0396 => "cv-0396",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0397 => "cv-0397",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0398 => "cv-0398",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0399 => "cv-0399",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0400 => "cv-0400",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0401 => "cv-0401",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0402 => "cv-0402",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0403 => "cv-0403",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0404 => "cv-0404",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0405 => "cv-0405",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0406 => "cv-0406",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0407 => "cv-0407",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0408 => "cv-0408",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0409 => "cv-0409",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0410 => "cv-0410",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0411 => "cv-0411",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0412 => "cv-0412",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0413 => "cv-0413",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0414 => "cv-0414",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0415 => "cv-0415",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0416 => "cv-0416",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0417 => "cv-0417",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0418 => "cv-0418",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0419 => "cv-0419",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0420 => "cv-0420",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0421 => "cv-0421",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0422 => "cv-0422",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0423 => "cv-0423",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0424 => "cv-0424",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0425 => "cv-0425",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0426 => "cv-0426",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0427 => "cv-0427",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0428 => "cv-0428",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0429 => "cv-0429",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0430 => "cv-0430",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0431 => "cv-0431",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0432 => "cv-0432",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0433 => "cv-0433",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0434 => "cv-0434",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0435 => "cv-0435",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0436 => "cv-0436",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0437 => "cv-0437",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0438 => "cv-0438",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0439 => "cv-0439",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0440 => "cv-0440",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0441 => "cv-0441",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0442 => "cv-0442",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0443 => "cv-0443",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0444 => "cv-0444",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0445 => "cv-0445",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0446 => "cv-0446",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0447 => "cv-0447",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0448 => "cv-0448",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0449 => "cv-0449",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0450 => "cv-0450",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0451 => "cv-0451",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0452 => "cv-0452",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0453 => "cv-0453",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0454 => "cv-0454",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0455 => "cv-0455",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0456 => "cv-0456",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0457 => "cv-0457",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0458 => "cv-0458",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0459 => "cv-0459",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0460 => "cv-0460",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0461 => "cv-0461",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0462 => "cv-0462",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0463 => "cv-0463",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0464 => "cv-0464",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0465 => "cv-0465",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0466 => "cv-0466",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0467 => "cv-0467",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0469 => "cv-0469",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0470 => "cv-0470",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0471 => "cv-0471",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0472 => "cv-0472",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0473 => "cv-0473",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0474 => "cv-0474",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0475 => "cv-0475",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0476 => "cv-0476",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0477 => "cv-0477",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0478 => "cv-0478",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0479 => "cv-0479",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0480 => "cv-0480",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0481 => "cv-0481",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0482 => "cv-0482",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0483 => "cv-0483",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0484 => "cv-0484",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0485 => "cv-0485",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0486 => "cv-0486",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0487 => "cv-0487",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0488 => "cv-0488",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::Cv0489 => "cv-0489",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::XQc => "xQc",
        };
        write!(f, "{}", str_val)
    }
}
