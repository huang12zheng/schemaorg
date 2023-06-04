//! AUTOGEN, and number relation with IndexMap
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Class {
    GeoShape = 3063280237,
    MonetaryAmountDistribution = 4182472441,
    WorkersUnion = 2354449681,
    Park = 3063902301,
    PetStore = 2141297939,
    DDxElement = 3739312093,
    Florist = 4107388797,
    AnatomicalStructure = 1148157293,
    DataCatalog = 3975243901,
    WebPageElement = 164209220,
    Apartment = 682686783,
    LiteraryEvent = 3820614094,
    MovieClip = 3128327123,
    EducationEvent = 160362512,
    SocialEvent = 1889398367,
    ImagingTest = 3398856307,
    InteractionCounter = 3090026830,
    ParentAudience = 940644531,
    ProductModel = 1671898108,
    PhotographAction = 2035014103,
    FlightReservation = 3315351391,
    MonetaryGrant = 649480962,
    TreatmentIndication = 2914818279,
    Cemetery = 3453476811,
    CategoryCodeSet = 3709361428,
    CheckInAction = 1783538081,
    MusicEvent = 554818348,
    MedicalProcedure = 1369445281,
    FoodEstablishmentReservation = 2394648340,
    CoverArt = 2206206094,
    HealthInsurancePlan = 1835997728,
    SportsOrganization = 152328769,
    CreateAction = 4022667902,
    AutoRepair = 1429241969,
    OnlineBusiness = 743580219,
    Suite = 1316961153,
    TipAction = 47422146,
    LearningResource = 525644022,
    OfferShippingDetails = 1492537954,
    BedAndBreakfast = 3834655431,
    EngineSpecification = 1586646630,
    Bridge = 3711930195,
    OnlineStore = 1318803782,
    Thesis = 1503683529,
    BusinessAudience = 412205789,
    Event = 1214997027,
    BrokerageAccount = 3783064857,
    AutomatedTeller = 1537451953,
    Crematorium = 3036550807,
    MedicalConditionStage = 3240449286,
    AssessAction = 3139790293,
    FireStation = 2548363654,
    CssSelectorType = 762738363,
    Ticket = 1656967007,
    InvestmentOrDeposit = 2549336762,
    CollectionPage = 3072860658,
    PhysicalActivity = 3637443471,
    LiquorStore = 749216674,
    Store = 887141590,
    EducationalOrganization = 785274729,
    EventSeries = 1161024551,
    TrainReservation = 718770048,
    MusicPlaylist = 3913861665,
    VirtualLocation = 610577297,
    AdultEntertainment = 3171389269,
    Recommendation = 2480168515,
    Audiobook = 893164333,
    Patient = 422368991,
    ProductCollection = 2578654287,
    OrganizationRole = 1588609372,
    FindAction = 472261073,
    GeoCircle = 3502448440,
    SportsActivityLocation = 2225828266,
    MeetingRoom = 2366328884,
    LymphaticVessel = 2556373328,
    BoatTrip = 817733381,
    EmployeeRole = 3506820117,
    BookStore = 3421174634,
    UpdateAction = 3794166446,
    MobileApplication = 913720843,
    DiagnosticProcedure = 2772705366,
    Attorney = 395904746,
    TradeAction = 859787905,
    BloodTest = 1280142968,
    RadioStation = 3156994950,
    ComputerStore = 1870520549,
    RentalCarReservation = 2704567250,
    ItemList = 3258153145,
    MerchantReturnPolicySeasonalOverride = 2269392873,
    BuyAction = 3816500264,
    IgnoreAction = 3709084003,
    ArriveAction = 1070309093,
    RecyclingCenter = 3084363620,
    RoofingContractor = 3895554247,
    Specialty = 379894082,
    GeoCoordinates = 4088985883,
    TaxiStand = 1239402491,
    ClothingStore = 219938239,
    VideoObjectSnapshot = 3996739089,
    Guide = 3923524211,
    MoneyTransfer = 2365233527,
    MeasurementTypeEnumeration = 1707587500,
    Festival = 3013069166,
    MusicAlbum = 1089367231,
    AskPublicNewsArticle = 191557196,
    ServiceChannel = 1052409649,
    OccupationalExperienceRequirements = 127326964,
    Organization = 3783806587,
    HealthPlanNetwork = 2980342936,
    TouristTrip = 3840178984,
    HobbyShop = 3985974567,
    Casino = 1128378944,
    SoftwareSourceCode = 2967425656,
    PodcastSeason = 1046350587,
    EventReservation = 33510378,
    PaymentService = 3474963710,
    AggregateRating = 3756027575,
    DataDownload = 2638558944,
    ComicSeries = 3139245936,
    GovernmentService = 4038347356,
    Continent = 3938182057,
    EducationalOccupationalCredential = 1876022922,
    InfectiousDisease = 3790597588,
    MusicGroup = 414702231,
    MedicalTestPanel = 4246116167,
    DrugStrength = 3717624150,
    TypeAndQuantityNode = 544474825,
    MediaSubscription = 3168909052,
    Product = 2639124198,
    MedicalAudience = 1164756255,
    MedicalTherapy = 1692306030,
    RadioEpisode = 41487103,
    MedicalSignOrSymptom = 1287458029,
    VoteAction = 3489783853,
    WPSideBar = 4091598700,
    ApartmentComplex = 1329274165,
    Sculpture = 3200461034,
    TattooParlor = 223892046,
    DepositAccount = 2842287647,
    MovingCompany = 5783710,
    AggregateOffer = 794033946,
    EmploymentAgency = 2010261824,
    Ligament = 1939830830,
    OutletStore = 1701795010,
    InternetCafe = 1338651184,
    Landform = 3379763310,
    AdministrativeArea = 4227990506,
    Role = 390429728,
    ListenAction = 2763407081,
    SocialMediaPosting = 833213540,
    MusicVenue = 105017724,
    Grant = 3333838969,
    PoliceStation = 3799591428,
    CableOrSatelliteService = 2978452501,
    PayAction = 3674447128,
    DeliveryTimeSettings = 3207933065,
    WarrantyPromise = 3304379364,
    MobilePhoneStore = 454569941,
    DrugCost = 3976428227,
    RentAction = 3389999540,
    ShortStory = 1699887454,
    BreadcrumbList = 2268926939,
    LandmarksOrHistoricalBuildings = 3562868180,
    Seat = 2359761768,
    WantAction = 3186622347,
    AudioObject = 4020693352,
    Museum = 3437305350,
    TrainTrip = 3925819840,
    GeospatialGeometry = 178363624,
    HealthAndBeautyBusiness = 3276548529,
    Vein = 1584056631,
    Artery = 1767235988,
    SiteNavigationElement = 1020123668,
    DoseSchedule = 3328437354,
    ConfirmAction = 4179998965,
    FoodService = 3804023410,
    DeactivateAction = 2119112932,
    TheaterGroup = 481634578,
    AutoRental = 2783336400,
    Accommodation = 2223997406,
    InviteAction = 4252559039,
    PodcastSeries = 2304747364,
    SizeSpecification = 1063534101,
    HealthTopicContent = 3161268691,
    CriticReview = 331481564,
    Class = 3148196878,
    Aquarium = 4254724763,
    PublicSwimmingPool = 851047351,
    PodcastEpisode = 2725558244,
    Dataset = 576077455,
    Conversation = 4141354370,
    CovidTestingFacility = 3441458745,
    MedicalIntangible = 3644323338,
    PostalCodeRangeSpecification = 3441704534,
    TaxiReservation = 2009696192,
    NightClub = 2146839646,
    ScheduleAction = 2842772167,
    ScholarlyArticle = 3855259599,
    BuddhistTemple = 1907347172,
    SatiricalArticle = 3321631513,
    FoodEstablishment = 111736795,
    MarryAction = 866299341,
    ProfilePage = 815090326,
    AmusementPark = 2484184778,
    BowlingAlley = 2229233849,
    PaymentCard = 1719570487,
    DataFeed = 1958539939,
    ElectronicsStore = 58498168,
    VideoGameClip = 2198097812,
    TieAction = 2103395133,
    Bone = 2117617827,
    AMRadioChannel = 3880415948,
    EventVenue = 807624515,
    ResearchProject = 2573532178,
    DislikeAction = 2251082390,
    Schedule = 2174150670,
    ContactPage = 1099185372,
    AlignmentObject = 3633024595,
    PaymentChargeSpecification = 341981240,
    ChooseAction = 1103774647,
    WebAPI = 2532137236,
    PlaceOfWorship = 2261007219,
    TVClip = 616410035,
    Mass = 1462921228,
    HyperToc = 3003805096,
    Restaurant = 1556190108,
    GovernmentPermit = 1256326133,
    SportsClub = 3491705444,
    PublicationEvent = 3953631794,
    TravelAgency = 2563546718,
    EnergyEfficiencyEnumeration = 2337605754,
    NailSalon = 3058090091,
    Plumber = 1854326940,
    TouristInformationCenter = 373436538,
    QuoteAction = 3461347542,
    OceanBodyOfWater = 2787626343,
    PlayGameAction = 2845843716,
    ActivateAction = 263445220,
    MenuSection = 2942442423,
    MovieRentalStore = 944333110,
    HomeAndConstructionBusiness = 2436347321,
    EmergencyService = 3456430735,
    Chapter = 122218113,
    Order = 3954251147,
    ArtGallery = 4215319234,
    CreativeWork = 3167797110,
    ComicCoverArt = 607811259,
    LikeAction = 367170961,
    Canal = 3074699419,
    InteractAction = 363333192,
    MathSolver = 306575815,
    Collection = 2760077856,
    ExerciseAction = 1936486832,
    Gene = 3380719445,
    Invoice = 2243306569,
    PostOffice = 1090365160,
    TextDigitalDocument = 1496161209,
    Flight = 2926013625,
    BedType = 3917910174,
    BlogPosting = 162094709,
    Distance = 3583355132,
    LodgingReservation = 4018597856,
    SearchResultsPage = 2289112067,
    TennisComplex = 2372248692,
    Embassy = 3141350999,
    MedicalGuideline = 1950453615,
    CampingPitch = 3810382279,
    LoseAction = 1514736911,
    ElementarySchool = 1608586045,
    EmailMessage = 1477140784,
    SaleEvent = 2199641104,
    ControlAction = 312908743,
    MediaReviewItem = 3027814233,
    ImageObjectSnapshot = 2337233725,
    Pharmacy = 952998150,
    ContactPoint = 2418257318,
    BusTrip = 3074203990,
    AutomotiveBusiness = 1110914186,
    Rating = 3668682394,
    Physician = 3708495063,
    Review = 3472772916,
    Campground = 1666126514,
    Joint = 3528633727,
    MerchantReturnPolicy = 487939298,
    CompleteDataFeed = 688793340,
    Article = 3231440483,
    City = 3278365814,
    HealthPlanCostSharingSpecification = 1834432599,
    MedicalObservationalStudy = 3069831681,
    AudioObjectSnapshot = 2978283008,
    Statement = 1504855218,
    Trip = 2812681522,
    MedicalIndication = 3138361118,
    ResearchOrganization = 2582447353,
    QAPage = 1423248441,
    Playground = 3315351315,
    ChemicalSubstance = 2357301383,
    SubwayStation = 809319512,
    SomeProducts = 4169817941,
    MonetaryAmount = 3925606266,
    InsertAction = 1121966723,
    ProductGroup = 3295671310,
    RecommendedDoseSchedule = 2747247739,
    ActionAccessSpecification = 2129446075,
    EntertainmentBusiness = 3085576327,
    Beach = 2641765409,
    ReactAction = 3565224897,
    MedicalScholarlyArticle = 1865666501,
    OfferForLease = 1918342880,
    CatholicChurch = 1416709879,
    FinancialProduct = 2921691899,
    ExhibitionEvent = 2443185882,
    MedicalEntity = 1396945767,
    ResumeAction = 4050703309,
    ProgramMembership = 1785210582,
    DiscoverAction = 2659365075,
    CafeOrCoffeeShop = 766937104,
    DryCleaningOrLaundry = 3698531307,
    RVPark = 3372858784,
    DigitalDocument = 3251274545,
    OpeningHoursSpecification = 2850333107,
    PresentationDigitalDocument = 4178374587,
    ProfessionalService = 308016316,
    Book = 3602199667,
    BankOrCreditUnion = 1698405615,
    PhysicalTherapy = 1377909069,
    Drug = 1500797393,
    ReservationPackage = 4126924779,
    AutoBodyShop = 82604823,
    CheckAction = 658738387,
    LegislationObject = 2321637683,
    Airport = 1695455061,
    AmpStory = 3651507578,
    CookAction = 3508232807,
    MedicalWebPage = 1035626704,
    StadiumOrArena = 2951524208,
    ReserveAction = 2713906469,
    BusinessEvent = 351998532,
    Residence = 2928334537,
    BroadcastChannel = 4268236265,
    MusicComposition = 156075518,
    WinAction = 3481817760,
    HowToTip = 2658912749,
    Hackathon = 138561980,
    StatisticalPopulation = 2585831504,
    WriteAction = 1277155992,
    HowToSection = 3393201292,
    FinancialService = 1135743627,
    HVACBusiness = 3137507655,
    RepaymentSpecification = 193303094,
    DeliveryEvent = 4012812112,
    ComputerLanguage = 399121665,
    PeopleAudience = 2244011695,
    ReportedDoseSchedule = 84731078,
    Motorcycle = 244202321,
    FollowAction = 2917988119,
    VideoGame = 1115600078,
    OpinionNewsArticle = 3757957557,
    DiscussionForumPosting = 1336418225,
    NGO = 610315440,
    BusStation = 369853720,
    Substance = 1405611026,
    TrainStation = 3013843410,
    SportingGoodsStore = 3675049741,
    UnRegisterAction = 167630604,
    ToyStore = 768988288,
    MedicalSign = 420964363,
    MovieSeries = 284568182,
    Car = 187808039,
    Pond = 1687546523,
    PropertyValueSpecification = 3002950929,
    BorrowAction = 2027004062,
    XPathType = 3773115658,
    RadioChannel = 1818092116,
    Thing = 880401820,
    ImageObject = 390492701,
    VitalSign = 1278032446,
    GardenStore = 3374299212,
    SearchRescueOrganization = 1224113290,
    TherapeuticProcedure = 1688679953,
    BrainStructure = 3617030550,
    Recipe = 2724388401,
    ReplaceAction = 3085510432,
    ChildCare = 325510054,
    LocationFeatureSpecification = 3894098464,
    GovernmentOffice = 466224521,
    Taxon = 1536513674,
    AccountingService = 422895069,
    Vessel = 3907653321,
    CommunicateAction = 2961523003,
    WebApplication = 3561761588,
    LodgingBusiness = 2715641726,
    HowToTool = 2507474457,
    InvestmentFund = 2586293745,
    RealEstateAgent = 2533637403,
    AdvertiserContentArticle = 1437560733,
    Drawing = 2797822033,
    RegisterAction = 2568335517,
    MortgageLoan = 3009969867,
    CorrectionComment = 976568125,
    Periodical = 3517511705,
    BusStop = 2497677446,
    OfficeEquipmentStore = 20234379,
    Church = 712480197,
    PrependAction = 1712987854,
    SkiResort = 2958499734,
    MedicalTrial = 926355860,
    HowTo = 1246552468,
    DanceGroup = 3324787020,
    Photograph = 475646065,
    HousePainter = 178068149,
    RealEstateListing = 3064436868,
    SellAction = 3763972740,
    ShareAction = 1257868466,
    Bakery = 3934395348,
    WholesaleStore = 3032103340,
    RejectAction = 1807846235,
    WarrantyScope = 3880811969,
    SpeakableSpecification = 3570567828,
    DepartmentStore = 3467409061,
    GasStation = 2497447486,
    PlanAction = 2129403953,
    MotorcycleDealer = 87135867,
    MedicalOrganization = 2445312183,
    OfferCatalog = 2600941193,
    GeneralContractor = 1331596781,
    Observation = 1282868402,
    CategoryCode = 219342986,
    DrinkAction = 3527133556,
    PlayAction = 3715644072,
    Report = 1340097818,
    DaySpa = 1133887289,
    AppendAction = 1811240919,
    Occupation = 3854097236,
    VisualArtwork = 3349775247,
    RadiationTherapy = 194738855,
    RadioSeries = 3492435606,
    MedicalSymptom = 4293339304,
    APIReference = 3581051814,
    MedicalRiskScore = 306959799,
    CreativeWorkSeries = 4218221908,
    Hotel = 77175590,
    AskAction = 413493874,
    MediaGallery = 3760998681,
    SizeGroupEnumeration = 1648930722,
    SchoolDistrict = 2862638307,
    LinkRole = 3585478518,
    TVEpisode = 1415614604,
    FMRadioChannel = 2982943695,
    Menu = 2513827951,
    Clip = 396576312,
    DownloadAction = 2280436770,
    PerformingArtsTheater = 3470195058,
    GatedResidenceCommunity = 1271212807,
    Zoo = 193828908,
    JoinAction = 2642372427,
    IceCreamShop = 2230865676,
    HinduTemple = 696677838,
    CancelAction = 1789214612,
    RadioSeason = 712728486,
    Dentist = 3478991013,
    WebPage = 528190571,
    DrugClass = 486154727,
    Game = 3078696287,
    CityHall = 2277190616,
    HowToDirection = 3644700495,
    MoveAction = 2926551542,
    Manuscript = 2411065772,
    BookSeries = 2452678847,
    TouristDestination = 1473234642,
    ShippingRateSettings = 604063281,
    RadioBroadcastService = 2509245984,
    NonprofitType = 3837610123,
    MusicStore = 2472245299,
    InstallAction = 3300438133,
    Message = 2927382241,
    BarOrPub = 2543399649,
    IndividualProduct = 1632510009,
    SportsTeam = 2541268568,
    NewsArticle = 930748286,
    HairSalon = 4193958983,
    UseAction = 1097675201,
    HowToStep = 551448055,
    HardwareStore = 1530744082,
    EndorsementRating = 3046487706,
    Brewery = 3923298944,
    Country = 167299156,
    BoatTerminal = 2947245162,
    Play = 1701079351,
    ParkingFacility = 2641553035,
    MedicalDevice = 3459175480,
    Winery = 3332373863,
    CheckOutAction = 4129898374,
    ArchiveOrganization = 1058414634,
    LoanOrCredit = 3347647090,
    StatusEnumeration = 1245022955,
    GroceryStore = 1254563452,
    OccupationalTherapy = 3040962034,
    Barcode = 1578475611,
    VeterinaryCare = 1035651939,
    BeautySalon = 3571213144,
    Vehicle = 3208424073,
    WebSite = 3673562719,
    Place = 1947560904,
    Electrician = 4167915993,
    BusinessEntityType = 1340462645,
    ReturnAction = 2891052125,
    PerformanceRole = 2981894523,
    MedicalBusiness = 2514000923,
    MedicalClinic = 3449248748,
    Protein = 1345948930,
    TrackAction = 957746539,
    DeliveryChargeSpecification = 446950446,
    ComicIssue = 3356001704,
    AgreeAction = 507876295,
    AcceptAction = 2372162475,
    Permit = 3211255315,
    CDCPMDRecord = 3027607288,
    MensClothingStore = 3182643127,
    Room = 1717748581,
    CollegeOrUniversity = 639573119,
    SheetMusic = 4163838331,
    WPAdBlock = 3268638029,
    SeaBodyOfWater = 4060249393,
    VisualArtsEvent = 2648891928,
    Language = 369384837,
    PathologyTest = 3988501505,
    Offer = 1263425085,
    VideoGameSeries = 2346512107,
    OnDemandEvent = 586130675,
    HealthClub = 1300792384,
    LegalService = 1887927757,
    FastFoodRestaurant = 1916466957,
    BusinessFunction = 1506594538,
    URL = 2152005672,
    EducationalAudience = 952197171,
    Legislation = 529583897,
    DepartAction = 1363699685,
    EnergyConsumptionDetails = 3315728720,
    Painting = 2619471129,
    CreditCard = 3007825237,
    ApplyAction = 2976543306,
    GiveAction = 149549676,
    ListItem = 3279134546,
    TVSeries = 3746453176,
    FloorPlan = 2294774779,
    Enumeration = 214438226,
    Model3D = 3420018731,
    BackgroundNewsArticle = 3987582786,
    Diet = 1689428127,
    House = 504294061,
    Course = 2283636166,
    SolveMathAction = 3948515171,
    Volcano = 431739730,
    ViewAction = 43128084,
    DefinedTerm = 781258347,
    SoftwareApplication = 3728622662,
    PawnShop = 850570862,
    AchieveAction = 4217411194,
    DanceEvent = 4212652359,
    ApprovedIndication = 2077034972,
    MedicalGuidelineContraindication = 4130261471,
    GolfCourse = 1036546497,
    BefriendAction = 247824310,
    Motel = 2790854515,
    EatAction = 1627798830,
    PreventionIndication = 1571615206,
    MolecularEntity = 1546573404,
    FAQPage = 1125474165,
    ReportageNewsArticle = 1394959576,
    QuantitativeValue = 529140644,
    PerformingGroup = 2258647018,
    Blog = 3753866718,
    DeleteAction = 2713432861,
    BikeStore = 3515442464,
    HighSchool = 268616826,
    Synagogue = 21739283,
    PalliativeProcedure = 1270283956,
    Optician = 2329418361,
    TelevisionChannel = 914173397,
    MedicalStudy = 4011164629,
    PublicationVolume = 2296844734,
    ShippingDeliveryTime = 1605568723,
    UnitPriceSpecification = 1559435741,
    RadioClip = 4237142469,
    TechArticle = 846973353,
    Hostel = 2306712207,
    PsychologicalTreatment = 1674332530,
    PerformAction = 3500452885,
    ChildrensEvent = 3436435929,
    VideoObject = 1596250207,
    TaxiService = 1751774596,
    SendAction = 3837704131,
    Demand = 3936492823,
    PaymentMethod = 1754544675,
    BankAccount = 133365085,
    SubscribeAction = 3905951624,
    Waterfall = 1752224946,
    TakeAction = 1871603238,
    State = 356097102,
    LeaveAction = 953768091,
    AllocateAction = 1543206499,
    Property = 551966814,
    TVSeason = 2132153524,
    WPHeader = 3312357014,
    CreativeWorkSeason = 1780104924,
    ShoppingCenter = 2487207617,
    MedicalCode = 1670597973,
    PronounceableText = 3729806761,
    MotorcycleRepair = 2380145568,
    WorkBasedProgram = 702630303,
    MedicalCondition = 487517508,
    DefinedRegion = 1503505177,
    Distillery = 3004582041,
    BusOrCoach = 2829183950,
    TransferAction = 1956675653,
    Researcher = 277502260,
    Quiz = 3704205625,
    Airline = 3658079611,
    ConsumeAction = 923017699,
    ReceiveAction = 2986300664,
    ReplyAction = 463696386,
    SearchAction = 3342315204,
    MedicalRiskCalculator = 3351276539,
    MovieTheater = 3146314340,
    WearAction = 1062605391,
    BusReservation = 1215672877,
    ArchiveComponent = 3648700516,
    Library = 2569774964,
    SpecialAnnouncement = 1134800673,
    EmployerReview = 1360702610,
    WebContent = 2720097115,
    Reservation = 378399886,
    HyperTocEntry = 1937422253,
    MedicalTest = 1188205705,
    SurgicalProcedure = 2452799456,
    DataType = 2099164604,
    VideoGallery = 3140079880,
    Quantity = 2393424789,
    Series = 2695504145,
    ScreeningEvent = 985665060,
    Claim = 3825893916,
    Mosque = 512089925,
    LibrarySystem = 1014403862,
    Nerve = 3084254875,
    Notary = 1298341718,
    WatchAction = 3249142537,
    AutoWash = 419173768,
    CommentAction = 3001431695,
    JewelryStore = 2516791768,
    ReviewAction = 2845652952,
    AnimalShelter = 1670529639,
    ImageGallery = 3152772575,
    LiveBlogPosting = 2304107019,
    PublicToilet = 3125200508,
    MedicalContraindication = 3151138621,
    ComedyEvent = 2890174748,
    SuspendAction = 2483326090,
    LendAction = 85802753,
    FundingScheme = 3708841186,
    TelevisionStation = 2993176799,
    MotorizedBicycle = 2690762687,
    Poster = 1466612170,
    Intangible = 1636672989,
    LifestyleModification = 2603837221,
    MiddleSchool = 2253475576,
    ComicStory = 3947876016,
    LakeBodyOfWater = 646556452,
    AboutPage = 2875970914,
    QualitativeValue = 729864965,
    GameServer = 1980139044,
    StructuredValue = 497459540,
    PreOrderAction = 3242804670,
    Duration = 1472349855,
    BroadcastEvent = 2523963321,
    MedicalRiskFactor = 955746368,
    Comment = 2455803113,
    ConvenienceStore = 3547777386,
    SingleFamilyResidence = 4121120104,
    MusicRelease = 2965431784,
    EmployerAggregateRating = 2886827758,
    Episode = 1628677166,
    ReviewNewsArticle = 804513427,
    PriceSpecification = 1066091549,
    Person = 2549511217,
    HomeGoodsStore = 3869715377,
    ClaimReview = 3384033369,
    NutritionInformation = 3328729716,
    EducationalOccupationalProgram = 1975862564,
    MenuItem = 1100038093,
    TravelAction = 339489896,
    Project = 2551227071,
    MaximumDoseSchedule = 475809200,
    Brand = 1888860708,
    HowToSupply = 1476875970,
    Audience = 398801668,
    ParcelDelivery = 3591904189,
    SeekToAction = 24951241,
    AuthorizeAction = 1511983620,
    TouristAttraction = 4038341330,
    Energy = 934108244,
    GovernmentBuilding = 4263714583,
    OfferForPurchase = 523024225,
    EntryPoint = 2190158732,
    Table = 855638908,
    DefinedTermSet = 3407026645,
    FoodEvent = 68604962,
    OrganizeAction = 924455099,
    AutoDealer = 4266522042,
    InsuranceAgency = 3265930214,
    MusicRecording = 180129094,
    Courthouse = 3154782988,
    ComedyClub = 1982935749,
    SpreadsheetDigitalDocument = 2563125099,
    Locksmith = 3584346950,
    MediaObject = 1398002052,
    DietarySupplement = 1960977687,
    OwnershipInfo = 2578564932,
    BioChemEntity = 3145988745,
    NoteDigitalDocument = 3782962735,
    Float = 1406957670,
    Consortium = 501247803,
    GovernmentOrganization = 2314910969,
    BroadcastService = 2261952042,
    CurrencyConversionService = 1680204812,
    AssignAction = 1593171174,
    DigitalDocumentPermission = 3631915742,
    BookmarkAction = 1227591708,
    BedDetails = 3917380448,
    Muscle = 4083220112,
    Action = 1195407118,
    BroadcastFrequencySpecification = 694326348,
    FundingAgency = 2647876347,
    CourseInstance = 2889289480,
    HowToItem = 2354084576,
    ExchangeRateSpecification = 959777778,
    SportsEvent = 3444481572,
    MedicalCause = 2683326288,
    HealthPlanFormulary = 3931829246,
    FilmAction = 4181217029,
    SelfStorage = 1916941542,
    WPFooter = 3933121837,
    DataFeedItem = 3098990227,
    CompoundPriceSpecification = 3614018760,
    QuantitativeValueDistribution = 1937131543,
    AutoPartsStore = 413357805,
    Hospital = 2567442392,
    EndorseAction = 3564480611,
    InformAction = 340578298,
    MedicalEnumeration = 4002047705,
    BoatReservation = 1215012047,
    SuperficialAnatomy = 2761876036,
    TheaterEvent = 1683212659,
    ReadAction = 3000670489,
    Answer = 2279710224,
    BodyOfWater = 3381548926,
    Map = 3150524125,
    PostalAddress = 567863452,
    JobPosting = 2346671253,
    DonateAction = 2436466132,
    DrawAction = 468236371,
    ExerciseGym = 1665764490,
    AddAction = 761819158,
    LegislativeBuilding = 1380183577,
    ShoeStore = 1499025629,
    MedicalRiskEstimator = 4072298662,
    FurnitureStore = 1399983334,
    MusicVideoObject = 1863220320,
    DrugLegalStatus = 3443340223,
    TireShop = 2473959109,
    PublicationIssue = 3742235104,
    Service = 4137625517,
    Mountain = 746910255,
    Corporation = 742456081,
    RsvpAction = 2142622794,
    UserReview = 757191382,
    Atlas = 3030496764,
    PaintAction = 2059037712,
    OrderAction = 3523588500,
    Newspaper = 3330509725,
    RiverBodyOfWater = 1818531595,
    Question = 2290374408,
    DiagnosticLab = 1106041431,
    CheckoutPage = 3605366747,
    Resort = 1551700861,
    NewsMediaOrganization = 2240631967,
    DefenceEstablishment = 2146595699,
    MedicalGuidelineRecommendation = 2599159960,
    HotelRoom = 2480853980,
    School = 370715837,
    AnalysisNewsArticle = 3101212205,
    AnatomicalSystem = 3934909117,
    MediaReview = 2418858879,
    ExercisePlan = 2137341291,
    Quotation = 704316229,
    DisagreeAction = 2861261484,
    PropertyValue = 2057813630,
    ItemPage = 1229526780,
    Reservoir = 258686492,
    LocalBusiness = 189234298,
    Movie = 4149623060,
    Integer = 3880829199,
    OrderItem = 3766349200,
    Preschool = 3505475849,
    CivicStructure = 2545450692,
}