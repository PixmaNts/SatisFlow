#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Item {
    AILimiter,
    AdaptiveControlUnit,
    AlcladAluminumSheet,
    AlienDNACapsule,
    AlienProtein,
    AluminaSolution,
    AluminumCasing,
    AluminumIngot,
    AluminumScrap,
    AssemblyDirectorSystem,
    AutomatedWiring,
    BaconAgaric,
    Battery,
    Bauxite,
    Beacon,
    BerylNut,
    Biomass,
    BlackPowder,
    BladeRunners,
    BluePowerSlug,
    Cable,
    CateriumIngot,
    CateriumOre,
    Chainsaw,
    CircuitBoard,
    ClusterNobelisk,
    Coal,
    ColorCartridge,
    CompactedCoal,
    Computer,
    Concrete,
    CoolingSystem,
    CopperIngot,
    CopperOre,
    CopperPowder,
    CopperSheet,
    CrudeOil,
    CrystalOscillator,
    ElectromagneticControlRod,
    EmptyCanister,
    EmptyFluidTank,
    EncasedIndustrialBeam,
    EncasedPlutoniumCell,
    EncasedUraniumCell,
    ExplosiveRebar,
    Fabric,
    FactoryCart,
    FlowerPetals,
    Fuel,
    FusedModularFrame,
    GasFilter,
    GasMask,
    GasNobelisk,
    GoldenFactoryCart,
    HUBParts,
    HatcherRemains,
    HazmatSuit,
    HeatSink,
    HeavyModularFrame,
    HeavyOilResidue,
    HighSpeedConnector,
    HogRemains,
    HomingRifleAmmo,
    HoverPack,
    IodineInfusedFilter,
    IronIngot,
    IronOre,
    IronPlate,
    IronRebar,
    IronRod,
    Jetpack,
    Leaves,
    Limestone,
    LiquidBiofuel,
    MagneticFieldGenerator,
    MedicinalInhaler,
    ModularEngine,
    ModularFrame,
    Motor,
    Mycelia,
    NitricAcid,
    NitrogenGas,
    Nobelisk,
    NobeliskDetonator,
    NonFissileUranium,
    NuclearPasta,
    NukeNobelisk,
    ObjectScanner,
    PackagedAluminaSolution,
    PackagedFuel,
    PackagedHeavyOilResidue,
    PackagedLiquidBiofuel,
    PackagedNitricAcid,
    PackagedNitrogenGas,
    PackagedOil,
    PackagedSulfuricAcid,
    PackagedTurbofuel,
    PackagedWater,
    Paleberry,
    Parachute,
    PetroleumCoke,
    PlasmaSpitterRemains,
    Plastic,
    PlutoniumFuelRod,
    PlutoniumPellet,
    PlutoniumWaste,
    PolymerResin,
    PortableMiner,
    PowerShard,
    PressureConversionCube,
    PulseNobelisk,
    PurplePowerSlug,
    QuartzCrystal,
    Quickwire,
    RadioControlUnit,
    RawQuartz,
    RebarGun,
    ReinforcedIronPlate,
    Rifle,
    RifleAmmo,
    Rotor,
    Rubber,
    Screw,
    ShatterRebar,
    Silica,
    SmartPlating,
    SmokelessPowder,
    SolidBiofuel,
    Stator,
    SteelBeam,
    SteelIngot,
    SteelPipe,
    StingerRemains,
    StunRebar,
    Sulfur,
    SulfuricAcid,
    Supercomputer,
    ThermalPropulsionRocket,
    TurboMotor,
    TurboRifleAmmo,
    Turbofuel,
    Uranium,
    UraniumFuelRod,
    UraniumWaste,
    VersatileFramework,
    Water,
    Wire,
    Wood,
    XenoBasher,
    XenoZapper,
    YellowPowerSlug,
    Zipline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Recipe {
    AILimiter,
    AWESOMEShop,
    AWESOMESink,
    AdaptiveControlUnit,
    AlcladAluminumSheet,
    AlienDNACapsule,
    AlternateAdheredIronPlate,
    AlternateAlcladCasing,
    AlternateAutomatedMiner,
    AlternateAutomatedSpeedWiring,
    AlternateBiocoal,
    AlternateBoltedFrame,
    AlternateBoltedIronPlate,
    AlternateCastScrew,
    AlternateCateriumCircuitBoard,
    AlternateCateriumComputer,
    AlternateCateriumWire,
    AlternateCharcoal,
    AlternateCheapSilica,
    AlternateClassicBattery,
    AlternateCoatedCable,
    AlternateCoatedIronCanister,
    AlternateCoatedIronPlate,
    AlternateCokeSteelIngot,
    AlternateCompactedCoal,
    AlternateCompactedSteelIngot,
    AlternateCoolingDevice,
    AlternateCopperAlloyIngot,
    AlternateCopperRotor,
    AlternateCrystalBeacon,
    AlternateCrystalComputer,
    AlternateDilutedFuel,
    AlternateDilutedPackagedFuel,
    AlternateElectricMotor,
    AlternateElectrodeAluminumScrap,
    AlternateElectrodeCircuitBoard,
    AlternateElectromagneticConnectionRod,
    AlternateEncasedIndustrialPipe,
    AlternateFertileUranium,
    AlternateFineBlackPowder,
    AlternateFineConcrete,
    AlternateFlexibleFramework,
    AlternateFusedQuickwire,
    AlternateFusedWire,
    AlternateHeatExchanger,
    AlternateHeatFusedFrame,
    AlternateHeavyEncasedFrame,
    AlternateHeavyFlexibleFrame,
    AlternateHeavyOilResidue,
    AlternateInfusedUraniumCell,
    AlternateInstantPlutoniumCell,
    AlternateInstantScrap,
    AlternateInsulatedCable,
    AlternateInsulatedCrystalOscillator,
    AlternateIronAlloyIngot,
    AlternateIronWire,
    AlternateOCSupercomputer,
    AlternatePlasticSmartPlating,
    AlternatePlutoniumFuelUnit,
    AlternatePolyesterFabric,
    AlternatePolymerResin,
    AlternatePureAluminumIngot,
    AlternatePureCateriumIngot,
    AlternatePureCopperIngot,
    AlternatePureIronIngot,
    AlternatePureQuartzCrystal,
    AlternateQuickwireCable,
    AlternateQuickwireStator,
    AlternateRadioConnectionUnit,
    AlternateRadioControlSystem,
    AlternateRecycledPlastic,
    AlternateRecycledRubber,
    AlternateRigourMotor,
    AlternateRubberConcrete,
    AlternateSiliconCircuitBoard,
    AlternateSiliconHighSpeedConnector,
    AlternateSloppyAlumina,
    AlternateSolidSteelIngot,
    AlternateSteamedCopperSheet,
    AlternateSteelCanister,
    AlternateSteelCoatedPlate,
    AlternateSteelRod,
    AlternateSteelRotor,
    AlternateSteelScrew,
    AlternateSteeledFrame,
    AlternateStitchedIronPlate,
    AlternateSuperStateComputer,
    AlternateTurboBlendFuel,
    AlternateTurboElectricMotor,
    AlternateTurboHeavyFuel,
    AlternateTurboPressureMotor,
    AlternateUraniumFuelUnit,
    AlternateWetConcrete,
    AluminaSolution,
    AluminumCasing,
    AluminumIngot,
    AluminumScrap,
    Assembler,
    AssemblyDirectorSystem,
    AutomatedGate,
    AutomatedWiring,
    BasicWall1m,
    BasicWall4m,
    Battery,
    Beacon,
    BeamConnector,
    BeamConnectorDouble,
    BeamSupport,
    BigConcretePillar,
    BigFramePillar,
    BigMetalPillar,
    BigPillarSupport,
    BiomassAlienProtein,
    BiomassBurner,
    BiomassLeaves,
    BiomassMycelia,
    BiomassWood,
    BlackPowder,
    BladeRunners,
    Blender,
    BlockSignal,
    BlueprintDesigner,
    Cable,
    CandyCane,
    CateriumIngot,
    CatwalkCorner,
    CatwalkCrossing,
    CatwalkRamp,
    CatwalkStairs,
    CatwalkStraight,
    CatwalkTCrossing,
    CeilingLight,
    CenterDoorWall,
    Chainsaw,
    CircuitBoard,
    ClusterNobelisk,
    CoalGenerator,
    ColorCartridge,
    Computer,
    Concrete,
    Constructor,
    ConveyorBeltMk1,
    ConveyorBeltMk2,
    ConveyorBeltMk3,
    ConveyorBeltMk4,
    ConveyorBeltMk5,
    ConveyorCeilingMount,
    ConveyorLiftFloorHole,
    ConveyorLiftMk1,
    ConveyorLiftMk2,
    ConveyorLiftMk3,
    ConveyorLiftMk4,
    ConveyorLiftMk5,
    ConveyorMerger,
    ConveyorPole,
    ConveyorSplitter,
    ConveyorWallMount,
    ConveyorWallX1,
    ConveyorWallX2,
    ConveyorWallX3,
    CoolingSystem,
    CopperIngot,
    CopperPowder,
    CopperSheet,
    CraftBench,
    CrystalOscillator,
    CyberWagon,
    DisplaySign,
    DoubleRamp2m,
    DoubleRamp4m,
    DoubleRamp8m,
    DoubleWallOutletMk1,
    DoubleWallOutletMk2,
    DoubleWallOutletMk3,
    DownCornerRamp1m,
    DownCornerRamp2m,
    DownCornerRamp4m,
    Drone,
    DronePort,
    ElectricLocomotive,
    ElectromagneticControlRod,
    EmptyCanister,
    EmptyFluidTank,
    EmptyPlatform,
    EmptyPlatformWithCatwalk,
    EncasedIndustrialBeam,
    EncasedPlutoniumCell,
    EncasedUraniumCell,
    EquipmentWorkshop,
    Explorer,
    ExplosiveRebar,
    FICSMASGiftTree,
    FICSMASPowerLight,
    FICSMASSnowDispenser,
    FICSMASWreath,
    Fabric,
    FactoryCart,
    FloodLightTower,
    FluidBuffer,
    FluidFreightPlatform,
    Foundation1m,
    Foundation2m,
    Foundation4m,
    Foundry,
    FrameFloor,
    FrameFoundation,
    FrameRamp,
    FrameWall,
    FrameWindow,
    FreightCar,
    FreightPlatform,
    Fuel,
    FuelGenerator,
    FullFrameWindow,
    FusedModularFrame,
    GasFilter,
    GasMask,
    GasNobelisk,
    GateHoleWall,
    GeothermalGenerator,
    GiantFICSMASTree,
    GlassFrameFoundation,
    GoldenFactoryCart,
    Half1mFoundation,
    Half2mFoundation,
    Half4mFoundation,
    HatcherProtein,
    HazardStorageBox,
    HazmatSuit,
    HeatSink,
    HeavyModularFrame,
    HexFrameWindow,
    HighSpeedConnector,
    HogProtein,
    HomingRifleAmmo,
    HoverPack,
    Hypertube,
    HypertubeEntrance,
    HypertubeFloorHole,
    HypertubeSupport,
    HypertubeWallHole,
    HypertubeWallSupport,
    IndustrialFluidBuffer,
    IndustrialStorageContainer,
    InnerCornerExtension1m,
    InnerCornerExtension2m,
    InnerCornerExtension4m,
    InnerCornerQuarterPipe,
    InnerCornerRoof1m,
    InnerCornerRoof2m,
    InnerCornerRoof4m,
    InvDownCorner1m,
    InvDownCorner2m,
    InvDownCorner4m,
    InvRamp1m,
    InvRamp2m,
    InvRamp4m,
    InvRampWall1m,
    InvRampWall2m,
    InvRampWall4m,
    InvRampWall8m,
    InvUpCorner1m,
    InvUpCorner2m,
    InvUpCorner4m,
    InvertedFrameRamp,
    InvertedInnerCornerQuarterPipe,
    InvertedOuterCornerQuarterPipe,
    InvertedQuarterPipe,
    IodineInfusedFilter,
    IronIngot,
    IronPlate,
    IronRebar,
    IronRod,
    Jetpack,
    JumpPad,
    LabelSign2m,
    LabelSign3m,
    LabelSign4m,
    Ladder,
    LargeBillboard,
    LightsControlPanel,
    LiquidBiofuel,
    LookoutTower,
    MAM,
    MagneticFieldGenerator,
    Manufacturer,
    MedicalStorageBox,
    MetalBeam,
    MinerMk1,
    MinerMk2,
    MinerMk3,
    ModernRailing,
    ModularEngine,
    ModularFrame,
    Motor,
    NitricAcid,
    Nobelisk,
    NobeliskDetonator,
    NonFissileUranium,
    NuclearPasta,
    NuclearPowerPlant,
    NukeNobelisk,
    NutritionalInhaler,
    ObjectScanner,
    OilExtractor,
    OuterCornerExtension1m,
    OuterCornerExtension2m,
    OuterCornerExtension4m,
    OuterCornerQuarterPipe,
    OuterCornerRoof1m,
    OuterCornerRoof2m,
    OuterCornerRoof4m,
    PackagedAluminaSolution,
    PackagedFuel,
    PackagedHeavyOilResidue,
    PackagedLiquidBiofuel,
    PackagedNitricAcid,
    PackagedNitrogenGas,
    PackagedOil,
    PackagedSulfuricAcid,
    PackagedTurbofuel,
    PackagedWater,
    Packager,
    PaintedBeam,
    PanelWindow,
    Parachute,
    ParticleAccelerator,
    PathSignal,
    PersonalStorageBox,
    PetroleumCoke,
    PipelineFloorHole,
    PipelineJunctionCross,
    PipelineMk1,
    PipelineMk1NoIndicator,
    PipelineMk2,
    PipelineMk2NoIndicator,
    PipelinePumpMk1,
    PipelinePumpMk2,
    PipelineSupport,
    PipelineWallHole,
    PipelineWallSupport,
    Plastic,
    PlutoniumFuelRod,
    PlutoniumPellet,
    PortableMiner,
    PortraitSign,
    PowerLine,
    PowerPoleMk1,
    PowerPoleMk2,
    PowerPoleMk3,
    PowerShard1,
    PowerShard2,
    PowerShard5,
    PowerStorage,
    PowerSwitch,
    PowerTower,
    PowerTowerPlatform,
    PressureConversionCube,
    PriorityPowerSwitch,
    ProgrammableSplitter,
    ProteinInhaler,
    PulseNobelisk,
    QuarterPipe,
    QuartzCrystal,
    Quickwire,
    RadarTower,
    RadioControlUnit,
    Railway,
    Ramp1m,
    Ramp2m,
    Ramp4m,
    RampWall1m,
    RampWall2m,
    RampWall4m,
    RampWall8m,
    RebarGun,
    Refinery,
    ReinforcedIronPlate,
    ReinforcedWindow,
    ResidualFuel,
    ResidualPlastic,
    ResidualRubber,
    ResourceWellExtractor,
    ResourceWellPressurizer,
    Rifle,
    RifleAmmo,
    RoadBarrier,
    Roof1m,
    Roof2m,
    Roof4m,
    RoofFlat,
    Rotor,
    Rubber,
    Screw,
    ShatterRebar,
    SideDoorWall,
    Silica,
    SingleWindow,
    SmallBillboard,
    SmallConcretePillar,
    SmallFramePillar,
    SmallMetalPillar,
    SmallPillarSupport,
    SmartPlating,
    SmartSplitter,
    Smelter,
    SmokelessPowder,
    Snowman,
    SolidBiofuel,
    SpaceElevator,
    SpitterProtein,
    SquareSign05m,
    SquareSign1m,
    SquareSign2m,
    StackableConveyorPole,
    StackableHypertubeSupport,
    StackablePipelineSupport,
    StairsLeft,
    StairsRight,
    Stator,
    SteelBeam,
    SteelIngot,
    SteelPipe,
    StingerProtein,
    StorageContainer,
    StreetLight,
    StunRebar,
    SulfuricAcid,
    Supercomputer,
    TheHUB,
    TherapeuticInhaler,
    ThermalPropulsionRocket,
    TiltedConcaveWall4m,
    TiltedConcaveWall8m,
    TiltedCornerWall4m,
    TiltedCornerWall8m,
    TiltedWall4m,
    TiltedWall8m,
    Tractor,
    TrainStation,
    Truck,
    TruckStation,
    TurboMotor,
    TurboRifleAmmo,
    Turbofuel,
    UJellyLandingPad,
    UnpackageAluminaSolution,
    UnpackageFuel,
    UnpackageHeavyOilResidue,
    UnpackageLiquidBiofuel,
    UnpackageNitricAcid,
    UnpackageNitrogenGas,
    UnpackageOil,
    UnpackageSulfuricAcid,
    UnpackageTurbofuel,
    UnpackageWater,
    UpCornerRamp1m,
    UpCornerRamp2m,
    UpCornerRamp4m,
    UraniumFuelRod,
    Valve,
    VersatileFramework,
    VitaminInhaler,
    Wall1a,
    WallMountedFloodLight,
    WallOutletMk1,
    WallOutletMk2,
    WallOutletMk3,
    WaterExtractor,
    Wire,
    XenoBasher,
    XenoZapper,
    Zipline,
}

pub fn item_name(i: Item) -> &'static str {
    match i {
        Item::AILimiter => "AI Limiter",
        Item::AdaptiveControlUnit => "Adaptive Control Unit",
        Item::AlcladAluminumSheet => "Alclad Aluminum Sheet",
        Item::AlienDNACapsule => "Alien DNA Capsule",
        Item::AlienProtein => "Alien Protein",
        Item::AluminaSolution => "Alumina Solution",
        Item::AluminumCasing => "Aluminum Casing",
        Item::AluminumIngot => "Aluminum Ingot",
        Item::AluminumScrap => "Aluminum Scrap",
        Item::AssemblyDirectorSystem => "Assembly Director System",
        Item::AutomatedWiring => "Automated Wiring",
        Item::BaconAgaric => "Bacon Agaric",
        Item::Battery => "Battery",
        Item::Bauxite => "Bauxite",
        Item::Beacon => "Beacon",
        Item::BerylNut => "Beryl Nut",
        Item::Biomass => "Biomass",
        Item::BlackPowder => "Black Powder",
        Item::BladeRunners => "Blade Runners",
        Item::BluePowerSlug => "Blue Power Slug",
        Item::Cable => "Cable",
        Item::CateriumIngot => "Caterium Ingot",
        Item::CateriumOre => "Caterium Ore",
        Item::Chainsaw => "Chainsaw",
        Item::CircuitBoard => "Circuit Board",
        Item::ClusterNobelisk => "Cluster Nobelisk",
        Item::Coal => "Coal",
        Item::ColorCartridge => "Color Cartridge",
        Item::CompactedCoal => "Compacted Coal",
        Item::Computer => "Computer",
        Item::Concrete => "Concrete",
        Item::CoolingSystem => "Cooling System",
        Item::CopperIngot => "Copper Ingot",
        Item::CopperOre => "Copper Ore",
        Item::CopperPowder => "Copper Powder",
        Item::CopperSheet => "Copper Sheet",
        Item::CrudeOil => "Crude Oil",
        Item::CrystalOscillator => "Crystal Oscillator",
        Item::ElectromagneticControlRod => "Electromagnetic Control Rod",
        Item::EmptyCanister => "Empty Canister",
        Item::EmptyFluidTank => "Empty Fluid Tank",
        Item::EncasedIndustrialBeam => "Encased Industrial Beam",
        Item::EncasedPlutoniumCell => "Encased Plutonium Cell",
        Item::EncasedUraniumCell => "Encased Uranium Cell",
        Item::ExplosiveRebar => "Explosive Rebar",
        Item::Fabric => "Fabric",
        Item::FactoryCart => "Factory Cart™",
        Item::FlowerPetals => "Flower Petals",
        Item::Fuel => "Fuel",
        Item::FusedModularFrame => "Fused Modular Frame",
        Item::GasFilter => "Gas Filter",
        Item::GasMask => "Gas Mask",
        Item::GasNobelisk => "Gas Nobelisk",
        Item::GoldenFactoryCart => "Golden Factory Cart™",
        Item::HUBParts => "HUB Parts",
        Item::HatcherRemains => "Hatcher Remains",
        Item::HazmatSuit => "Hazmat Suit",
        Item::HeatSink => "Heat Sink",
        Item::HeavyModularFrame => "Heavy Modular Frame",
        Item::HeavyOilResidue => "Heavy Oil Residue",
        Item::HighSpeedConnector => "High-Speed Connector",
        Item::HogRemains => "Hog Remains",
        Item::HomingRifleAmmo => "Homing Rifle Ammo",
        Item::HoverPack => "Hover Pack",
        Item::IodineInfusedFilter => "Iodine Infused Filter",
        Item::IronIngot => "Iron Ingot",
        Item::IronOre => "Iron Ore",
        Item::IronPlate => "Iron Plate",
        Item::IronRebar => "Iron Rebar",
        Item::IronRod => "Iron Rod",
        Item::Jetpack => "Jetpack",
        Item::Leaves => "Leaves",
        Item::Limestone => "Limestone",
        Item::LiquidBiofuel => "Liquid Biofuel",
        Item::MagneticFieldGenerator => "Magnetic Field Generator",
        Item::MedicinalInhaler => "Medicinal Inhaler",
        Item::ModularEngine => "Modular Engine",
        Item::ModularFrame => "Modular Frame",
        Item::Motor => "Motor",
        Item::Mycelia => "Mycelia",
        Item::NitricAcid => "Nitric Acid",
        Item::NitrogenGas => "Nitrogen Gas",
        Item::Nobelisk => "Nobelisk",
        Item::NobeliskDetonator => "Nobelisk Detonator",
        Item::NonFissileUranium => "Non-fissile Uranium",
        Item::NuclearPasta => "Nuclear Pasta",
        Item::NukeNobelisk => "Nuke Nobelisk",
        Item::ObjectScanner => "Object Scanner",
        Item::PackagedAluminaSolution => "Packaged Alumina Solution",
        Item::PackagedFuel => "Packaged Fuel",
        Item::PackagedHeavyOilResidue => "Packaged Heavy Oil Residue",
        Item::PackagedLiquidBiofuel => "Packaged Liquid Biofuel",
        Item::PackagedNitricAcid => "Packaged Nitric Acid",
        Item::PackagedNitrogenGas => "Packaged Nitrogen Gas",
        Item::PackagedOil => "Packaged Oil",
        Item::PackagedSulfuricAcid => "Packaged Sulfuric Acid",
        Item::PackagedTurbofuel => "Packaged Turbofuel",
        Item::PackagedWater => "Packaged Water",
        Item::Paleberry => "Paleberry",
        Item::Parachute => "Parachute",
        Item::PetroleumCoke => "Petroleum Coke",
        Item::PlasmaSpitterRemains => "Plasma Spitter Remains",
        Item::Plastic => "Plastic",
        Item::PlutoniumFuelRod => "Plutonium Fuel Rod",
        Item::PlutoniumPellet => "Plutonium Pellet",
        Item::PlutoniumWaste => "Plutonium Waste",
        Item::PolymerResin => "Polymer Resin",
        Item::PortableMiner => "Portable Miner",
        Item::PowerShard => "Power Shard",
        Item::PressureConversionCube => "Pressure Conversion Cube",
        Item::PulseNobelisk => "Pulse Nobelisk",
        Item::PurplePowerSlug => "Purple Power Slug",
        Item::QuartzCrystal => "Quartz Crystal",
        Item::Quickwire => "Quickwire",
        Item::RadioControlUnit => "Radio Control Unit",
        Item::RawQuartz => "Raw Quartz",
        Item::RebarGun => "Rebar Gun",
        Item::ReinforcedIronPlate => "Reinforced Iron Plate",
        Item::Rifle => "Rifle",
        Item::RifleAmmo => "Rifle Ammo",
        Item::Rotor => "Rotor",
        Item::Rubber => "Rubber",
        Item::Screw => "Screw",
        Item::ShatterRebar => "Shatter Rebar",
        Item::Silica => "Silica",
        Item::SmartPlating => "Smart Plating",
        Item::SmokelessPowder => "Smokeless Powder",
        Item::SolidBiofuel => "Solid Biofuel",
        Item::Stator => "Stator",
        Item::SteelBeam => "Steel Beam",
        Item::SteelIngot => "Steel Ingot",
        Item::SteelPipe => "Steel Pipe",
        Item::StingerRemains => "Stinger Remains",
        Item::StunRebar => "Stun Rebar",
        Item::Sulfur => "Sulfur",
        Item::SulfuricAcid => "Sulfuric Acid",
        Item::Supercomputer => "Supercomputer",
        Item::ThermalPropulsionRocket => "Thermal Propulsion Rocket",
        Item::TurboMotor => "Turbo Motor",
        Item::TurboRifleAmmo => "Turbo Rifle Ammo",
        Item::Turbofuel => "Turbofuel",
        Item::Uranium => "Uranium",
        Item::UraniumFuelRod => "Uranium Fuel Rod",
        Item::UraniumWaste => "Uranium Waste",
        Item::VersatileFramework => "Versatile Framework",
        Item::Water => "Water",
        Item::Wire => "Wire",
        Item::Wood => "Wood",
        Item::XenoBasher => "Xeno-Basher",
        Item::XenoZapper => "Xeno-Zapper",
        Item::YellowPowerSlug => "Yellow Power Slug",
        Item::Zipline => "Zipline",
    }
}

pub fn item_by_name(name: &str) -> Option<Item> {
    match name {
        "AI Limiter" => Some(Item::AILimiter),
        "Adaptive Control Unit" => Some(Item::AdaptiveControlUnit),
        "Alclad Aluminum Sheet" => Some(Item::AlcladAluminumSheet),
        "Alien DNA Capsule" => Some(Item::AlienDNACapsule),
        "Alien Protein" => Some(Item::AlienProtein),
        "Alumina Solution" => Some(Item::AluminaSolution),
        "Aluminum Casing" => Some(Item::AluminumCasing),
        "Aluminum Ingot" => Some(Item::AluminumIngot),
        "Aluminum Scrap" => Some(Item::AluminumScrap),
        "Assembly Director System" => Some(Item::AssemblyDirectorSystem),
        "Automated Wiring" => Some(Item::AutomatedWiring),
        "Bacon Agaric" => Some(Item::BaconAgaric),
        "Battery" => Some(Item::Battery),
        "Bauxite" => Some(Item::Bauxite),
        "Beacon" => Some(Item::Beacon),
        "Beryl Nut" => Some(Item::BerylNut),
        "Biomass" => Some(Item::Biomass),
        "Black Powder" => Some(Item::BlackPowder),
        "Blade Runners" => Some(Item::BladeRunners),
        "Blue Power Slug" => Some(Item::BluePowerSlug),
        "Cable" => Some(Item::Cable),
        "Caterium Ingot" => Some(Item::CateriumIngot),
        "Caterium Ore" => Some(Item::CateriumOre),
        "Chainsaw" => Some(Item::Chainsaw),
        "Circuit Board" => Some(Item::CircuitBoard),
        "Cluster Nobelisk" => Some(Item::ClusterNobelisk),
        "Coal" => Some(Item::Coal),
        "Color Cartridge" => Some(Item::ColorCartridge),
        "Compacted Coal" => Some(Item::CompactedCoal),
        "Computer" => Some(Item::Computer),
        "Concrete" => Some(Item::Concrete),
        "Cooling System" => Some(Item::CoolingSystem),
        "Copper Ingot" => Some(Item::CopperIngot),
        "Copper Ore" => Some(Item::CopperOre),
        "Copper Powder" => Some(Item::CopperPowder),
        "Copper Sheet" => Some(Item::CopperSheet),
        "Crude Oil" => Some(Item::CrudeOil),
        "Crystal Oscillator" => Some(Item::CrystalOscillator),
        "Electromagnetic Control Rod" => Some(Item::ElectromagneticControlRod),
        "Empty Canister" => Some(Item::EmptyCanister),
        "Empty Fluid Tank" => Some(Item::EmptyFluidTank),
        "Encased Industrial Beam" => Some(Item::EncasedIndustrialBeam),
        "Encased Plutonium Cell" => Some(Item::EncasedPlutoniumCell),
        "Encased Uranium Cell" => Some(Item::EncasedUraniumCell),
        "Explosive Rebar" => Some(Item::ExplosiveRebar),
        "Fabric" => Some(Item::Fabric),
        "Factory Cart™" => Some(Item::FactoryCart),
        "Flower Petals" => Some(Item::FlowerPetals),
        "Fuel" => Some(Item::Fuel),
        "Fused Modular Frame" => Some(Item::FusedModularFrame),
        "Gas Filter" => Some(Item::GasFilter),
        "Gas Mask" => Some(Item::GasMask),
        "Gas Nobelisk" => Some(Item::GasNobelisk),
        "Golden Factory Cart™" => Some(Item::GoldenFactoryCart),
        "HUB Parts" => Some(Item::HUBParts),
        "Hatcher Remains" => Some(Item::HatcherRemains),
        "Hazmat Suit" => Some(Item::HazmatSuit),
        "Heat Sink" => Some(Item::HeatSink),
        "Heavy Modular Frame" => Some(Item::HeavyModularFrame),
        "Heavy Oil Residue" => Some(Item::HeavyOilResidue),
        "High-Speed Connector" => Some(Item::HighSpeedConnector),
        "Hog Remains" => Some(Item::HogRemains),
        "Homing Rifle Ammo" => Some(Item::HomingRifleAmmo),
        "Hover Pack" => Some(Item::HoverPack),
        "Iodine Infused Filter" => Some(Item::IodineInfusedFilter),
        "Iron Ingot" => Some(Item::IronIngot),
        "Iron Ore" => Some(Item::IronOre),
        "Iron Plate" => Some(Item::IronPlate),
        "Iron Rebar" => Some(Item::IronRebar),
        "Iron Rod" => Some(Item::IronRod),
        "Jetpack" => Some(Item::Jetpack),
        "Leaves" => Some(Item::Leaves),
        "Limestone" => Some(Item::Limestone),
        "Liquid Biofuel" => Some(Item::LiquidBiofuel),
        "Magnetic Field Generator" => Some(Item::MagneticFieldGenerator),
        "Medicinal Inhaler" => Some(Item::MedicinalInhaler),
        "Modular Engine" => Some(Item::ModularEngine),
        "Modular Frame" => Some(Item::ModularFrame),
        "Motor" => Some(Item::Motor),
        "Mycelia" => Some(Item::Mycelia),
        "Nitric Acid" => Some(Item::NitricAcid),
        "Nitrogen Gas" => Some(Item::NitrogenGas),
        "Nobelisk" => Some(Item::Nobelisk),
        "Nobelisk Detonator" => Some(Item::NobeliskDetonator),
        "Non-fissile Uranium" => Some(Item::NonFissileUranium),
        "Nuclear Pasta" => Some(Item::NuclearPasta),
        "Nuke Nobelisk" => Some(Item::NukeNobelisk),
        "Object Scanner" => Some(Item::ObjectScanner),
        "Packaged Alumina Solution" => Some(Item::PackagedAluminaSolution),
        "Packaged Fuel" => Some(Item::PackagedFuel),
        "Packaged Heavy Oil Residue" => Some(Item::PackagedHeavyOilResidue),
        "Packaged Liquid Biofuel" => Some(Item::PackagedLiquidBiofuel),
        "Packaged Nitric Acid" => Some(Item::PackagedNitricAcid),
        "Packaged Nitrogen Gas" => Some(Item::PackagedNitrogenGas),
        "Packaged Oil" => Some(Item::PackagedOil),
        "Packaged Sulfuric Acid" => Some(Item::PackagedSulfuricAcid),
        "Packaged Turbofuel" => Some(Item::PackagedTurbofuel),
        "Packaged Water" => Some(Item::PackagedWater),
        "Paleberry" => Some(Item::Paleberry),
        "Parachute" => Some(Item::Parachute),
        "Petroleum Coke" => Some(Item::PetroleumCoke),
        "Plasma Spitter Remains" => Some(Item::PlasmaSpitterRemains),
        "Plastic" => Some(Item::Plastic),
        "Plutonium Fuel Rod" => Some(Item::PlutoniumFuelRod),
        "Plutonium Pellet" => Some(Item::PlutoniumPellet),
        "Plutonium Waste" => Some(Item::PlutoniumWaste),
        "Polymer Resin" => Some(Item::PolymerResin),
        "Portable Miner" => Some(Item::PortableMiner),
        "Power Shard" => Some(Item::PowerShard),
        "Pressure Conversion Cube" => Some(Item::PressureConversionCube),
        "Pulse Nobelisk" => Some(Item::PulseNobelisk),
        "Purple Power Slug" => Some(Item::PurplePowerSlug),
        "Quartz Crystal" => Some(Item::QuartzCrystal),
        "Quickwire" => Some(Item::Quickwire),
        "Radio Control Unit" => Some(Item::RadioControlUnit),
        "Raw Quartz" => Some(Item::RawQuartz),
        "Rebar Gun" => Some(Item::RebarGun),
        "Reinforced Iron Plate" => Some(Item::ReinforcedIronPlate),
        "Rifle" => Some(Item::Rifle),
        "Rifle Ammo" => Some(Item::RifleAmmo),
        "Rotor" => Some(Item::Rotor),
        "Rubber" => Some(Item::Rubber),
        "Screw" => Some(Item::Screw),
        "Shatter Rebar" => Some(Item::ShatterRebar),
        "Silica" => Some(Item::Silica),
        "Smart Plating" => Some(Item::SmartPlating),
        "Smokeless Powder" => Some(Item::SmokelessPowder),
        "Solid Biofuel" => Some(Item::SolidBiofuel),
        "Stator" => Some(Item::Stator),
        "Steel Beam" => Some(Item::SteelBeam),
        "Steel Ingot" => Some(Item::SteelIngot),
        "Steel Pipe" => Some(Item::SteelPipe),
        "Stinger Remains" => Some(Item::StingerRemains),
        "Stun Rebar" => Some(Item::StunRebar),
        "Sulfur" => Some(Item::Sulfur),
        "Sulfuric Acid" => Some(Item::SulfuricAcid),
        "Supercomputer" => Some(Item::Supercomputer),
        "Thermal Propulsion Rocket" => Some(Item::ThermalPropulsionRocket),
        "Turbo Motor" => Some(Item::TurboMotor),
        "Turbo Rifle Ammo" => Some(Item::TurboRifleAmmo),
        "Turbofuel" => Some(Item::Turbofuel),
        "Uranium" => Some(Item::Uranium),
        "Uranium Fuel Rod" => Some(Item::UraniumFuelRod),
        "Uranium Waste" => Some(Item::UraniumWaste),
        "Versatile Framework" => Some(Item::VersatileFramework),
        "Water" => Some(Item::Water),
        "Wire" => Some(Item::Wire),
        "Wood" => Some(Item::Wood),
        "Xeno-Basher" => Some(Item::XenoBasher),
        "Xeno-Zapper" => Some(Item::XenoZapper),
        "Yellow Power Slug" => Some(Item::YellowPowerSlug),
        "Zipline" => Some(Item::Zipline),
        _ => None,
    }
}

pub struct RecipeInfo {
    pub recipe: Recipe,
    pub name: &'static str,
    pub machine: crate::models::types::MachineType,
    pub inputs: &'static [(Item, f32)],
    pub outputs: &'static [(Item, f32)],
}

const IN_AILimiter: &[(Item, f32)] = &[
    (Item::CopperSheet, 25.000000),
    (Item::Quickwire, 100.000000),
];

const OUT_AILimiter: &[(Item, f32)] = &[(Item::AILimiter, 5.000000)];

const IN_AWESOMEShop: &[(Item, f32)] = &[
    (Item::Screw, 12000.000000),
    (Item::IronPlate, 600.000000),
    (Item::Cable, 600.000000),
];

const OUT_AWESOMEShop: &[(Item, f32)] = &[];

const IN_AWESOMESink: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 900.000000),
    (Item::Cable, 1800.000000),
    (Item::Concrete, 2700.000000),
];

const OUT_AWESOMESink: &[(Item, f32)] = &[];

const IN_AdaptiveControlUnit: &[(Item, f32)] = &[
    (Item::AutomatedWiring, 7.500000),
    (Item::CircuitBoard, 5.000000),
    (Item::HeavyModularFrame, 1.000000),
    (Item::Computer, 1.000000),
];

const OUT_AdaptiveControlUnit: &[(Item, f32)] = &[(Item::AdaptiveControlUnit, 1.000000)];

const IN_AlcladAluminumSheet: &[(Item, f32)] = &[
    (Item::AluminumIngot, 30.000000),
    (Item::CopperIngot, 10.000000),
];

const OUT_AlcladAluminumSheet: &[(Item, f32)] = &[(Item::AlcladAluminumSheet, 30.000000)];

const IN_AlienDNACapsule: &[(Item, f32)] = &[(Item::AlienProtein, 10.000000)];

const OUT_AlienDNACapsule: &[(Item, f32)] = &[(Item::AlienDNACapsule, 10.000000)];

const IN_AlternateAdheredIronPlate: &[(Item, f32)] =
    &[(Item::IronPlate, 11.250000), (Item::Rubber, 3.750000)];

const OUT_AlternateAdheredIronPlate: &[(Item, f32)] = &[(Item::ReinforcedIronPlate, 3.750000)];

const IN_AlternateAlcladCasing: &[(Item, f32)] = &[
    (Item::AluminumIngot, 150.000000),
    (Item::CopperIngot, 75.000000),
];

const OUT_AlternateAlcladCasing: &[(Item, f32)] = &[(Item::AluminumCasing, 112.500000)];

const IN_AlternateAutomatedMiner: &[(Item, f32)] = &[
    (Item::Motor, 1.000000),
    (Item::SteelPipe, 4.000000),
    (Item::IronRod, 4.000000),
    (Item::IronPlate, 2.000000),
];

const OUT_AlternateAutomatedMiner: &[(Item, f32)] = &[(Item::PortableMiner, 1.000000)];

const IN_AlternateAutomatedSpeedWiring: &[(Item, f32)] = &[
    (Item::Stator, 3.750000),
    (Item::Wire, 75.000000),
    (Item::HighSpeedConnector, 1.875000),
];

const OUT_AlternateAutomatedSpeedWiring: &[(Item, f32)] = &[(Item::AutomatedWiring, 7.500000)];

const IN_AlternateBiocoal: &[(Item, f32)] = &[(Item::Biomass, 37.500000)];

const OUT_AlternateBiocoal: &[(Item, f32)] = &[(Item::Coal, 45.000000)];

const IN_AlternateBoltedFrame: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 7.500000),
    (Item::Screw, 140.000000),
];

const OUT_AlternateBoltedFrame: &[(Item, f32)] = &[(Item::ModularFrame, 5.000000)];

const IN_AlternateBoltedIronPlate: &[(Item, f32)] =
    &[(Item::IronPlate, 90.000000), (Item::Screw, 250.000000)];

const OUT_AlternateBoltedIronPlate: &[(Item, f32)] = &[(Item::ReinforcedIronPlate, 15.000000)];

const IN_AlternateCastScrew: &[(Item, f32)] = &[(Item::IronIngot, 12.500000)];

const OUT_AlternateCastScrew: &[(Item, f32)] = &[(Item::Screw, 50.000000)];

const IN_AlternateCateriumCircuitBoard: &[(Item, f32)] =
    &[(Item::Plastic, 12.500000), (Item::Quickwire, 37.500000)];

const OUT_AlternateCateriumCircuitBoard: &[(Item, f32)] = &[(Item::CircuitBoard, 8.750000)];

const IN_AlternateCateriumComputer: &[(Item, f32)] = &[
    (Item::CircuitBoard, 26.250000),
    (Item::Quickwire, 105.000000),
    (Item::Rubber, 45.000000),
];

const OUT_AlternateCateriumComputer: &[(Item, f32)] = &[(Item::Computer, 3.750000)];

const IN_AlternateCateriumWire: &[(Item, f32)] = &[(Item::CateriumIngot, 15.000000)];

const OUT_AlternateCateriumWire: &[(Item, f32)] = &[(Item::Wire, 120.000000)];

const IN_AlternateCharcoal: &[(Item, f32)] = &[(Item::Wood, 15.000000)];

const OUT_AlternateCharcoal: &[(Item, f32)] = &[(Item::Coal, 150.000000)];

const IN_AlternateCheapSilica: &[(Item, f32)] =
    &[(Item::RawQuartz, 11.250000), (Item::Limestone, 18.750000)];

const OUT_AlternateCheapSilica: &[(Item, f32)] = &[(Item::Silica, 26.250000)];

const IN_AlternateClassicBattery: &[(Item, f32)] = &[
    (Item::Sulfur, 45.000000),
    (Item::AlcladAluminumSheet, 52.500000),
    (Item::Plastic, 60.000000),
    (Item::Wire, 90.000000),
];

const OUT_AlternateClassicBattery: &[(Item, f32)] = &[(Item::Battery, 30.000000)];

const IN_AlternateCoatedCable: &[(Item, f32)] =
    &[(Item::Wire, 37.500000), (Item::HeavyOilResidue, 15.000000)];

const OUT_AlternateCoatedCable: &[(Item, f32)] = &[(Item::Cable, 67.500000)];

const IN_AlternateCoatedIronCanister: &[(Item, f32)] =
    &[(Item::IronPlate, 30.000000), (Item::CopperSheet, 15.000000)];

const OUT_AlternateCoatedIronCanister: &[(Item, f32)] = &[(Item::EmptyCanister, 60.000000)];

const IN_AlternateCoatedIronPlate: &[(Item, f32)] =
    &[(Item::IronIngot, 50.000000), (Item::Plastic, 10.000000)];

const OUT_AlternateCoatedIronPlate: &[(Item, f32)] = &[(Item::IronPlate, 75.000000)];

const IN_AlternateCokeSteelIngot: &[(Item, f32)] =
    &[(Item::IronOre, 75.000000), (Item::PetroleumCoke, 75.000000)];

const OUT_AlternateCokeSteelIngot: &[(Item, f32)] = &[(Item::SteelIngot, 100.000000)];

const IN_AlternateCompactedCoal: &[(Item, f32)] =
    &[(Item::Coal, 25.000000), (Item::Sulfur, 25.000000)];

const OUT_AlternateCompactedCoal: &[(Item, f32)] = &[(Item::CompactedCoal, 25.000000)];

const IN_AlternateCompactedSteelIngot: &[(Item, f32)] =
    &[(Item::IronOre, 22.500000), (Item::CompactedCoal, 11.250000)];

const OUT_AlternateCompactedSteelIngot: &[(Item, f32)] = &[(Item::SteelIngot, 37.500000)];

const IN_AlternateCoolingDevice: &[(Item, f32)] = &[
    (Item::HeatSink, 9.375000),
    (Item::Motor, 1.875000),
    (Item::NitrogenGas, 45.000000),
];

const OUT_AlternateCoolingDevice: &[(Item, f32)] = &[(Item::CoolingSystem, 3.750000)];

const IN_AlternateCopperAlloyIngot: &[(Item, f32)] =
    &[(Item::CopperOre, 50.000000), (Item::IronOre, 25.000000)];

const OUT_AlternateCopperAlloyIngot: &[(Item, f32)] = &[(Item::CopperIngot, 100.000000)];

const IN_AlternateCopperRotor: &[(Item, f32)] =
    &[(Item::CopperSheet, 22.500000), (Item::Screw, 195.000000)];

const OUT_AlternateCopperRotor: &[(Item, f32)] = &[(Item::Rotor, 11.250000)];

const IN_AlternateCrystalBeacon: &[(Item, f32)] = &[
    (Item::SteelBeam, 2.000000),
    (Item::SteelPipe, 8.000000),
    (Item::CrystalOscillator, 0.500000),
];

const OUT_AlternateCrystalBeacon: &[(Item, f32)] = &[(Item::Beacon, 10.000000)];

const IN_AlternateCrystalComputer: &[(Item, f32)] = &[
    (Item::CircuitBoard, 7.500000),
    (Item::CrystalOscillator, 2.812500),
];

const OUT_AlternateCrystalComputer: &[(Item, f32)] = &[(Item::Computer, 2.812500)];

const IN_AlternateDilutedFuel: &[(Item, f32)] = &[
    (Item::HeavyOilResidue, 50.000000),
    (Item::Water, 100.000000),
];

const OUT_AlternateDilutedFuel: &[(Item, f32)] = &[(Item::Fuel, 100.000000)];

const IN_AlternateDilutedPackagedFuel: &[(Item, f32)] = &[
    (Item::HeavyOilResidue, 30.000000),
    (Item::PackagedWater, 60.000000),
];

const OUT_AlternateDilutedPackagedFuel: &[(Item, f32)] = &[(Item::PackagedFuel, 60.000000)];

const IN_AlternateElectricMotor: &[(Item, f32)] = &[
    (Item::ElectromagneticControlRod, 3.750000),
    (Item::Rotor, 7.500000),
];

const OUT_AlternateElectricMotor: &[(Item, f32)] = &[(Item::Motor, 7.500000)];

const IN_AlternateElectrodeAluminumScrap: &[(Item, f32)] = &[
    (Item::AluminaSolution, 180.000000),
    (Item::PetroleumCoke, 60.000000),
];

const OUT_AlternateElectrodeAluminumScrap: &[(Item, f32)] =
    &[(Item::AluminumScrap, 300.000000), (Item::Water, 105.000000)];

const IN_AlternateElectrodeCircuitBoard: &[(Item, f32)] =
    &[(Item::Rubber, 30.000000), (Item::PetroleumCoke, 45.000000)];

const OUT_AlternateElectrodeCircuitBoard: &[(Item, f32)] = &[(Item::CircuitBoard, 5.000000)];

const IN_AlternateElectromagneticConnectionRod: &[(Item, f32)] = &[
    (Item::Stator, 8.000000),
    (Item::HighSpeedConnector, 4.000000),
];

const OUT_AlternateElectromagneticConnectionRod: &[(Item, f32)] =
    &[(Item::ElectromagneticControlRod, 8.000000)];

const IN_AlternateEncasedIndustrialPipe: &[(Item, f32)] =
    &[(Item::SteelPipe, 28.000000), (Item::Concrete, 20.000000)];

const OUT_AlternateEncasedIndustrialPipe: &[(Item, f32)] =
    &[(Item::EncasedIndustrialBeam, 4.000000)];

const IN_AlternateFertileUranium: &[(Item, f32)] = &[
    (Item::Uranium, 25.000000),
    (Item::UraniumWaste, 25.000000),
    (Item::NitricAcid, 15.000000),
    (Item::SulfuricAcid, 25.000000),
];

const OUT_AlternateFertileUranium: &[(Item, f32)] = &[
    (Item::NonFissileUranium, 100.000000),
    (Item::Water, 40.000000),
];

const IN_AlternateFineBlackPowder: &[(Item, f32)] =
    &[(Item::Sulfur, 7.500000), (Item::CompactedCoal, 3.750000)];

const OUT_AlternateFineBlackPowder: &[(Item, f32)] = &[(Item::BlackPowder, 15.000000)];

const IN_AlternateFineConcrete: &[(Item, f32)] =
    &[(Item::Silica, 7.500000), (Item::Limestone, 30.000000)];

const OUT_AlternateFineConcrete: &[(Item, f32)] = &[(Item::Concrete, 25.000000)];

const IN_AlternateFlexibleFramework: &[(Item, f32)] = &[
    (Item::ModularFrame, 3.750000),
    (Item::SteelBeam, 22.500000),
    (Item::Rubber, 30.000000),
];

const OUT_AlternateFlexibleFramework: &[(Item, f32)] = &[(Item::VersatileFramework, 7.500000)];

const IN_AlternateFusedQuickwire: &[(Item, f32)] = &[
    (Item::CateriumIngot, 7.500000),
    (Item::CopperIngot, 37.500000),
];

const OUT_AlternateFusedQuickwire: &[(Item, f32)] = &[(Item::Quickwire, 90.000000)];

const IN_AlternateFusedWire: &[(Item, f32)] = &[
    (Item::CopperIngot, 12.000000),
    (Item::CateriumIngot, 3.000000),
];

const OUT_AlternateFusedWire: &[(Item, f32)] = &[(Item::Wire, 90.000000)];

const IN_AlternateHeatExchanger: &[(Item, f32)] =
    &[(Item::AluminumCasing, 30.000000), (Item::Rubber, 30.000000)];

const OUT_AlternateHeatExchanger: &[(Item, f32)] = &[(Item::HeatSink, 10.000000)];

const IN_AlternateHeatFusedFrame: &[(Item, f32)] = &[
    (Item::HeavyModularFrame, 3.000000),
    (Item::AluminumIngot, 150.000000),
    (Item::NitricAcid, 24.000000),
    (Item::Fuel, 30.000000),
];

const OUT_AlternateHeatFusedFrame: &[(Item, f32)] = &[(Item::FusedModularFrame, 3.000000)];

const IN_AlternateHeavyEncasedFrame: &[(Item, f32)] = &[
    (Item::ModularFrame, 7.500000),
    (Item::EncasedIndustrialBeam, 9.375000),
    (Item::SteelPipe, 33.750000),
    (Item::Concrete, 20.625000),
];

const OUT_AlternateHeavyEncasedFrame: &[(Item, f32)] = &[(Item::HeavyModularFrame, 2.812500)];

const IN_AlternateHeavyFlexibleFrame: &[(Item, f32)] = &[
    (Item::ModularFrame, 18.750000),
    (Item::EncasedIndustrialBeam, 11.250000),
    (Item::Rubber, 75.000000),
    (Item::Screw, 390.000000),
];

const OUT_AlternateHeavyFlexibleFrame: &[(Item, f32)] = &[(Item::HeavyModularFrame, 3.750000)];

const IN_AlternateHeavyOilResidue: &[(Item, f32)] = &[(Item::CrudeOil, 30.000000)];

const OUT_AlternateHeavyOilResidue: &[(Item, f32)] = &[
    (Item::HeavyOilResidue, 40.000000),
    (Item::PolymerResin, 20.000000),
];

const IN_AlternateInfusedUraniumCell: &[(Item, f32)] = &[
    (Item::Uranium, 25.000000),
    (Item::Silica, 15.000000),
    (Item::Sulfur, 25.000000),
    (Item::Quickwire, 75.000000),
];

const OUT_AlternateInfusedUraniumCell: &[(Item, f32)] = &[(Item::EncasedUraniumCell, 20.000000)];

const IN_AlternateInstantPlutoniumCell: &[(Item, f32)] = &[
    (Item::NonFissileUranium, 75.000000),
    (Item::AluminumCasing, 10.000000),
];

const OUT_AlternateInstantPlutoniumCell: &[(Item, f32)] =
    &[(Item::EncasedPlutoniumCell, 10.000000)];

const IN_AlternateInstantScrap: &[(Item, f32)] = &[
    (Item::Bauxite, 150.000000),
    (Item::Coal, 100.000000),
    (Item::SulfuricAcid, 50.000000),
    (Item::Water, 60.000000),
];

const OUT_AlternateInstantScrap: &[(Item, f32)] =
    &[(Item::AluminumScrap, 300.000000), (Item::Water, 50.000000)];

const IN_AlternateInsulatedCable: &[(Item, f32)] =
    &[(Item::Wire, 45.000000), (Item::Rubber, 30.000000)];

const OUT_AlternateInsulatedCable: &[(Item, f32)] = &[(Item::Cable, 100.000000)];

const IN_AlternateInsulatedCrystalOscillator: &[(Item, f32)] = &[
    (Item::QuartzCrystal, 18.750000),
    (Item::Rubber, 13.125000),
    (Item::AILimiter, 1.875000),
];

const OUT_AlternateInsulatedCrystalOscillator: &[(Item, f32)] =
    &[(Item::CrystalOscillator, 1.875000)];

const IN_AlternateIronAlloyIngot: &[(Item, f32)] =
    &[(Item::IronOre, 20.000000), (Item::CopperOre, 20.000000)];

const OUT_AlternateIronAlloyIngot: &[(Item, f32)] = &[(Item::IronIngot, 50.000000)];

const IN_AlternateIronWire: &[(Item, f32)] = &[(Item::IronIngot, 12.500000)];

const OUT_AlternateIronWire: &[(Item, f32)] = &[(Item::Wire, 22.500000)];

const IN_AlternateOCSupercomputer: &[(Item, f32)] = &[
    (Item::RadioControlUnit, 9.000000),
    (Item::CoolingSystem, 9.000000),
];

const OUT_AlternateOCSupercomputer: &[(Item, f32)] = &[(Item::Supercomputer, 3.000000)];

const IN_AlternatePlasticSmartPlating: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 2.500000),
    (Item::Rotor, 2.500000),
    (Item::Plastic, 7.500000),
];

const OUT_AlternatePlasticSmartPlating: &[(Item, f32)] = &[(Item::SmartPlating, 5.000000)];

const IN_AlternatePlutoniumFuelUnit: &[(Item, f32)] = &[
    (Item::EncasedPlutoniumCell, 10.000000),
    (Item::PressureConversionCube, 0.500000),
];

const OUT_AlternatePlutoniumFuelUnit: &[(Item, f32)] = &[(Item::PlutoniumFuelRod, 0.500000)];

const IN_AlternatePolyesterFabric: &[(Item, f32)] =
    &[(Item::PolymerResin, 30.000000), (Item::Water, 30.000000)];

const OUT_AlternatePolyesterFabric: &[(Item, f32)] = &[(Item::Fabric, 30.000000)];

const IN_AlternatePolymerResin: &[(Item, f32)] = &[(Item::CrudeOil, 60.000000)];

const OUT_AlternatePolymerResin: &[(Item, f32)] = &[
    (Item::PolymerResin, 130.000000),
    (Item::HeavyOilResidue, 20.000000),
];

const IN_AlternatePureAluminumIngot: &[(Item, f32)] = &[(Item::AluminumScrap, 60.000000)];

const OUT_AlternatePureAluminumIngot: &[(Item, f32)] = &[(Item::AluminumIngot, 30.000000)];

const IN_AlternatePureCateriumIngot: &[(Item, f32)] =
    &[(Item::CateriumOre, 24.000000), (Item::Water, 24.000000)];

const OUT_AlternatePureCateriumIngot: &[(Item, f32)] = &[(Item::CateriumIngot, 12.000000)];

const IN_AlternatePureCopperIngot: &[(Item, f32)] =
    &[(Item::CopperOre, 15.000000), (Item::Water, 10.000000)];

const OUT_AlternatePureCopperIngot: &[(Item, f32)] = &[(Item::CopperIngot, 37.500000)];

const IN_AlternatePureIronIngot: &[(Item, f32)] =
    &[(Item::IronOre, 35.000000), (Item::Water, 20.000000)];

const OUT_AlternatePureIronIngot: &[(Item, f32)] = &[(Item::IronIngot, 65.000000)];

const IN_AlternatePureQuartzCrystal: &[(Item, f32)] =
    &[(Item::RawQuartz, 67.500000), (Item::Water, 37.500000)];

const OUT_AlternatePureQuartzCrystal: &[(Item, f32)] = &[(Item::QuartzCrystal, 52.500000)];

const IN_AlternateQuickwireCable: &[(Item, f32)] =
    &[(Item::Quickwire, 7.500000), (Item::Rubber, 5.000000)];

const OUT_AlternateQuickwireCable: &[(Item, f32)] = &[(Item::Cable, 27.500000)];

const IN_AlternateQuickwireStator: &[(Item, f32)] =
    &[(Item::SteelPipe, 16.000000), (Item::Quickwire, 60.000000)];

const OUT_AlternateQuickwireStator: &[(Item, f32)] = &[(Item::Stator, 8.000000)];

const IN_AlternateRadioConnectionUnit: &[(Item, f32)] = &[
    (Item::HeatSink, 15.000000),
    (Item::HighSpeedConnector, 7.500000),
    (Item::QuartzCrystal, 45.000000),
];

const OUT_AlternateRadioConnectionUnit: &[(Item, f32)] = &[(Item::RadioControlUnit, 3.750000)];

const IN_AlternateRadioControlSystem: &[(Item, f32)] = &[
    (Item::CrystalOscillator, 1.500000),
    (Item::CircuitBoard, 15.000000),
    (Item::AluminumCasing, 90.000000),
    (Item::Rubber, 45.000000),
];

const OUT_AlternateRadioControlSystem: &[(Item, f32)] = &[(Item::RadioControlUnit, 4.500000)];

const IN_AlternateRecycledPlastic: &[(Item, f32)] =
    &[(Item::Rubber, 30.000000), (Item::Fuel, 30.000000)];

const OUT_AlternateRecycledPlastic: &[(Item, f32)] = &[(Item::Plastic, 60.000000)];

const IN_AlternateRecycledRubber: &[(Item, f32)] =
    &[(Item::Plastic, 30.000000), (Item::Fuel, 30.000000)];

const OUT_AlternateRecycledRubber: &[(Item, f32)] = &[(Item::Rubber, 60.000000)];

const IN_AlternateRigourMotor: &[(Item, f32)] = &[
    (Item::Rotor, 3.750000),
    (Item::Stator, 3.750000),
    (Item::CrystalOscillator, 1.250000),
];

const OUT_AlternateRigourMotor: &[(Item, f32)] = &[(Item::Motor, 7.500000)];

const IN_AlternateRubberConcrete: &[(Item, f32)] =
    &[(Item::Limestone, 50.000000), (Item::Rubber, 10.000000)];

const OUT_AlternateRubberConcrete: &[(Item, f32)] = &[(Item::Concrete, 45.000000)];

const IN_AlternateSiliconCircuitBoard: &[(Item, f32)] =
    &[(Item::CopperSheet, 27.500000), (Item::Silica, 27.500000)];

const OUT_AlternateSiliconCircuitBoard: &[(Item, f32)] = &[(Item::CircuitBoard, 12.500000)];

const IN_AlternateSiliconHighSpeedConnector: &[(Item, f32)] = &[
    (Item::Quickwire, 90.000000),
    (Item::Silica, 37.500000),
    (Item::CircuitBoard, 3.000000),
];

const OUT_AlternateSiliconHighSpeedConnector: &[(Item, f32)] =
    &[(Item::HighSpeedConnector, 3.000000)];

const IN_AlternateSloppyAlumina: &[(Item, f32)] =
    &[(Item::Bauxite, 200.000000), (Item::Water, 200.000000)];

const OUT_AlternateSloppyAlumina: &[(Item, f32)] = &[(Item::AluminaSolution, 240.000000)];

const IN_AlternateSolidSteelIngot: &[(Item, f32)] =
    &[(Item::IronIngot, 40.000000), (Item::Coal, 40.000000)];

const OUT_AlternateSolidSteelIngot: &[(Item, f32)] = &[(Item::SteelIngot, 60.000000)];

const IN_AlternateSteamedCopperSheet: &[(Item, f32)] =
    &[(Item::CopperIngot, 22.500000), (Item::Water, 22.500000)];

const OUT_AlternateSteamedCopperSheet: &[(Item, f32)] = &[(Item::CopperSheet, 22.500000)];

const IN_AlternateSteelCanister: &[(Item, f32)] = &[(Item::SteelIngot, 60.000000)];

const OUT_AlternateSteelCanister: &[(Item, f32)] = &[(Item::EmptyCanister, 40.000000)];

const IN_AlternateSteelCoatedPlate: &[(Item, f32)] =
    &[(Item::SteelIngot, 7.500000), (Item::Plastic, 5.000000)];

const OUT_AlternateSteelCoatedPlate: &[(Item, f32)] = &[(Item::IronPlate, 45.000000)];

const IN_AlternateSteelRod: &[(Item, f32)] = &[(Item::SteelIngot, 12.000000)];

const OUT_AlternateSteelRod: &[(Item, f32)] = &[(Item::IronRod, 48.000000)];

const IN_AlternateSteelRotor: &[(Item, f32)] =
    &[(Item::SteelPipe, 10.000000), (Item::Wire, 30.000000)];

const OUT_AlternateSteelRotor: &[(Item, f32)] = &[(Item::Rotor, 5.000000)];

const IN_AlternateSteelScrew: &[(Item, f32)] = &[(Item::SteelBeam, 5.000000)];

const OUT_AlternateSteelScrew: &[(Item, f32)] = &[(Item::Screw, 260.000000)];

const IN_AlternateSteeledFrame: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 2.000000),
    (Item::SteelPipe, 10.000000),
];

const OUT_AlternateSteeledFrame: &[(Item, f32)] = &[(Item::ModularFrame, 3.000000)];

const IN_AlternateStitchedIronPlate: &[(Item, f32)] =
    &[(Item::IronPlate, 18.750000), (Item::Wire, 37.500000)];

const OUT_AlternateStitchedIronPlate: &[(Item, f32)] = &[(Item::ReinforcedIronPlate, 5.625000)];

const IN_AlternateSuperStateComputer: &[(Item, f32)] = &[
    (Item::Computer, 3.600000),
    (Item::ElectromagneticControlRod, 2.400000),
    (Item::Battery, 24.000000),
    (Item::Wire, 54.000000),
];

const OUT_AlternateSuperStateComputer: &[(Item, f32)] = &[(Item::Supercomputer, 2.400000)];

const IN_AlternateTurboBlendFuel: &[(Item, f32)] = &[
    (Item::Fuel, 15.000000),
    (Item::HeavyOilResidue, 30.000000),
    (Item::Sulfur, 22.500000),
    (Item::PetroleumCoke, 22.500000),
];

const OUT_AlternateTurboBlendFuel: &[(Item, f32)] = &[(Item::Turbofuel, 45.000000)];

const IN_AlternateTurboElectricMotor: &[(Item, f32)] = &[
    (Item::Motor, 6.562500),
    (Item::RadioControlUnit, 8.437500),
    (Item::ElectromagneticControlRod, 4.687500),
    (Item::Rotor, 6.562500),
];

const OUT_AlternateTurboElectricMotor: &[(Item, f32)] = &[(Item::TurboMotor, 2.812500)];

const IN_AlternateTurboHeavyFuel: &[(Item, f32)] = &[
    (Item::HeavyOilResidue, 37.500000),
    (Item::CompactedCoal, 30.000000),
];

const OUT_AlternateTurboHeavyFuel: &[(Item, f32)] = &[(Item::Turbofuel, 30.000000)];

const IN_AlternateTurboPressureMotor: &[(Item, f32)] = &[
    (Item::Motor, 7.500000),
    (Item::PressureConversionCube, 1.875000),
    (Item::PackagedNitrogenGas, 45.000000),
    (Item::Stator, 15.000000),
];

const OUT_AlternateTurboPressureMotor: &[(Item, f32)] = &[(Item::TurboMotor, 3.750000)];

const IN_AlternateUraniumFuelUnit: &[(Item, f32)] = &[
    (Item::EncasedUraniumCell, 20.000000),
    (Item::ElectromagneticControlRod, 2.000000),
    (Item::CrystalOscillator, 0.600000),
    (Item::Beacon, 1.200000),
];

const OUT_AlternateUraniumFuelUnit: &[(Item, f32)] = &[(Item::UraniumFuelRod, 0.600000)];

const IN_AlternateWetConcrete: &[(Item, f32)] =
    &[(Item::Limestone, 120.000000), (Item::Water, 100.000000)];

const OUT_AlternateWetConcrete: &[(Item, f32)] = &[(Item::Concrete, 80.000000)];

const IN_AluminaSolution: &[(Item, f32)] =
    &[(Item::Bauxite, 120.000000), (Item::Water, 180.000000)];

const OUT_AluminaSolution: &[(Item, f32)] = &[
    (Item::AluminaSolution, 120.000000),
    (Item::Silica, 50.000000),
];

const IN_AluminumCasing: &[(Item, f32)] = &[(Item::AluminumIngot, 90.000000)];

const OUT_AluminumCasing: &[(Item, f32)] = &[(Item::AluminumCasing, 60.000000)];

const IN_AluminumIngot: &[(Item, f32)] =
    &[(Item::AluminumScrap, 90.000000), (Item::Silica, 75.000000)];

const OUT_AluminumIngot: &[(Item, f32)] = &[(Item::AluminumIngot, 60.000000)];

const IN_AluminumScrap: &[(Item, f32)] = &[
    (Item::AluminaSolution, 240.000000),
    (Item::Coal, 120.000000),
];

const OUT_AluminumScrap: &[(Item, f32)] =
    &[(Item::AluminumScrap, 360.000000), (Item::Water, 120.000000)];

const IN_Assembler: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 480.000000),
    (Item::Rotor, 240.000000),
    (Item::Cable, 600.000000),
];

const OUT_Assembler: &[(Item, f32)] = &[];

const IN_AssemblyDirectorSystem: &[(Item, f32)] = &[
    (Item::AdaptiveControlUnit, 1.500000),
    (Item::Supercomputer, 0.750000),
];

const OUT_AssemblyDirectorSystem: &[(Item, f32)] = &[(Item::AssemblyDirectorSystem, 0.750000)];

const IN_AutomatedGate: &[(Item, f32)] = &[(Item::IronPlate, 360.000000)];

const OUT_AutomatedGate: &[(Item, f32)] = &[];

const IN_AutomatedWiring: &[(Item, f32)] = &[(Item::Stator, 2.500000), (Item::Cable, 50.000000)];

const OUT_AutomatedWiring: &[(Item, f32)] = &[(Item::AutomatedWiring, 2.500000)];

const IN_BasicWall1m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_BasicWall1m: &[(Item, f32)] = &[];

const IN_BasicWall4m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_BasicWall4m: &[(Item, f32)] = &[];

const IN_Battery: &[(Item, f32)] = &[
    (Item::SulfuricAcid, 50.000000),
    (Item::AluminaSolution, 40.000000),
    (Item::AluminumCasing, 20.000000),
];

const OUT_Battery: &[(Item, f32)] = &[(Item::Battery, 20.000000), (Item::Water, 30.000000)];

const IN_Beacon: &[(Item, f32)] = &[
    (Item::IronPlate, 22.500000),
    (Item::IronRod, 7.500000),
    (Item::Wire, 112.500000),
    (Item::Cable, 15.000000),
];

const OUT_Beacon: &[(Item, f32)] = &[(Item::Beacon, 7.500000)];

const IN_BeamConnector: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_BeamConnector: &[(Item, f32)] = &[];

const IN_BeamConnectorDouble: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_BeamConnectorDouble: &[(Item, f32)] = &[];

const IN_BeamSupport: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_BeamSupport: &[(Item, f32)] = &[];

const IN_BigConcretePillar: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_BigConcretePillar: &[(Item, f32)] = &[];

const IN_BigFramePillar: &[(Item, f32)] = &[(Item::SteelBeam, 300.000000)];

const OUT_BigFramePillar: &[(Item, f32)] = &[];

const IN_BigMetalPillar: &[(Item, f32)] =
    &[(Item::IronPlate, 120.000000), (Item::Concrete, 180.000000)];

const OUT_BigMetalPillar: &[(Item, f32)] = &[];

const IN_BigPillarSupport: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_BigPillarSupport: &[(Item, f32)] = &[];

const IN_BiomassAlienProtein: &[(Item, f32)] = &[(Item::AlienProtein, 15.000000)];

const OUT_BiomassAlienProtein: &[(Item, f32)] = &[(Item::Biomass, 1500.000000)];

const IN_BiomassBurner: &[(Item, f32)] = &[
    (Item::IronPlate, 900.000000),
    (Item::IronRod, 900.000000),
    (Item::Wire, 1500.000000),
];

const OUT_BiomassBurner: &[(Item, f32)] = &[];

const IN_BiomassLeaves: &[(Item, f32)] = &[(Item::Leaves, 120.000000)];

const OUT_BiomassLeaves: &[(Item, f32)] = &[(Item::Biomass, 60.000000)];

const IN_BiomassMycelia: &[(Item, f32)] = &[(Item::Mycelia, 15.000000)];

const OUT_BiomassMycelia: &[(Item, f32)] = &[(Item::Biomass, 150.000000)];

const IN_BiomassWood: &[(Item, f32)] = &[(Item::Wood, 60.000000)];

const OUT_BiomassWood: &[(Item, f32)] = &[(Item::Biomass, 300.000000)];

const IN_BlackPowder: &[(Item, f32)] = &[(Item::Coal, 15.000000), (Item::Sulfur, 15.000000)];

const OUT_BlackPowder: &[(Item, f32)] = &[(Item::BlackPowder, 30.000000)];

const IN_BladeRunners: &[(Item, f32)] = &[
    (Item::Silica, 20.000000),
    (Item::ModularFrame, 3.000000),
    (Item::Rotor, 3.000000),
];

const OUT_BladeRunners: &[(Item, f32)] = &[(Item::BladeRunners, 1.000000)];

const IN_Blender: &[(Item, f32)] = &[
    (Item::Motor, 1200.000000),
    (Item::HeavyModularFrame, 600.000000),
    (Item::AluminumCasing, 3000.000000),
    (Item::RadioControlUnit, 300.000000),
];

const OUT_Blender: &[(Item, f32)] = &[];

const IN_BlockSignal: &[(Item, f32)] = &[
    (Item::SteelPipe, 120.000000),
    (Item::CopperSheet, 60.000000),
    (Item::CircuitBoard, 120.000000),
];

const OUT_BlockSignal: &[(Item, f32)] = &[];

const IN_BlueprintDesigner: &[(Item, f32)] = &[
    (Item::ModularFrame, 900.000000),
    (Item::Cable, 1500.000000),
    (Item::Concrete, 6000.000000),
    (Item::SteelBeam, 6000.000000),
];

const OUT_BlueprintDesigner: &[(Item, f32)] = &[];

const IN_Cable: &[(Item, f32)] = &[(Item::Wire, 60.000000)];

const OUT_Cable: &[(Item, f32)] = &[(Item::Cable, 30.000000)];

const IN_CandyCane: &[(Item, f32)] = &[];

const OUT_CandyCane: &[(Item, f32)] = &[];

const IN_CateriumIngot: &[(Item, f32)] = &[(Item::CateriumOre, 45.000000)];

const OUT_CateriumIngot: &[(Item, f32)] = &[(Item::CateriumIngot, 15.000000)];

const IN_CatwalkCorner: &[(Item, f32)] =
    &[(Item::IronRod, 120.000000), (Item::IronPlate, 60.000000)];

const OUT_CatwalkCorner: &[(Item, f32)] = &[];

const IN_CatwalkCrossing: &[(Item, f32)] =
    &[(Item::IronRod, 120.000000), (Item::IronPlate, 60.000000)];

const OUT_CatwalkCrossing: &[(Item, f32)] = &[];

const IN_CatwalkRamp: &[(Item, f32)] = &[(Item::IronRod, 120.000000), (Item::IronPlate, 60.000000)];

const OUT_CatwalkRamp: &[(Item, f32)] = &[];

const IN_CatwalkStairs: &[(Item, f32)] =
    &[(Item::IronRod, 120.000000), (Item::IronPlate, 60.000000)];

const OUT_CatwalkStairs: &[(Item, f32)] = &[];

const IN_CatwalkStraight: &[(Item, f32)] =
    &[(Item::IronRod, 120.000000), (Item::IronPlate, 60.000000)];

const OUT_CatwalkStraight: &[(Item, f32)] = &[];

const IN_CatwalkTCrossing: &[(Item, f32)] =
    &[(Item::IronRod, 120.000000), (Item::IronPlate, 60.000000)];

const OUT_CatwalkTCrossing: &[(Item, f32)] = &[];

const IN_CeilingLight: &[(Item, f32)] = &[
    (Item::Quickwire, 3000.000000),
    (Item::Wire, 960.000000),
    (Item::SteelBeam, 360.000000),
];

const OUT_CeilingLight: &[(Item, f32)] = &[];

const IN_CenterDoorWall: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_CenterDoorWall: &[(Item, f32)] = &[];

const IN_Chainsaw: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 5.000000),
    (Item::IronRod, 25.000000),
    (Item::Screw, 160.000000),
    (Item::Cable, 15.000000),
];

const OUT_Chainsaw: &[(Item, f32)] = &[(Item::Chainsaw, 1.000000)];

const IN_CircuitBoard: &[(Item, f32)] =
    &[(Item::CopperSheet, 15.000000), (Item::Plastic, 30.000000)];

const OUT_CircuitBoard: &[(Item, f32)] = &[(Item::CircuitBoard, 7.500000)];

const IN_ClusterNobelisk: &[(Item, f32)] = &[
    (Item::Nobelisk, 7.500000),
    (Item::SmokelessPowder, 10.000000),
];

const OUT_ClusterNobelisk: &[(Item, f32)] = &[(Item::ClusterNobelisk, 2.500000)];

const IN_CoalGenerator: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 1200.000000),
    (Item::Rotor, 600.000000),
    (Item::Cable, 1800.000000),
];

const OUT_CoalGenerator: &[(Item, f32)] = &[];

const IN_ColorCartridge: &[(Item, f32)] = &[(Item::FlowerPetals, 50.000000)];

const OUT_ColorCartridge: &[(Item, f32)] = &[(Item::ColorCartridge, 100.000000)];

const IN_Computer: &[(Item, f32)] = &[
    (Item::CircuitBoard, 25.000000),
    (Item::Cable, 22.500000),
    (Item::Plastic, 45.000000),
    (Item::Screw, 130.000000),
];

const OUT_Computer: &[(Item, f32)] = &[(Item::Computer, 2.500000)];

const IN_Concrete: &[(Item, f32)] = &[(Item::Limestone, 45.000000)];

const OUT_Concrete: &[(Item, f32)] = &[(Item::Concrete, 15.000000)];

const IN_Constructor: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 120.000000),
    (Item::Cable, 480.000000),
];

const OUT_Constructor: &[(Item, f32)] = &[];

const IN_ConveyorBeltMk1: &[(Item, f32)] = &[(Item::IronPlate, 60.000000)];

const OUT_ConveyorBeltMk1: &[(Item, f32)] = &[];

const IN_ConveyorBeltMk2: &[(Item, f32)] = &[(Item::ReinforcedIronPlate, 60.000000)];

const OUT_ConveyorBeltMk2: &[(Item, f32)] = &[];

const IN_ConveyorBeltMk3: &[(Item, f32)] = &[(Item::SteelBeam, 60.000000)];

const OUT_ConveyorBeltMk3: &[(Item, f32)] = &[];

const IN_ConveyorBeltMk4: &[(Item, f32)] = &[(Item::EncasedIndustrialBeam, 60.000000)];

const OUT_ConveyorBeltMk4: &[(Item, f32)] = &[];

const IN_ConveyorBeltMk5: &[(Item, f32)] = &[(Item::AlcladAluminumSheet, 60.000000)];

const OUT_ConveyorBeltMk5: &[(Item, f32)] = &[];

const IN_ConveyorCeilingMount: &[(Item, f32)] = &[
    (Item::IronRod, 120.000000),
    (Item::IronPlate, 120.000000),
    (Item::Concrete, 120.000000),
];

const OUT_ConveyorCeilingMount: &[(Item, f32)] = &[];

const IN_ConveyorLiftFloorHole: &[(Item, f32)] = &[
    (Item::IronRod, 120.000000),
    (Item::IronPlate, 120.000000),
    (Item::Concrete, 120.000000),
];

const OUT_ConveyorLiftFloorHole: &[(Item, f32)] = &[];

const IN_ConveyorLiftMk1: &[(Item, f32)] = &[(Item::IronPlate, 120.000000)];

const OUT_ConveyorLiftMk1: &[(Item, f32)] = &[];

const IN_ConveyorLiftMk2: &[(Item, f32)] = &[(Item::ReinforcedIronPlate, 120.000000)];

const OUT_ConveyorLiftMk2: &[(Item, f32)] = &[];

const IN_ConveyorLiftMk3: &[(Item, f32)] = &[(Item::SteelBeam, 120.000000)];

const OUT_ConveyorLiftMk3: &[(Item, f32)] = &[];

const IN_ConveyorLiftMk4: &[(Item, f32)] = &[(Item::EncasedIndustrialBeam, 120.000000)];

const OUT_ConveyorLiftMk4: &[(Item, f32)] = &[];

const IN_ConveyorLiftMk5: &[(Item, f32)] = &[(Item::AlcladAluminumSheet, 120.000000)];

const OUT_ConveyorLiftMk5: &[(Item, f32)] = &[];

const IN_ConveyorMerger: &[(Item, f32)] =
    &[(Item::IronPlate, 120.000000), (Item::IronRod, 120.000000)];

const OUT_ConveyorMerger: &[(Item, f32)] = &[];

const IN_ConveyorPole: &[(Item, f32)] = &[
    (Item::IronRod, 60.000000),
    (Item::IronPlate, 60.000000),
    (Item::Concrete, 60.000000),
];

const OUT_ConveyorPole: &[(Item, f32)] = &[];

const IN_ConveyorSplitter: &[(Item, f32)] =
    &[(Item::IronPlate, 120.000000), (Item::Cable, 120.000000)];

const OUT_ConveyorSplitter: &[(Item, f32)] = &[];

const IN_ConveyorWallMount: &[(Item, f32)] = &[
    (Item::IronRod, 120.000000),
    (Item::IronPlate, 120.000000),
    (Item::Concrete, 120.000000),
];

const OUT_ConveyorWallMount: &[(Item, f32)] = &[];

const IN_ConveyorWallX1: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_ConveyorWallX1: &[(Item, f32)] = &[];

const IN_ConveyorWallX2: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_ConveyorWallX2: &[(Item, f32)] = &[];

const IN_ConveyorWallX3: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_ConveyorWallX3: &[(Item, f32)] = &[];

const IN_CoolingSystem: &[(Item, f32)] = &[
    (Item::HeatSink, 12.000000),
    (Item::Rubber, 12.000000),
    (Item::Water, 30.000000),
    (Item::NitrogenGas, 150.000000),
];

const OUT_CoolingSystem: &[(Item, f32)] = &[(Item::CoolingSystem, 6.000000)];

const IN_CopperIngot: &[(Item, f32)] = &[(Item::CopperOre, 30.000000)];

const OUT_CopperIngot: &[(Item, f32)] = &[(Item::CopperIngot, 30.000000)];

const IN_CopperPowder: &[(Item, f32)] = &[(Item::CopperIngot, 300.000000)];

const OUT_CopperPowder: &[(Item, f32)] = &[(Item::CopperPowder, 50.000000)];

const IN_CopperSheet: &[(Item, f32)] = &[(Item::CopperIngot, 20.000000)];

const OUT_CopperSheet: &[(Item, f32)] = &[(Item::CopperSheet, 10.000000)];

const IN_CraftBench: &[(Item, f32)] = &[(Item::IronPlate, 180.000000), (Item::IronRod, 180.000000)];

const OUT_CraftBench: &[(Item, f32)] = &[];

const IN_CrystalOscillator: &[(Item, f32)] = &[
    (Item::QuartzCrystal, 18.000000),
    (Item::Cable, 14.000000),
    (Item::ReinforcedIronPlate, 2.500000),
];

const OUT_CrystalOscillator: &[(Item, f32)] = &[(Item::CrystalOscillator, 1.000000)];

const IN_CyberWagon: &[(Item, f32)] = &[(Item::ReinforcedIronPlate, 600.000000)];

const OUT_CyberWagon: &[(Item, f32)] = &[];

const IN_DisplaySign: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 120.000000),
    (Item::QuartzCrystal, 300.000000),
];

const OUT_DisplaySign: &[(Item, f32)] = &[];

const IN_DoubleRamp2m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_DoubleRamp2m: &[(Item, f32)] = &[];

const IN_DoubleRamp4m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_DoubleRamp4m: &[(Item, f32)] = &[];

const IN_DoubleRamp8m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_DoubleRamp8m: &[(Item, f32)] = &[];

const IN_DoubleWallOutletMk1: &[(Item, f32)] =
    &[(Item::Wire, 480.000000), (Item::IronRod, 120.000000)];

const OUT_DoubleWallOutletMk1: &[(Item, f32)] = &[];

const IN_DoubleWallOutletMk2: &[(Item, f32)] =
    &[(Item::Quickwire, 960.000000), (Item::IronRod, 240.000000)];

const OUT_DoubleWallOutletMk2: &[(Item, f32)] = &[];

const IN_DoubleWallOutletMk3: &[(Item, f32)] = &[
    (Item::HighSpeedConnector, 360.000000),
    (Item::SteelPipe, 360.000000),
];

const OUT_DoubleWallOutletMk3: &[(Item, f32)] = &[];

const IN_DownCornerRamp1m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_DownCornerRamp1m: &[(Item, f32)] = &[];

const IN_DownCornerRamp2m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_DownCornerRamp2m: &[(Item, f32)] = &[];

const IN_DownCornerRamp4m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_DownCornerRamp4m: &[(Item, f32)] = &[];

const IN_Drone: &[(Item, f32)] = &[
    (Item::Motor, 240.000000),
    (Item::AlcladAluminumSheet, 600.000000),
    (Item::RadioControlUnit, 60.000000),
    (Item::AILimiter, 120.000000),
    (Item::PortableMiner, 60.000000),
];

const OUT_Drone: &[(Item, f32)] = &[];

const IN_DronePort: &[(Item, f32)] = &[
    (Item::HeavyModularFrame, 1200.000000),
    (Item::HighSpeedConnector, 1200.000000),
    (Item::AlcladAluminumSheet, 3000.000000),
    (Item::AluminumCasing, 3000.000000),
    (Item::RadioControlUnit, 600.000000),
];

const OUT_DronePort: &[(Item, f32)] = &[];

const IN_ElectricLocomotive: &[(Item, f32)] = &[
    (Item::HeavyModularFrame, 300.000000),
    (Item::Motor, 600.000000),
    (Item::SteelPipe, 900.000000),
    (Item::Computer, 300.000000),
];

const OUT_ElectricLocomotive: &[(Item, f32)] = &[];

const IN_ElectromagneticControlRod: &[(Item, f32)] =
    &[(Item::Stator, 6.000000), (Item::AILimiter, 4.000000)];

const OUT_ElectromagneticControlRod: &[(Item, f32)] =
    &[(Item::ElectromagneticControlRod, 4.000000)];

const IN_EmptyCanister: &[(Item, f32)] = &[(Item::Plastic, 30.000000)];

const OUT_EmptyCanister: &[(Item, f32)] = &[(Item::EmptyCanister, 60.000000)];

const IN_EmptyFluidTank: &[(Item, f32)] = &[(Item::AluminumIngot, 60.000000)];

const OUT_EmptyFluidTank: &[(Item, f32)] = &[(Item::EmptyFluidTank, 60.000000)];

const IN_EmptyPlatform: &[(Item, f32)] = &[
    (Item::HeavyModularFrame, 360.000000),
    (Item::Concrete, 3000.000000),
];

const OUT_EmptyPlatform: &[(Item, f32)] = &[];

const IN_EmptyPlatformWithCatwalk: &[(Item, f32)] = &[
    (Item::HeavyModularFrame, 360.000000),
    (Item::Concrete, 3000.000000),
];

const OUT_EmptyPlatformWithCatwalk: &[(Item, f32)] = &[];

const IN_EncasedIndustrialBeam: &[(Item, f32)] =
    &[(Item::SteelBeam, 24.000000), (Item::Concrete, 30.000000)];

const OUT_EncasedIndustrialBeam: &[(Item, f32)] = &[(Item::EncasedIndustrialBeam, 6.000000)];

const IN_EncasedPlutoniumCell: &[(Item, f32)] = &[
    (Item::PlutoniumPellet, 10.000000),
    (Item::Concrete, 20.000000),
];

const OUT_EncasedPlutoniumCell: &[(Item, f32)] = &[(Item::EncasedPlutoniumCell, 5.000000)];

const IN_EncasedUraniumCell: &[(Item, f32)] = &[
    (Item::Uranium, 50.000000),
    (Item::Concrete, 15.000000),
    (Item::SulfuricAcid, 40.000000),
];

const OUT_EncasedUraniumCell: &[(Item, f32)] = &[
    (Item::EncasedUraniumCell, 25.000000),
    (Item::SulfuricAcid, 10.000000),
];

const IN_EquipmentWorkshop: &[(Item, f32)] =
    &[(Item::IronPlate, 360.000000), (Item::IronRod, 240.000000)];

const OUT_EquipmentWorkshop: &[(Item, f32)] = &[];

const IN_Explorer: &[(Item, f32)] = &[
    (Item::CrystalOscillator, 300.000000),
    (Item::Motor, 300.000000),
    (Item::SteelPipe, 900.000000),
    (Item::HeavyModularFrame, 300.000000),
];

const OUT_Explorer: &[(Item, f32)] = &[];

const IN_ExplosiveRebar: &[(Item, f32)] = &[
    (Item::IronRebar, 10.000000),
    (Item::SmokelessPowder, 10.000000),
    (Item::SteelPipe, 10.000000),
];

const OUT_ExplosiveRebar: &[(Item, f32)] = &[(Item::ExplosiveRebar, 5.000000)];

const IN_FICSMASGiftTree: &[(Item, f32)] = &[];

const OUT_FICSMASGiftTree: &[(Item, f32)] = &[];

const IN_FICSMASPowerLight: &[(Item, f32)] = &[(Item::Cable, 60.000000)];

const OUT_FICSMASPowerLight: &[(Item, f32)] = &[];

const IN_FICSMASSnowDispenser: &[(Item, f32)] = &[];

const OUT_FICSMASSnowDispenser: &[(Item, f32)] = &[];

const IN_FICSMASWreath: &[(Item, f32)] = &[];

const OUT_FICSMASWreath: &[(Item, f32)] = &[];

const IN_Fabric: &[(Item, f32)] = &[(Item::Mycelia, 15.000000), (Item::Biomass, 75.000000)];

const OUT_Fabric: &[(Item, f32)] = &[(Item::Fabric, 15.000000)];

const IN_FactoryCart: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 12.000000),
    (Item::IronRod, 12.000000),
    (Item::Rotor, 6.000000),
];

const OUT_FactoryCart: &[(Item, f32)] = &[(Item::FactoryCart, 3.000000)];

const IN_FloodLightTower: &[(Item, f32)] = &[
    (Item::Quickwire, 1500.000000),
    (Item::CopperSheet, 240.000000),
    (Item::EncasedIndustrialBeam, 480.000000),
];

const OUT_FloodLightTower: &[(Item, f32)] = &[];

const IN_FluidBuffer: &[(Item, f32)] = &[
    (Item::CopperSheet, 600.000000),
    (Item::ModularFrame, 300.000000),
];

const OUT_FluidBuffer: &[(Item, f32)] = &[];

const IN_FluidFreightPlatform: &[(Item, f32)] = &[
    (Item::HeavyModularFrame, 360.000000),
    (Item::Computer, 120.000000),
    (Item::Concrete, 3000.000000),
    (Item::Cable, 1500.000000),
    (Item::Motor, 300.000000),
];

const OUT_FluidFreightPlatform: &[(Item, f32)] = &[];

const IN_Foundation1m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_Foundation1m: &[(Item, f32)] = &[];

const IN_Foundation2m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_Foundation2m: &[(Item, f32)] = &[];

const IN_Foundation4m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_Foundation4m: &[(Item, f32)] = &[];

const IN_Foundry: &[(Item, f32)] = &[
    (Item::ModularFrame, 600.000000),
    (Item::Rotor, 600.000000),
    (Item::Concrete, 1200.000000),
];

const OUT_Foundry: &[(Item, f32)] = &[];

const IN_FrameFloor: &[(Item, f32)] = &[(Item::SteelBeam, 120.000000)];

const OUT_FrameFloor: &[(Item, f32)] = &[];

const IN_FrameFoundation: &[(Item, f32)] = &[(Item::SteelBeam, 300.000000)];

const OUT_FrameFoundation: &[(Item, f32)] = &[];

const IN_FrameRamp: &[(Item, f32)] = &[(Item::SteelBeam, 300.000000)];

const OUT_FrameRamp: &[(Item, f32)] = &[];

const IN_FrameWall: &[(Item, f32)] = &[(Item::SteelBeam, 120.000000)];

const OUT_FrameWall: &[(Item, f32)] = &[];

const IN_FrameWindow: &[(Item, f32)] = &[(Item::Silica, 120.000000)];

const OUT_FrameWindow: &[(Item, f32)] = &[];

const IN_FreightCar: &[(Item, f32)] = &[
    (Item::HeavyModularFrame, 240.000000),
    (Item::SteelPipe, 600.000000),
];

const OUT_FreightCar: &[(Item, f32)] = &[];

const IN_FreightPlatform: &[(Item, f32)] = &[
    (Item::HeavyModularFrame, 360.000000),
    (Item::Computer, 120.000000),
    (Item::Concrete, 3000.000000),
    (Item::Cable, 1500.000000),
    (Item::Motor, 300.000000),
];

const OUT_FreightPlatform: &[(Item, f32)] = &[];

const IN_Fuel: &[(Item, f32)] = &[(Item::CrudeOil, 60.000000)];

const OUT_Fuel: &[(Item, f32)] = &[(Item::Fuel, 40.000000), (Item::PolymerResin, 30.000000)];

const IN_FuelGenerator: &[(Item, f32)] = &[
    (Item::Computer, 300.000000),
    (Item::HeavyModularFrame, 600.000000),
    (Item::Motor, 900.000000),
    (Item::Rubber, 3000.000000),
    (Item::Quickwire, 3000.000000),
];

const OUT_FuelGenerator: &[(Item, f32)] = &[];

const IN_FullFrameWindow: &[(Item, f32)] =
    &[(Item::Silica, 120.000000), (Item::SteelBeam, 120.000000)];

const OUT_FullFrameWindow: &[(Item, f32)] = &[];

const IN_FusedModularFrame: &[(Item, f32)] = &[
    (Item::HeavyModularFrame, 1.500000),
    (Item::AluminumCasing, 75.000000),
    (Item::NitrogenGas, 37.500000),
];

const OUT_FusedModularFrame: &[(Item, f32)] = &[(Item::FusedModularFrame, 1.500000)];

const IN_GasFilter: &[(Item, f32)] = &[
    (Item::Coal, 37.500000),
    (Item::Rubber, 15.000000),
    (Item::Fabric, 15.000000),
];

const OUT_GasFilter: &[(Item, f32)] = &[(Item::GasFilter, 7.500000)];

const IN_GasMask: &[(Item, f32)] = &[
    (Item::Rubber, 100.000000),
    (Item::Plastic, 100.000000),
    (Item::Fabric, 100.000000),
];

const OUT_GasMask: &[(Item, f32)] = &[(Item::GasMask, 1.000000)];

const IN_GasNobelisk: &[(Item, f32)] = &[(Item::Nobelisk, 5.000000), (Item::Biomass, 50.000000)];

const OUT_GasNobelisk: &[(Item, f32)] = &[(Item::GasNobelisk, 5.000000)];

const IN_GateHoleWall: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_GateHoleWall: &[(Item, f32)] = &[];

const IN_GeothermalGenerator: &[(Item, f32)] = &[
    (Item::Supercomputer, 480.000000),
    (Item::HeavyModularFrame, 960.000000),
    (Item::HighSpeedConnector, 960.000000),
    (Item::CopperSheet, 2400.000000),
    (Item::Rubber, 4800.000000),
];

const OUT_GeothermalGenerator: &[(Item, f32)] = &[];

const IN_GiantFICSMASTree: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 3000.000000),
    (Item::Concrete, 30000.000000),
];

const OUT_GiantFICSMASTree: &[(Item, f32)] = &[];

const IN_GlassFrameFoundation: &[(Item, f32)] =
    &[(Item::SteelBeam, 120.000000), (Item::Concrete, 300.000000)];

const OUT_GlassFrameFoundation: &[(Item, f32)] = &[];

const IN_GoldenFactoryCart: &[(Item, f32)] = &[
    (Item::CateriumIngot, 45.000000),
    (Item::IronRod, 12.000000),
    (Item::Rotor, 6.000000),
];

const OUT_GoldenFactoryCart: &[(Item, f32)] = &[(Item::GoldenFactoryCart, 3.000000)];

const IN_Half1mFoundation: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_Half1mFoundation: &[(Item, f32)] = &[];

const IN_Half2mFoundation: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_Half2mFoundation: &[(Item, f32)] = &[];

const IN_Half4mFoundation: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_Half4mFoundation: &[(Item, f32)] = &[];

const IN_HatcherProtein: &[(Item, f32)] = &[(Item::HatcherRemains, 20.000000)];

const OUT_HatcherProtein: &[(Item, f32)] = &[(Item::AlienProtein, 20.000000)];

const IN_HazardStorageBox: &[(Item, f32)] =
    &[(Item::IronPlate, 360.000000), (Item::IronRod, 360.000000)];

const OUT_HazardStorageBox: &[(Item, f32)] = &[];

const IN_HazmatSuit: &[(Item, f32)] = &[
    (Item::Rubber, 25.000000),
    (Item::Plastic, 25.000000),
    (Item::AlcladAluminumSheet, 25.000000),
    (Item::Fabric, 25.000000),
];

const OUT_HazmatSuit: &[(Item, f32)] = &[(Item::HazmatSuit, 0.500000)];

const IN_HeatSink: &[(Item, f32)] = &[
    (Item::AlcladAluminumSheet, 37.500000),
    (Item::CopperSheet, 22.500000),
];

const OUT_HeatSink: &[(Item, f32)] = &[(Item::HeatSink, 7.500000)];

const IN_HeavyModularFrame: &[(Item, f32)] = &[
    (Item::ModularFrame, 10.000000),
    (Item::SteelPipe, 30.000000),
    (Item::EncasedIndustrialBeam, 10.000000),
    (Item::Screw, 200.000000),
];

const OUT_HeavyModularFrame: &[(Item, f32)] = &[(Item::HeavyModularFrame, 2.000000)];

const IN_HexFrameWindow: &[(Item, f32)] =
    &[(Item::Silica, 120.000000), (Item::SteelBeam, 120.000000)];

const OUT_HexFrameWindow: &[(Item, f32)] = &[];

const IN_HighSpeedConnector: &[(Item, f32)] = &[
    (Item::Quickwire, 210.000000),
    (Item::Cable, 37.500000),
    (Item::CircuitBoard, 3.750000),
];

const OUT_HighSpeedConnector: &[(Item, f32)] = &[(Item::HighSpeedConnector, 3.750000)];

const IN_HogProtein: &[(Item, f32)] = &[(Item::HogRemains, 20.000000)];

const OUT_HogProtein: &[(Item, f32)] = &[(Item::AlienProtein, 20.000000)];

const IN_HomingRifleAmmo: &[(Item, f32)] = &[
    (Item::RifleAmmo, 50.000000),
    (Item::HighSpeedConnector, 2.500000),
];

const OUT_HomingRifleAmmo: &[(Item, f32)] = &[(Item::HomingRifleAmmo, 25.000000)];

const IN_HoverPack: &[(Item, f32)] = &[
    (Item::Motor, 4.000000),
    (Item::HeavyModularFrame, 2.000000),
    (Item::Computer, 4.000000),
    (Item::AlcladAluminumSheet, 20.000000),
];

const OUT_HoverPack: &[(Item, f32)] = &[(Item::HoverPack, 0.500000)];

const IN_Hypertube: &[(Item, f32)] =
    &[(Item::CopperSheet, 60.000000), (Item::SteelPipe, 60.000000)];

const OUT_Hypertube: &[(Item, f32)] = &[];

const IN_HypertubeEntrance: &[(Item, f32)] = &[
    (Item::EncasedIndustrialBeam, 240.000000),
    (Item::Rotor, 240.000000),
    (Item::SteelPipe, 600.000000),
];

const OUT_HypertubeEntrance: &[(Item, f32)] = &[];

const IN_HypertubeFloorHole: &[(Item, f32)] = &[
    (Item::IronRod, 120.000000),
    (Item::IronPlate, 120.000000),
    (Item::Concrete, 120.000000),
];

const OUT_HypertubeFloorHole: &[(Item, f32)] = &[];

const IN_HypertubeSupport: &[(Item, f32)] =
    &[(Item::IronPlate, 120.000000), (Item::Concrete, 120.000000)];

const OUT_HypertubeSupport: &[(Item, f32)] = &[];

const IN_HypertubeWallHole: &[(Item, f32)] =
    &[(Item::IronPlate, 120.000000), (Item::Concrete, 120.000000)];

const OUT_HypertubeWallHole: &[(Item, f32)] = &[];

const IN_HypertubeWallSupport: &[(Item, f32)] =
    &[(Item::IronPlate, 120.000000), (Item::Concrete, 120.000000)];

const OUT_HypertubeWallSupport: &[(Item, f32)] = &[];

const IN_IndustrialFluidBuffer: &[(Item, f32)] = &[
    (Item::Plastic, 1800.000000),
    (Item::HeavyModularFrame, 180.000000),
];

const OUT_IndustrialFluidBuffer: &[(Item, f32)] = &[];

const IN_IndustrialStorageContainer: &[(Item, f32)] = &[
    (Item::SteelBeam, 1200.000000),
    (Item::SteelPipe, 1200.000000),
];

const OUT_IndustrialStorageContainer: &[(Item, f32)] = &[];

const IN_InnerCornerExtension1m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InnerCornerExtension1m: &[(Item, f32)] = &[];

const IN_InnerCornerExtension2m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InnerCornerExtension2m: &[(Item, f32)] = &[];

const IN_InnerCornerExtension4m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InnerCornerExtension4m: &[(Item, f32)] = &[];

const IN_InnerCornerQuarterPipe: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InnerCornerQuarterPipe: &[(Item, f32)] = &[];

const IN_InnerCornerRoof1m: &[(Item, f32)] = &[(Item::Concrete, 180.000000)];

const OUT_InnerCornerRoof1m: &[(Item, f32)] = &[];

const IN_InnerCornerRoof2m: &[(Item, f32)] = &[(Item::Concrete, 180.000000)];

const OUT_InnerCornerRoof2m: &[(Item, f32)] = &[];

const IN_InnerCornerRoof4m: &[(Item, f32)] = &[(Item::Concrete, 180.000000)];

const OUT_InnerCornerRoof4m: &[(Item, f32)] = &[];

const IN_InvDownCorner1m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InvDownCorner1m: &[(Item, f32)] = &[];

const IN_InvDownCorner2m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InvDownCorner2m: &[(Item, f32)] = &[];

const IN_InvDownCorner4m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InvDownCorner4m: &[(Item, f32)] = &[];

const IN_InvRamp1m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InvRamp1m: &[(Item, f32)] = &[];

const IN_InvRamp2m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InvRamp2m: &[(Item, f32)] = &[];

const IN_InvRamp4m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InvRamp4m: &[(Item, f32)] = &[];

const IN_InvRampWall1m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_InvRampWall1m: &[(Item, f32)] = &[];

const IN_InvRampWall2m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_InvRampWall2m: &[(Item, f32)] = &[];

const IN_InvRampWall4m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_InvRampWall4m: &[(Item, f32)] = &[];

const IN_InvRampWall8m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_InvRampWall8m: &[(Item, f32)] = &[];

const IN_InvUpCorner1m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InvUpCorner1m: &[(Item, f32)] = &[];

const IN_InvUpCorner2m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InvUpCorner2m: &[(Item, f32)] = &[];

const IN_InvUpCorner4m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InvUpCorner4m: &[(Item, f32)] = &[];

const IN_InvertedFrameRamp: &[(Item, f32)] = &[(Item::SteelBeam, 300.000000)];

const OUT_InvertedFrameRamp: &[(Item, f32)] = &[];

const IN_InvertedInnerCornerQuarterPipe: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InvertedInnerCornerQuarterPipe: &[(Item, f32)] = &[];

const IN_InvertedOuterCornerQuarterPipe: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InvertedOuterCornerQuarterPipe: &[(Item, f32)] = &[];

const IN_InvertedQuarterPipe: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_InvertedQuarterPipe: &[(Item, f32)] = &[];

const IN_IodineInfusedFilter: &[(Item, f32)] = &[
    (Item::GasFilter, 3.750000),
    (Item::Quickwire, 30.000000),
    (Item::AluminumCasing, 3.750000),
];

const OUT_IodineInfusedFilter: &[(Item, f32)] = &[(Item::IodineInfusedFilter, 3.750000)];

const IN_IronIngot: &[(Item, f32)] = &[(Item::IronOre, 30.000000)];

const OUT_IronIngot: &[(Item, f32)] = &[(Item::IronIngot, 30.000000)];

const IN_IronPlate: &[(Item, f32)] = &[(Item::IronIngot, 30.000000)];

const OUT_IronPlate: &[(Item, f32)] = &[(Item::IronPlate, 20.000000)];

const IN_IronRebar: &[(Item, f32)] = &[(Item::IronRod, 15.000000)];

const OUT_IronRebar: &[(Item, f32)] = &[(Item::IronRebar, 15.000000)];

const IN_IronRod: &[(Item, f32)] = &[(Item::IronIngot, 15.000000)];

const OUT_IronRod: &[(Item, f32)] = &[(Item::IronRod, 15.000000)];

const IN_Jetpack: &[(Item, f32)] = &[
    (Item::Plastic, 25.000000),
    (Item::Rubber, 25.000000),
    (Item::CircuitBoard, 7.500000),
    (Item::Motor, 2.500000),
];

const OUT_Jetpack: &[(Item, f32)] = &[(Item::Jetpack, 0.500000)];

const IN_JumpPad: &[(Item, f32)] = &[
    (Item::Rotor, 120.000000),
    (Item::IronPlate, 900.000000),
    (Item::Cable, 600.000000),
];

const OUT_JumpPad: &[(Item, f32)] = &[];

const IN_LabelSign2m: &[(Item, f32)] = &[
    (Item::IronPlate, 120.000000),
    (Item::QuartzCrystal, 120.000000),
];

const OUT_LabelSign2m: &[(Item, f32)] = &[];

const IN_LabelSign3m: &[(Item, f32)] = &[
    (Item::IronPlate, 180.000000),
    (Item::QuartzCrystal, 180.000000),
];

const OUT_LabelSign3m: &[(Item, f32)] = &[];

const IN_LabelSign4m: &[(Item, f32)] = &[
    (Item::IronPlate, 240.000000),
    (Item::QuartzCrystal, 240.000000),
];

const OUT_LabelSign4m: &[(Item, f32)] = &[];

const IN_Ladder: &[(Item, f32)] = &[(Item::IronRod, 120.000000)];

const OUT_Ladder: &[(Item, f32)] = &[];

const IN_LargeBillboard: &[(Item, f32)] = &[
    (Item::EncasedIndustrialBeam, 720.000000),
    (Item::CopperSheet, 1200.000000),
    (Item::CrystalOscillator, 300.000000),
];

const OUT_LargeBillboard: &[(Item, f32)] = &[];

const IN_LightsControlPanel: &[(Item, f32)] = &[
    (Item::Cable, 600.000000),
    (Item::ReinforcedIronPlate, 300.000000),
    (Item::AILimiter, 180.000000),
];

const OUT_LightsControlPanel: &[(Item, f32)] = &[];

const IN_LiquidBiofuel: &[(Item, f32)] =
    &[(Item::SolidBiofuel, 90.000000), (Item::Water, 45.000000)];

const OUT_LiquidBiofuel: &[(Item, f32)] = &[(Item::LiquidBiofuel, 60.000000)];

const IN_LookoutTower: &[(Item, f32)] =
    &[(Item::IronPlate, 300.000000), (Item::IronRod, 300.000000)];

const OUT_LookoutTower: &[(Item, f32)] = &[];

const IN_MAM: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 300.000000),
    (Item::Cable, 900.000000),
    (Item::Wire, 2700.000000),
];

const OUT_MAM: &[(Item, f32)] = &[];

const IN_MagneticFieldGenerator: &[(Item, f32)] = &[
    (Item::VersatileFramework, 2.500000),
    (Item::ElectromagneticControlRod, 1.000000),
    (Item::Battery, 5.000000),
];

const OUT_MagneticFieldGenerator: &[(Item, f32)] = &[(Item::MagneticFieldGenerator, 1.000000)];

const IN_Manufacturer: &[(Item, f32)] = &[
    (Item::Motor, 300.000000),
    (Item::HeavyModularFrame, 600.000000),
    (Item::Cable, 3000.000000),
    (Item::Plastic, 3000.000000),
];

const OUT_Manufacturer: &[(Item, f32)] = &[];

const IN_MedicalStorageBox: &[(Item, f32)] =
    &[(Item::IronPlate, 360.000000), (Item::IronRod, 360.000000)];

const OUT_MedicalStorageBox: &[(Item, f32)] = &[];

const IN_MetalBeam: &[(Item, f32)] = &[(Item::SteelBeam, 120.000000)];

const OUT_MetalBeam: &[(Item, f32)] = &[];

const IN_MinerMk1: &[(Item, f32)] = &[
    (Item::PortableMiner, 60.000000),
    (Item::IronPlate, 600.000000),
    (Item::Concrete, 600.000000),
];

const OUT_MinerMk1: &[(Item, f32)] = &[];

const IN_MinerMk2: &[(Item, f32)] = &[
    (Item::PortableMiner, 120.000000),
    (Item::EncasedIndustrialBeam, 600.000000),
    (Item::SteelPipe, 1200.000000),
    (Item::ModularFrame, 600.000000),
];

const OUT_MinerMk2: &[(Item, f32)] = &[];

const IN_MinerMk3: &[(Item, f32)] = &[
    (Item::PortableMiner, 180.000000),
    (Item::SteelPipe, 3000.000000),
    (Item::Supercomputer, 300.000000),
    (Item::FusedModularFrame, 600.000000),
    (Item::TurboMotor, 180.000000),
];

const OUT_MinerMk3: &[(Item, f32)] = &[];

const IN_ModernRailing: &[(Item, f32)] = &[(Item::IronRod, 120.000000)];

const OUT_ModernRailing: &[(Item, f32)] = &[];

const IN_ModularEngine: &[(Item, f32)] = &[
    (Item::Motor, 2.000000),
    (Item::Rubber, 15.000000),
    (Item::SmartPlating, 2.000000),
];

const OUT_ModularEngine: &[(Item, f32)] = &[(Item::ModularEngine, 1.000000)];

const IN_ModularFrame: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 3.000000),
    (Item::IronRod, 12.000000),
];

const OUT_ModularFrame: &[(Item, f32)] = &[(Item::ModularFrame, 2.000000)];

const IN_Motor: &[(Item, f32)] = &[(Item::Rotor, 10.000000), (Item::Stator, 10.000000)];

const OUT_Motor: &[(Item, f32)] = &[(Item::Motor, 5.000000)];

const IN_NitricAcid: &[(Item, f32)] = &[
    (Item::NitrogenGas, 120.000000),
    (Item::Water, 30.000000),
    (Item::IronPlate, 10.000000),
];

const OUT_NitricAcid: &[(Item, f32)] = &[(Item::NitricAcid, 30.000000)];

const IN_Nobelisk: &[(Item, f32)] = &[(Item::BlackPowder, 20.000000), (Item::SteelPipe, 20.000000)];

const OUT_Nobelisk: &[(Item, f32)] = &[(Item::Nobelisk, 10.000000)];

const IN_NobeliskDetonator: &[(Item, f32)] = &[
    (Item::ObjectScanner, 0.750000),
    (Item::SteelBeam, 7.500000),
    (Item::Cable, 37.500000),
];

const OUT_NobeliskDetonator: &[(Item, f32)] = &[(Item::NobeliskDetonator, 0.750000)];

const IN_NonFissileUranium: &[(Item, f32)] = &[
    (Item::UraniumWaste, 37.500000),
    (Item::Silica, 25.000000),
    (Item::NitricAcid, 15.000000),
    (Item::SulfuricAcid, 15.000000),
];

const OUT_NonFissileUranium: &[(Item, f32)] = &[
    (Item::NonFissileUranium, 50.000000),
    (Item::Water, 15.000000),
];

const IN_NuclearPasta: &[(Item, f32)] = &[
    (Item::CopperPowder, 100.000000),
    (Item::PressureConversionCube, 0.500000),
];

const OUT_NuclearPasta: &[(Item, f32)] = &[(Item::NuclearPasta, 0.500000)];

const IN_NuclearPowerPlant: &[(Item, f32)] = &[
    (Item::Concrete, 15000.000000),
    (Item::HeavyModularFrame, 1500.000000),
    (Item::Supercomputer, 300.000000),
    (Item::Cable, 6000.000000),
    (Item::AlcladAluminumSheet, 6000.000000),
];

const OUT_NuclearPowerPlant: &[(Item, f32)] = &[];

const IN_NukeNobelisk: &[(Item, f32)] = &[
    (Item::Nobelisk, 2.500000),
    (Item::EncasedUraniumCell, 10.000000),
    (Item::SmokelessPowder, 5.000000),
    (Item::AILimiter, 3.000000),
];

const OUT_NukeNobelisk: &[(Item, f32)] = &[(Item::NukeNobelisk, 0.500000)];

const IN_NutritionalInhaler: &[(Item, f32)] = &[
    (Item::BaconAgaric, 3.000000),
    (Item::Paleberry, 6.000000),
    (Item::BerylNut, 15.000000),
];

const OUT_NutritionalInhaler: &[(Item, f32)] = &[(Item::MedicinalInhaler, 3.000000)];

const IN_ObjectScanner: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 6.000000),
    (Item::Wire, 30.000000),
    (Item::Screw, 75.000000),
];

const OUT_ObjectScanner: &[(Item, f32)] = &[(Item::ObjectScanner, 1.500000)];

const IN_OilExtractor: &[(Item, f32)] = &[
    (Item::Motor, 900.000000),
    (Item::EncasedIndustrialBeam, 1200.000000),
    (Item::Cable, 3600.000000),
];

const OUT_OilExtractor: &[(Item, f32)] = &[];

const IN_OuterCornerExtension1m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_OuterCornerExtension1m: &[(Item, f32)] = &[];

const IN_OuterCornerExtension2m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_OuterCornerExtension2m: &[(Item, f32)] = &[];

const IN_OuterCornerExtension4m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_OuterCornerExtension4m: &[(Item, f32)] = &[];

const IN_OuterCornerQuarterPipe: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_OuterCornerQuarterPipe: &[(Item, f32)] = &[];

const IN_OuterCornerRoof1m: &[(Item, f32)] = &[(Item::Concrete, 180.000000)];

const OUT_OuterCornerRoof1m: &[(Item, f32)] = &[];

const IN_OuterCornerRoof2m: &[(Item, f32)] = &[(Item::Concrete, 180.000000)];

const OUT_OuterCornerRoof2m: &[(Item, f32)] = &[];

const IN_OuterCornerRoof4m: &[(Item, f32)] = &[(Item::Concrete, 180.000000)];

const OUT_OuterCornerRoof4m: &[(Item, f32)] = &[];

const IN_PackagedAluminaSolution: &[(Item, f32)] = &[
    (Item::AluminaSolution, 120.000000),
    (Item::EmptyCanister, 120.000000),
];

const OUT_PackagedAluminaSolution: &[(Item, f32)] = &[(Item::PackagedAluminaSolution, 120.000000)];

const IN_PackagedFuel: &[(Item, f32)] =
    &[(Item::Fuel, 40.000000), (Item::EmptyCanister, 40.000000)];

const OUT_PackagedFuel: &[(Item, f32)] = &[(Item::PackagedFuel, 40.000000)];

const IN_PackagedHeavyOilResidue: &[(Item, f32)] = &[
    (Item::HeavyOilResidue, 30.000000),
    (Item::EmptyCanister, 30.000000),
];

const OUT_PackagedHeavyOilResidue: &[(Item, f32)] = &[(Item::PackagedHeavyOilResidue, 30.000000)];

const IN_PackagedLiquidBiofuel: &[(Item, f32)] = &[
    (Item::LiquidBiofuel, 40.000000),
    (Item::EmptyCanister, 40.000000),
];

const OUT_PackagedLiquidBiofuel: &[(Item, f32)] = &[(Item::PackagedLiquidBiofuel, 40.000000)];

const IN_PackagedNitricAcid: &[(Item, f32)] = &[
    (Item::NitricAcid, 30.000000),
    (Item::EmptyFluidTank, 30.000000),
];

const OUT_PackagedNitricAcid: &[(Item, f32)] = &[(Item::PackagedNitricAcid, 30.000000)];

const IN_PackagedNitrogenGas: &[(Item, f32)] = &[
    (Item::NitrogenGas, 240.000000),
    (Item::EmptyFluidTank, 60.000000),
];

const OUT_PackagedNitrogenGas: &[(Item, f32)] = &[(Item::PackagedNitrogenGas, 60.000000)];

const IN_PackagedOil: &[(Item, f32)] = &[
    (Item::CrudeOil, 30.000000),
    (Item::EmptyCanister, 30.000000),
];

const OUT_PackagedOil: &[(Item, f32)] = &[(Item::PackagedOil, 30.000000)];

const IN_PackagedSulfuricAcid: &[(Item, f32)] = &[
    (Item::SulfuricAcid, 40.000000),
    (Item::EmptyCanister, 40.000000),
];

const OUT_PackagedSulfuricAcid: &[(Item, f32)] = &[(Item::PackagedSulfuricAcid, 40.000000)];

const IN_PackagedTurbofuel: &[(Item, f32)] = &[
    (Item::Turbofuel, 20.000000),
    (Item::EmptyCanister, 20.000000),
];

const OUT_PackagedTurbofuel: &[(Item, f32)] = &[(Item::PackagedTurbofuel, 20.000000)];

const IN_PackagedWater: &[(Item, f32)] =
    &[(Item::Water, 60.000000), (Item::EmptyCanister, 60.000000)];

const OUT_PackagedWater: &[(Item, f32)] = &[(Item::PackagedWater, 60.000000)];

const IN_Packager: &[(Item, f32)] = &[
    (Item::SteelBeam, 1200.000000),
    (Item::Rubber, 600.000000),
    (Item::Plastic, 600.000000),
];

const OUT_Packager: &[(Item, f32)] = &[];

const IN_PaintedBeam: &[(Item, f32)] = &[(Item::SteelBeam, 120.000000)];

const OUT_PaintedBeam: &[(Item, f32)] = &[];

const IN_PanelWindow: &[(Item, f32)] = &[(Item::Silica, 120.000000)];

const OUT_PanelWindow: &[(Item, f32)] = &[];

const IN_Parachute: &[(Item, f32)] = &[(Item::Fabric, 30.000000), (Item::Cable, 15.000000)];

const OUT_Parachute: &[(Item, f32)] = &[(Item::Parachute, 1.500000)];

const IN_ParticleAccelerator: &[(Item, f32)] = &[
    (Item::RadioControlUnit, 1500.000000),
    (Item::ElectromagneticControlRod, 6000.000000),
    (Item::Supercomputer, 600.000000),
    (Item::CoolingSystem, 3000.000000),
    (Item::FusedModularFrame, 1200.000000),
    (Item::TurboMotor, 600.000000),
];

const OUT_ParticleAccelerator: &[(Item, f32)] = &[];

const IN_PathSignal: &[(Item, f32)] = &[
    (Item::SteelPipe, 120.000000),
    (Item::CopperSheet, 60.000000),
    (Item::Computer, 60.000000),
];

const OUT_PathSignal: &[(Item, f32)] = &[];

const IN_PersonalStorageBox: &[(Item, f32)] =
    &[(Item::IronPlate, 360.000000), (Item::IronRod, 360.000000)];

const OUT_PersonalStorageBox: &[(Item, f32)] = &[];

const IN_PetroleumCoke: &[(Item, f32)] = &[(Item::HeavyOilResidue, 40.000000)];

const OUT_PetroleumCoke: &[(Item, f32)] = &[(Item::PetroleumCoke, 120.000000)];

const IN_PipelineFloorHole: &[(Item, f32)] = &[
    (Item::IronRod, 120.000000),
    (Item::IronPlate, 120.000000),
    (Item::Concrete, 120.000000),
];

const OUT_PipelineFloorHole: &[(Item, f32)] = &[];

const IN_PipelineJunctionCross: &[(Item, f32)] = &[(Item::CopperSheet, 300.000000)];

const OUT_PipelineJunctionCross: &[(Item, f32)] = &[];

const IN_PipelineMk1: &[(Item, f32)] = &[(Item::CopperSheet, 60.000000)];

const OUT_PipelineMk1: &[(Item, f32)] = &[];

const IN_PipelineMk1NoIndicator: &[(Item, f32)] = &[(Item::CopperSheet, 60.000000)];

const OUT_PipelineMk1NoIndicator: &[(Item, f32)] = &[];

const IN_PipelineMk2: &[(Item, f32)] =
    &[(Item::CopperSheet, 120.000000), (Item::Plastic, 60.000000)];

const OUT_PipelineMk2: &[(Item, f32)] = &[];

const IN_PipelineMk2NoIndicator: &[(Item, f32)] =
    &[(Item::CopperSheet, 120.000000), (Item::Plastic, 60.000000)];

const OUT_PipelineMk2NoIndicator: &[(Item, f32)] = &[];

const IN_PipelinePumpMk1: &[(Item, f32)] =
    &[(Item::CopperSheet, 120.000000), (Item::Rotor, 120.000000)];

const OUT_PipelinePumpMk1: &[(Item, f32)] = &[];

const IN_PipelinePumpMk2: &[(Item, f32)] = &[
    (Item::Motor, 120.000000),
    (Item::EncasedIndustrialBeam, 240.000000),
    (Item::Plastic, 480.000000),
];

const OUT_PipelinePumpMk2: &[(Item, f32)] = &[];

const IN_PipelineSupport: &[(Item, f32)] =
    &[(Item::IronPlate, 120.000000), (Item::Concrete, 120.000000)];

const OUT_PipelineSupport: &[(Item, f32)] = &[];

const IN_PipelineWallHole: &[(Item, f32)] = &[
    (Item::IronRod, 120.000000),
    (Item::IronPlate, 60.000000),
    (Item::Concrete, 120.000000),
];

const OUT_PipelineWallHole: &[(Item, f32)] = &[];

const IN_PipelineWallSupport: &[(Item, f32)] = &[
    (Item::IronRod, 120.000000),
    (Item::IronPlate, 60.000000),
    (Item::Concrete, 120.000000),
];

const OUT_PipelineWallSupport: &[(Item, f32)] = &[];

const IN_Plastic: &[(Item, f32)] = &[(Item::CrudeOil, 30.000000)];

const OUT_Plastic: &[(Item, f32)] = &[
    (Item::Plastic, 20.000000),
    (Item::HeavyOilResidue, 10.000000),
];

const IN_PlutoniumFuelRod: &[(Item, f32)] = &[
    (Item::EncasedPlutoniumCell, 7.500000),
    (Item::SteelBeam, 4.500000),
    (Item::ElectromagneticControlRod, 1.500000),
    (Item::HeatSink, 2.500000),
];

const OUT_PlutoniumFuelRod: &[(Item, f32)] = &[(Item::PlutoniumFuelRod, 0.250000)];

const IN_PlutoniumPellet: &[(Item, f32)] = &[
    (Item::NonFissileUranium, 100.000000),
    (Item::UraniumWaste, 25.000000),
];

const OUT_PlutoniumPellet: &[(Item, f32)] = &[(Item::PlutoniumPellet, 30.000000)];

const IN_PortableMiner: &[(Item, f32)] = &[(Item::IronPlate, 3.000000), (Item::IronRod, 6.000000)];

const OUT_PortableMiner: &[(Item, f32)] = &[(Item::PortableMiner, 1.500000)];

const IN_PortraitSign: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 240.000000),
    (Item::QuartzCrystal, 600.000000),
];

const OUT_PortraitSign: &[(Item, f32)] = &[];

const IN_PowerLine: &[(Item, f32)] = &[(Item::Cable, 60.000000)];

const OUT_PowerLine: &[(Item, f32)] = &[];

const IN_PowerPoleMk1: &[(Item, f32)] = &[
    (Item::Wire, 180.000000),
    (Item::IronRod, 60.000000),
    (Item::Concrete, 60.000000),
];

const OUT_PowerPoleMk1: &[(Item, f32)] = &[];

const IN_PowerPoleMk2: &[(Item, f32)] = &[
    (Item::Quickwire, 360.000000),
    (Item::IronRod, 120.000000),
    (Item::Concrete, 120.000000),
];

const OUT_PowerPoleMk2: &[(Item, f32)] = &[];

const IN_PowerPoleMk3: &[(Item, f32)] = &[
    (Item::HighSpeedConnector, 120.000000),
    (Item::SteelPipe, 120.000000),
    (Item::Rubber, 180.000000),
];

const OUT_PowerPoleMk3: &[(Item, f32)] = &[];

const IN_PowerShard1: &[(Item, f32)] = &[(Item::BluePowerSlug, 7.500000)];

const OUT_PowerShard1: &[(Item, f32)] = &[(Item::PowerShard, 7.500000)];

const IN_PowerShard2: &[(Item, f32)] = &[(Item::YellowPowerSlug, 5.000000)];

const OUT_PowerShard2: &[(Item, f32)] = &[(Item::PowerShard, 10.000000)];

const IN_PowerShard5: &[(Item, f32)] = &[(Item::PurplePowerSlug, 2.500000)];

const OUT_PowerShard5: &[(Item, f32)] = &[(Item::PowerShard, 12.500000)];

const IN_PowerStorage: &[(Item, f32)] = &[
    (Item::Wire, 6000.000000),
    (Item::ModularFrame, 600.000000),
    (Item::Stator, 300.000000),
];

const OUT_PowerStorage: &[(Item, f32)] = &[];

const IN_PowerSwitch: &[(Item, f32)] = &[
    (Item::Quickwire, 1200.000000),
    (Item::SteelBeam, 240.000000),
    (Item::AILimiter, 60.000000),
];

const OUT_PowerSwitch: &[(Item, f32)] = &[];

const IN_PowerTower: &[(Item, f32)] = &[
    (Item::Concrete, 600.000000),
    (Item::Wire, 1200.000000),
    (Item::SteelBeam, 1200.000000),
];

const OUT_PowerTower: &[(Item, f32)] = &[];

const IN_PowerTowerPlatform: &[(Item, f32)] = &[
    (Item::Concrete, 600.000000),
    (Item::Wire, 1200.000000),
    (Item::SteelBeam, 1200.000000),
];

const OUT_PowerTowerPlatform: &[(Item, f32)] = &[];

const IN_PressureConversionCube: &[(Item, f32)] = &[
    (Item::FusedModularFrame, 1.000000),
    (Item::RadioControlUnit, 2.000000),
];

const OUT_PressureConversionCube: &[(Item, f32)] = &[(Item::PressureConversionCube, 1.000000)];

const IN_PriorityPowerSwitch: &[(Item, f32)] = &[
    (Item::CircuitBoard, 240.000000),
    (Item::SteelBeam, 360.000000),
    (Item::HighSpeedConnector, 120.000000),
];

const OUT_PriorityPowerSwitch: &[(Item, f32)] = &[];

const IN_ProgrammableSplitter: &[(Item, f32)] = &[
    (Item::HeavyModularFrame, 60.000000),
    (Item::Motor, 60.000000),
    (Item::Supercomputer, 60.000000),
];

const OUT_ProgrammableSplitter: &[(Item, f32)] = &[];

const IN_ProteinInhaler: &[(Item, f32)] =
    &[(Item::AlienProtein, 3.000000), (Item::BerylNut, 30.000000)];

const OUT_ProteinInhaler: &[(Item, f32)] = &[(Item::MedicinalInhaler, 3.000000)];

const IN_PulseNobelisk: &[(Item, f32)] = &[
    (Item::Nobelisk, 5.000000),
    (Item::CrystalOscillator, 1.000000),
];

const OUT_PulseNobelisk: &[(Item, f32)] = &[(Item::PulseNobelisk, 5.000000)];

const IN_QuarterPipe: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_QuarterPipe: &[(Item, f32)] = &[];

const IN_QuartzCrystal: &[(Item, f32)] = &[(Item::RawQuartz, 37.500000)];

const OUT_QuartzCrystal: &[(Item, f32)] = &[(Item::QuartzCrystal, 22.500000)];

const IN_Quickwire: &[(Item, f32)] = &[(Item::CateriumIngot, 12.000000)];

const OUT_Quickwire: &[(Item, f32)] = &[(Item::Quickwire, 60.000000)];

const IN_RadarTower: &[(Item, f32)] = &[
    (Item::HeavyModularFrame, 1800.000000),
    (Item::CrystalOscillator, 1800.000000),
    (Item::CircuitBoard, 600.000000),
    (Item::Cable, 6000.000000),
];

const OUT_RadarTower: &[(Item, f32)] = &[];

const IN_RadioControlUnit: &[(Item, f32)] = &[
    (Item::AluminumCasing, 40.000000),
    (Item::CrystalOscillator, 1.250000),
    (Item::Computer, 1.250000),
];

const OUT_RadioControlUnit: &[(Item, f32)] = &[(Item::RadioControlUnit, 2.500000)];

const IN_Railway: &[(Item, f32)] = &[(Item::SteelPipe, 60.000000), (Item::SteelBeam, 60.000000)];

const OUT_Railway: &[(Item, f32)] = &[];

const IN_Ramp1m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_Ramp1m: &[(Item, f32)] = &[];

const IN_Ramp2m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_Ramp2m: &[(Item, f32)] = &[];

const IN_Ramp4m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_Ramp4m: &[(Item, f32)] = &[];

const IN_RampWall1m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_RampWall1m: &[(Item, f32)] = &[];

const IN_RampWall2m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_RampWall2m: &[(Item, f32)] = &[];

const IN_RampWall4m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_RampWall4m: &[(Item, f32)] = &[];

const IN_RampWall8m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_RampWall8m: &[(Item, f32)] = &[];

const IN_RebarGun: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 6.000000),
    (Item::IronRod, 16.000000),
    (Item::Screw, 100.000000),
];

const OUT_RebarGun: &[(Item, f32)] = &[(Item::RebarGun, 1.000000)];

const IN_Refinery: &[(Item, f32)] = &[
    (Item::Motor, 600.000000),
    (Item::EncasedIndustrialBeam, 600.000000),
    (Item::SteelPipe, 1800.000000),
    (Item::CopperSheet, 1200.000000),
];

const OUT_Refinery: &[(Item, f32)] = &[];

const IN_ReinforcedIronPlate: &[(Item, f32)] =
    &[(Item::IronPlate, 30.000000), (Item::Screw, 60.000000)];

const OUT_ReinforcedIronPlate: &[(Item, f32)] = &[(Item::ReinforcedIronPlate, 5.000000)];

const IN_ReinforcedWindow: &[(Item, f32)] = &[(Item::Silica, 120.000000)];

const OUT_ReinforcedWindow: &[(Item, f32)] = &[];

const IN_ResidualFuel: &[(Item, f32)] = &[(Item::HeavyOilResidue, 60.000000)];

const OUT_ResidualFuel: &[(Item, f32)] = &[(Item::Fuel, 40.000000)];

const IN_ResidualPlastic: &[(Item, f32)] =
    &[(Item::PolymerResin, 60.000000), (Item::Water, 20.000000)];

const OUT_ResidualPlastic: &[(Item, f32)] = &[(Item::Plastic, 20.000000)];

const IN_ResidualRubber: &[(Item, f32)] =
    &[(Item::PolymerResin, 40.000000), (Item::Water, 40.000000)];

const OUT_ResidualRubber: &[(Item, f32)] = &[(Item::Rubber, 20.000000)];

const IN_ResourceWellExtractor: &[(Item, f32)] =
    &[(Item::SteelBeam, 600.000000), (Item::Plastic, 600.000000)];

const OUT_ResourceWellExtractor: &[(Item, f32)] = &[];

const IN_ResourceWellPressurizer: &[(Item, f32)] = &[
    (Item::Wire, 12000.000000),
    (Item::Rubber, 3000.000000),
    (Item::EncasedIndustrialBeam, 3000.000000),
    (Item::Motor, 3000.000000),
];

const OUT_ResourceWellPressurizer: &[(Item, f32)] = &[];

const IN_Rifle: &[(Item, f32)] = &[
    (Item::Motor, 1.000000),
    (Item::Rubber, 5.000000),
    (Item::SteelPipe, 12.500000),
    (Item::Screw, 125.000000),
];

const OUT_Rifle: &[(Item, f32)] = &[(Item::Rifle, 0.500000)];

const IN_RifleAmmo: &[(Item, f32)] = &[
    (Item::CopperSheet, 15.000000),
    (Item::SmokelessPowder, 10.000000),
];

const OUT_RifleAmmo: &[(Item, f32)] = &[(Item::RifleAmmo, 75.000000)];

const IN_RoadBarrier: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_RoadBarrier: &[(Item, f32)] = &[];

const IN_Roof1m: &[(Item, f32)] = &[(Item::Concrete, 180.000000)];

const OUT_Roof1m: &[(Item, f32)] = &[];

const IN_Roof2m: &[(Item, f32)] = &[(Item::Concrete, 180.000000)];

const OUT_Roof2m: &[(Item, f32)] = &[];

const IN_Roof4m: &[(Item, f32)] = &[(Item::Concrete, 180.000000)];

const OUT_Roof4m: &[(Item, f32)] = &[];

const IN_RoofFlat: &[(Item, f32)] = &[(Item::Concrete, 180.000000)];

const OUT_RoofFlat: &[(Item, f32)] = &[];

const IN_Rotor: &[(Item, f32)] = &[(Item::IronRod, 20.000000), (Item::Screw, 100.000000)];

const OUT_Rotor: &[(Item, f32)] = &[(Item::Rotor, 4.000000)];

const IN_Rubber: &[(Item, f32)] = &[(Item::CrudeOil, 30.000000)];

const OUT_Rubber: &[(Item, f32)] = &[
    (Item::Rubber, 20.000000),
    (Item::HeavyOilResidue, 20.000000),
];

const IN_Screw: &[(Item, f32)] = &[(Item::IronRod, 10.000000)];

const OUT_Screw: &[(Item, f32)] = &[(Item::Screw, 40.000000)];

const IN_ShatterRebar: &[(Item, f32)] = &[
    (Item::IronRebar, 10.000000),
    (Item::QuartzCrystal, 15.000000),
];

const OUT_ShatterRebar: &[(Item, f32)] = &[(Item::ShatterRebar, 5.000000)];

const IN_SideDoorWall: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_SideDoorWall: &[(Item, f32)] = &[];

const IN_Silica: &[(Item, f32)] = &[(Item::RawQuartz, 22.500000)];

const OUT_Silica: &[(Item, f32)] = &[(Item::Silica, 37.500000)];

const IN_SingleWindow: &[(Item, f32)] = &[(Item::Silica, 120.000000)];

const OUT_SingleWindow: &[(Item, f32)] = &[];

const IN_SmallBillboard: &[(Item, f32)] = &[
    (Item::EncasedIndustrialBeam, 180.000000),
    (Item::CopperSheet, 240.000000),
    (Item::CrystalOscillator, 60.000000),
];

const OUT_SmallBillboard: &[(Item, f32)] = &[];

const IN_SmallConcretePillar: &[(Item, f32)] = &[(Item::Concrete, 180.000000)];

const OUT_SmallConcretePillar: &[(Item, f32)] = &[];

const IN_SmallFramePillar: &[(Item, f32)] = &[(Item::SteelBeam, 180.000000)];

const OUT_SmallFramePillar: &[(Item, f32)] = &[];

const IN_SmallMetalPillar: &[(Item, f32)] =
    &[(Item::IronPlate, 60.000000), (Item::Concrete, 120.000000)];

const OUT_SmallMetalPillar: &[(Item, f32)] = &[];

const IN_SmallPillarSupport: &[(Item, f32)] = &[(Item::Concrete, 180.000000)];

const OUT_SmallPillarSupport: &[(Item, f32)] = &[];

const IN_SmartPlating: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 2.000000),
    (Item::Rotor, 2.000000),
];

const OUT_SmartPlating: &[(Item, f32)] = &[(Item::SmartPlating, 2.000000)];

const IN_SmartSplitter: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 120.000000),
    (Item::Rotor, 120.000000),
    (Item::AILimiter, 60.000000),
];

const OUT_SmartSplitter: &[(Item, f32)] = &[];

const IN_Smelter: &[(Item, f32)] = &[(Item::IronRod, 300.000000), (Item::Wire, 480.000000)];

const OUT_Smelter: &[(Item, f32)] = &[];

const IN_SmokelessPowder: &[(Item, f32)] = &[
    (Item::BlackPowder, 20.000000),
    (Item::HeavyOilResidue, 10.000000),
];

const OUT_SmokelessPowder: &[(Item, f32)] = &[(Item::SmokelessPowder, 20.000000)];

const IN_Snowman: &[(Item, f32)] = &[];

const OUT_Snowman: &[(Item, f32)] = &[];

const IN_SolidBiofuel: &[(Item, f32)] = &[(Item::Biomass, 120.000000)];

const OUT_SolidBiofuel: &[(Item, f32)] = &[(Item::SolidBiofuel, 60.000000)];

const IN_SpaceElevator: &[(Item, f32)] = &[
    (Item::Concrete, 30000.000000),
    (Item::IronPlate, 15000.000000),
    (Item::IronRod, 24000.000000),
    (Item::Wire, 90000.000000),
];

const OUT_SpaceElevator: &[(Item, f32)] = &[];

const IN_SpitterProtein: &[(Item, f32)] = &[(Item::PlasmaSpitterRemains, 20.000000)];

const OUT_SpitterProtein: &[(Item, f32)] = &[(Item::AlienProtein, 20.000000)];

const IN_SquareSign05m: &[(Item, f32)] = &[
    (Item::IronPlate, 60.000000),
    (Item::QuartzCrystal, 60.000000),
];

const OUT_SquareSign05m: &[(Item, f32)] = &[];

const IN_SquareSign1m: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 60.000000),
    (Item::QuartzCrystal, 300.000000),
];

const OUT_SquareSign1m: &[(Item, f32)] = &[];

const IN_SquareSign2m: &[(Item, f32)] = &[
    (Item::ReinforcedIronPlate, 180.000000),
    (Item::QuartzCrystal, 600.000000),
];

const OUT_SquareSign2m: &[(Item, f32)] = &[];

const IN_StackableConveyorPole: &[(Item, f32)] = &[
    (Item::IronRod, 120.000000),
    (Item::IronPlate, 120.000000),
    (Item::Concrete, 120.000000),
];

const OUT_StackableConveyorPole: &[(Item, f32)] = &[];

const IN_StackableHypertubeSupport: &[(Item, f32)] = &[
    (Item::IronPlate, 240.000000),
    (Item::IronRod, 120.000000),
    (Item::Concrete, 120.000000),
];

const OUT_StackableHypertubeSupport: &[(Item, f32)] = &[];

const IN_StackablePipelineSupport: &[(Item, f32)] = &[
    (Item::IronPlate, 240.000000),
    (Item::IronRod, 120.000000),
    (Item::Concrete, 120.000000),
];

const OUT_StackablePipelineSupport: &[(Item, f32)] = &[];

const IN_StairsLeft: &[(Item, f32)] =
    &[(Item::Concrete, 180.000000), (Item::IronPlate, 180.000000)];

const OUT_StairsLeft: &[(Item, f32)] = &[];

const IN_StairsRight: &[(Item, f32)] =
    &[(Item::Concrete, 180.000000), (Item::IronPlate, 180.000000)];

const OUT_StairsRight: &[(Item, f32)] = &[];

const IN_Stator: &[(Item, f32)] = &[(Item::SteelPipe, 15.000000), (Item::Wire, 40.000000)];

const OUT_Stator: &[(Item, f32)] = &[(Item::Stator, 5.000000)];

const IN_SteelBeam: &[(Item, f32)] = &[(Item::SteelIngot, 60.000000)];

const OUT_SteelBeam: &[(Item, f32)] = &[(Item::SteelBeam, 15.000000)];

const IN_SteelIngot: &[(Item, f32)] = &[(Item::IronOre, 45.000000), (Item::Coal, 45.000000)];

const OUT_SteelIngot: &[(Item, f32)] = &[(Item::SteelIngot, 45.000000)];

const IN_SteelPipe: &[(Item, f32)] = &[(Item::SteelIngot, 30.000000)];

const OUT_SteelPipe: &[(Item, f32)] = &[(Item::SteelPipe, 20.000000)];

const IN_StingerProtein: &[(Item, f32)] = &[(Item::StingerRemains, 20.000000)];

const OUT_StingerProtein: &[(Item, f32)] = &[(Item::AlienProtein, 20.000000)];

const IN_StorageContainer: &[(Item, f32)] =
    &[(Item::IronPlate, 600.000000), (Item::IronRod, 600.000000)];

const OUT_StorageContainer: &[(Item, f32)] = &[];

const IN_StreetLight: &[(Item, f32)] = &[
    (Item::Quickwire, 600.000000),
    (Item::Wire, 240.000000),
    (Item::IronRod, 240.000000),
];

const OUT_StreetLight: &[(Item, f32)] = &[];

const IN_StunRebar: &[(Item, f32)] = &[(Item::IronRebar, 10.000000), (Item::Quickwire, 50.000000)];

const OUT_StunRebar: &[(Item, f32)] = &[(Item::StunRebar, 10.000000)];

const IN_SulfuricAcid: &[(Item, f32)] = &[(Item::Sulfur, 50.000000), (Item::Water, 50.000000)];

const OUT_SulfuricAcid: &[(Item, f32)] = &[(Item::SulfuricAcid, 50.000000)];

const IN_Supercomputer: &[(Item, f32)] = &[
    (Item::Computer, 3.750000),
    (Item::AILimiter, 3.750000),
    (Item::HighSpeedConnector, 5.625000),
    (Item::Plastic, 52.500000),
];

const OUT_Supercomputer: &[(Item, f32)] = &[(Item::Supercomputer, 1.875000)];

const IN_TheHUB: &[(Item, f32)] = &[(Item::HUBParts, 60.000000)];

const OUT_TheHUB: &[(Item, f32)] = &[];

const IN_TherapeuticInhaler: &[(Item, f32)] = &[
    (Item::Mycelia, 45.000000),
    (Item::AlienProtein, 3.000000),
    (Item::BaconAgaric, 3.000000),
];

const OUT_TherapeuticInhaler: &[(Item, f32)] = &[(Item::MedicinalInhaler, 3.000000)];

const IN_ThermalPropulsionRocket: &[(Item, f32)] = &[
    (Item::ModularEngine, 2.500000),
    (Item::TurboMotor, 1.000000),
    (Item::CoolingSystem, 3.000000),
    (Item::FusedModularFrame, 1.000000),
];

const OUT_ThermalPropulsionRocket: &[(Item, f32)] = &[(Item::ThermalPropulsionRocket, 1.000000)];

const IN_TiltedConcaveWall4m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_TiltedConcaveWall4m: &[(Item, f32)] = &[];

const IN_TiltedConcaveWall8m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_TiltedConcaveWall8m: &[(Item, f32)] = &[];

const IN_TiltedCornerWall4m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_TiltedCornerWall4m: &[(Item, f32)] = &[];

const IN_TiltedCornerWall8m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_TiltedCornerWall8m: &[(Item, f32)] = &[];

const IN_TiltedWall4m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_TiltedWall4m: &[(Item, f32)] = &[];

const IN_TiltedWall8m: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_TiltedWall8m: &[(Item, f32)] = &[];

const IN_Tractor: &[(Item, f32)] = &[
    (Item::ModularFrame, 300.000000),
    (Item::Rotor, 300.000000),
    (Item::ReinforcedIronPlate, 600.000000),
];

const OUT_Tractor: &[(Item, f32)] = &[];

const IN_TrainStation: &[(Item, f32)] = &[
    (Item::HeavyModularFrame, 240.000000),
    (Item::Computer, 480.000000),
    (Item::Concrete, 3000.000000),
    (Item::Cable, 1500.000000),
];

const OUT_TrainStation: &[(Item, f32)] = &[];

const IN_Truck: &[(Item, f32)] = &[
    (Item::Motor, 900.000000),
    (Item::CircuitBoard, 600.000000),
    (Item::HeavyModularFrame, 300.000000),
    (Item::Rubber, 3000.000000),
    (Item::EncasedIndustrialBeam, 1200.000000),
];

const OUT_Truck: &[(Item, f32)] = &[];

const IN_TruckStation: &[(Item, f32)] = &[
    (Item::ModularFrame, 900.000000),
    (Item::Rotor, 1200.000000),
    (Item::Cable, 3000.000000),
];

const OUT_TruckStation: &[(Item, f32)] = &[];

const IN_TurboMotor: &[(Item, f32)] = &[
    (Item::CoolingSystem, 7.500000),
    (Item::RadioControlUnit, 3.750000),
    (Item::Motor, 7.500000),
    (Item::Rubber, 45.000000),
];

const OUT_TurboMotor: &[(Item, f32)] = &[(Item::TurboMotor, 1.875000)];

const IN_TurboRifleAmmo: &[(Item, f32)] = &[
    (Item::RifleAmmo, 125.000000),
    (Item::AluminumCasing, 15.000000),
    (Item::PackagedTurbofuel, 15.000000),
];

const OUT_TurboRifleAmmo: &[(Item, f32)] = &[(Item::TurboRifleAmmo, 250.000000)];

const IN_Turbofuel: &[(Item, f32)] = &[(Item::Fuel, 22.500000), (Item::CompactedCoal, 15.000000)];

const OUT_Turbofuel: &[(Item, f32)] = &[(Item::Turbofuel, 18.750000)];

const IN_UJellyLandingPad: &[(Item, f32)] = &[
    (Item::Rotor, 120.000000),
    (Item::Cable, 1200.000000),
    (Item::Biomass, 12000.000000),
];

const OUT_UJellyLandingPad: &[(Item, f32)] = &[];

const IN_UnpackageAluminaSolution: &[(Item, f32)] = &[(Item::PackagedAluminaSolution, 120.000000)];

const OUT_UnpackageAluminaSolution: &[(Item, f32)] = &[
    (Item::AluminaSolution, 120.000000),
    (Item::EmptyCanister, 120.000000),
];

const IN_UnpackageFuel: &[(Item, f32)] = &[(Item::PackagedFuel, 60.000000)];

const OUT_UnpackageFuel: &[(Item, f32)] =
    &[(Item::Fuel, 60.000000), (Item::EmptyCanister, 60.000000)];

const IN_UnpackageHeavyOilResidue: &[(Item, f32)] = &[(Item::PackagedHeavyOilResidue, 20.000000)];

const OUT_UnpackageHeavyOilResidue: &[(Item, f32)] = &[
    (Item::HeavyOilResidue, 20.000000),
    (Item::EmptyCanister, 20.000000),
];

const IN_UnpackageLiquidBiofuel: &[(Item, f32)] = &[(Item::PackagedLiquidBiofuel, 60.000000)];

const OUT_UnpackageLiquidBiofuel: &[(Item, f32)] = &[
    (Item::LiquidBiofuel, 60.000000),
    (Item::EmptyCanister, 60.000000),
];

const IN_UnpackageNitricAcid: &[(Item, f32)] = &[(Item::PackagedNitricAcid, 20.000000)];

const OUT_UnpackageNitricAcid: &[(Item, f32)] = &[
    (Item::NitricAcid, 20.000000),
    (Item::EmptyFluidTank, 20.000000),
];

const IN_UnpackageNitrogenGas: &[(Item, f32)] = &[(Item::PackagedNitrogenGas, 60.000000)];

const OUT_UnpackageNitrogenGas: &[(Item, f32)] = &[
    (Item::NitrogenGas, 240.000000),
    (Item::EmptyFluidTank, 60.000000),
];

const IN_UnpackageOil: &[(Item, f32)] = &[(Item::PackagedOil, 60.000000)];

const OUT_UnpackageOil: &[(Item, f32)] = &[
    (Item::CrudeOil, 60.000000),
    (Item::EmptyCanister, 60.000000),
];

const IN_UnpackageSulfuricAcid: &[(Item, f32)] = &[(Item::PackagedSulfuricAcid, 60.000000)];

const OUT_UnpackageSulfuricAcid: &[(Item, f32)] = &[
    (Item::SulfuricAcid, 60.000000),
    (Item::EmptyCanister, 60.000000),
];

const IN_UnpackageTurbofuel: &[(Item, f32)] = &[(Item::PackagedTurbofuel, 20.000000)];

const OUT_UnpackageTurbofuel: &[(Item, f32)] = &[
    (Item::Turbofuel, 20.000000),
    (Item::EmptyCanister, 20.000000),
];

const IN_UnpackageWater: &[(Item, f32)] = &[(Item::PackagedWater, 120.000000)];

const OUT_UnpackageWater: &[(Item, f32)] =
    &[(Item::Water, 120.000000), (Item::EmptyCanister, 120.000000)];

const IN_UpCornerRamp1m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_UpCornerRamp1m: &[(Item, f32)] = &[];

const IN_UpCornerRamp2m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_UpCornerRamp2m: &[(Item, f32)] = &[];

const IN_UpCornerRamp4m: &[(Item, f32)] = &[(Item::Concrete, 300.000000)];

const OUT_UpCornerRamp4m: &[(Item, f32)] = &[];

const IN_UraniumFuelRod: &[(Item, f32)] = &[
    (Item::EncasedUraniumCell, 20.000000),
    (Item::EncasedIndustrialBeam, 1.200000),
    (Item::ElectromagneticControlRod, 2.000000),
];

const OUT_UraniumFuelRod: &[(Item, f32)] = &[(Item::UraniumFuelRod, 0.400000)];

const IN_Valve: &[(Item, f32)] = &[(Item::Rubber, 240.000000), (Item::SteelBeam, 240.000000)];

const OUT_Valve: &[(Item, f32)] = &[];

const IN_VersatileFramework: &[(Item, f32)] =
    &[(Item::ModularFrame, 2.500000), (Item::SteelBeam, 30.000000)];

const OUT_VersatileFramework: &[(Item, f32)] = &[(Item::VersatileFramework, 5.000000)];

const IN_VitaminInhaler: &[(Item, f32)] =
    &[(Item::Mycelia, 30.000000), (Item::Paleberry, 15.000000)];

const OUT_VitaminInhaler: &[(Item, f32)] = &[(Item::MedicinalInhaler, 3.000000)];

const IN_Wall1a: &[(Item, f32)] = &[(Item::Concrete, 120.000000)];

const OUT_Wall1a: &[(Item, f32)] = &[];

const IN_WallMountedFloodLight: &[(Item, f32)] = &[
    (Item::Quickwire, 1500.000000),
    (Item::CopperSheet, 240.000000),
    (Item::EncasedIndustrialBeam, 120.000000),
];

const OUT_WallMountedFloodLight: &[(Item, f32)] = &[];

const IN_WallOutletMk1: &[(Item, f32)] = &[(Item::Wire, 240.000000), (Item::IronRod, 60.000000)];

const OUT_WallOutletMk1: &[(Item, f32)] = &[];

const IN_WallOutletMk2: &[(Item, f32)] =
    &[(Item::Quickwire, 480.000000), (Item::IronRod, 120.000000)];

const OUT_WallOutletMk2: &[(Item, f32)] = &[];

const IN_WallOutletMk3: &[(Item, f32)] = &[
    (Item::HighSpeedConnector, 180.000000),
    (Item::SteelPipe, 180.000000),
];

const OUT_WallOutletMk3: &[(Item, f32)] = &[];

const IN_WaterExtractor: &[(Item, f32)] = &[
    (Item::CopperSheet, 1200.000000),
    (Item::ReinforcedIronPlate, 600.000000),
    (Item::Rotor, 600.000000),
];

const OUT_WaterExtractor: &[(Item, f32)] = &[];

const IN_Wire: &[(Item, f32)] = &[(Item::CopperIngot, 15.000000)];

const OUT_Wire: &[(Item, f32)] = &[(Item::Wire, 30.000000)];

const IN_XenoBasher: &[(Item, f32)] = &[
    (Item::ModularFrame, 3.750000),
    (Item::XenoZapper, 1.500000),
    (Item::Cable, 18.750000),
    (Item::Wire, 375.000000),
];

const OUT_XenoBasher: &[(Item, f32)] = &[(Item::XenoBasher, 0.750000)];

const IN_XenoZapper: &[(Item, f32)] = &[
    (Item::IronRod, 15.000000),
    (Item::ReinforcedIronPlate, 3.000000),
    (Item::Cable, 22.500000),
    (Item::Wire, 75.000000),
];

const OUT_XenoZapper: &[(Item, f32)] = &[(Item::XenoZapper, 1.500000)];

const IN_Zipline: &[(Item, f32)] = &[
    (Item::XenoZapper, 1.500000),
    (Item::Quickwire, 45.000000),
    (Item::IronRod, 4.500000),
    (Item::Cable, 15.000000),
];

const OUT_Zipline: &[(Item, f32)] = &[(Item::Zipline, 1.500000)];

pub static RECIPE_INFOS: &[RecipeInfo] = &[
    RecipeInfo {
        recipe: Recipe::AILimiter,
        name: "AI Limiter",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AILimiter,
        outputs: OUT_AILimiter,
    },
    RecipeInfo {
        recipe: Recipe::AWESOMEShop,
        name: "AWESOME Shop",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AWESOMEShop,
        outputs: OUT_AWESOMEShop,
    },
    RecipeInfo {
        recipe: Recipe::AWESOMESink,
        name: "AWESOME Sink",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AWESOMESink,
        outputs: OUT_AWESOMESink,
    },
    RecipeInfo {
        recipe: Recipe::AdaptiveControlUnit,
        name: "Adaptive Control Unit",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AdaptiveControlUnit,
        outputs: OUT_AdaptiveControlUnit,
    },
    RecipeInfo {
        recipe: Recipe::AlcladAluminumSheet,
        name: "Alclad Aluminum Sheet",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlcladAluminumSheet,
        outputs: OUT_AlcladAluminumSheet,
    },
    RecipeInfo {
        recipe: Recipe::AlienDNACapsule,
        name: "Alien DNA Capsule",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AlienDNACapsule,
        outputs: OUT_AlienDNACapsule,
    },
    RecipeInfo {
        recipe: Recipe::AlternateAdheredIronPlate,
        name: "Alternate: Adhered Iron Plate",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateAdheredIronPlate,
        outputs: OUT_AlternateAdheredIronPlate,
    },
    RecipeInfo {
        recipe: Recipe::AlternateAlcladCasing,
        name: "Alternate: Alclad Casing",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateAlcladCasing,
        outputs: OUT_AlternateAlcladCasing,
    },
    RecipeInfo {
        recipe: Recipe::AlternateAutomatedMiner,
        name: "Alternate: Automated Miner",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateAutomatedMiner,
        outputs: OUT_AlternateAutomatedMiner,
    },
    RecipeInfo {
        recipe: Recipe::AlternateAutomatedSpeedWiring,
        name: "Alternate: Automated Speed Wiring",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateAutomatedSpeedWiring,
        outputs: OUT_AlternateAutomatedSpeedWiring,
    },
    RecipeInfo {
        recipe: Recipe::AlternateBiocoal,
        name: "Alternate: Biocoal",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AlternateBiocoal,
        outputs: OUT_AlternateBiocoal,
    },
    RecipeInfo {
        recipe: Recipe::AlternateBoltedFrame,
        name: "Alternate: Bolted Frame",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateBoltedFrame,
        outputs: OUT_AlternateBoltedFrame,
    },
    RecipeInfo {
        recipe: Recipe::AlternateBoltedIronPlate,
        name: "Alternate: Bolted Iron Plate",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateBoltedIronPlate,
        outputs: OUT_AlternateBoltedIronPlate,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCastScrew,
        name: "Alternate: Cast Screw",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AlternateCastScrew,
        outputs: OUT_AlternateCastScrew,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCateriumCircuitBoard,
        name: "Alternate: Caterium Circuit Board",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateCateriumCircuitBoard,
        outputs: OUT_AlternateCateriumCircuitBoard,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCateriumComputer,
        name: "Alternate: Caterium Computer",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateCateriumComputer,
        outputs: OUT_AlternateCateriumComputer,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCateriumWire,
        name: "Alternate: Caterium Wire",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AlternateCateriumWire,
        outputs: OUT_AlternateCateriumWire,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCharcoal,
        name: "Alternate: Charcoal",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AlternateCharcoal,
        outputs: OUT_AlternateCharcoal,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCheapSilica,
        name: "Alternate: Cheap Silica",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateCheapSilica,
        outputs: OUT_AlternateCheapSilica,
    },
    RecipeInfo {
        recipe: Recipe::AlternateClassicBattery,
        name: "Alternate: Classic Battery",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateClassicBattery,
        outputs: OUT_AlternateClassicBattery,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCoatedCable,
        name: "Alternate: Coated Cable",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternateCoatedCable,
        outputs: OUT_AlternateCoatedCable,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCoatedIronCanister,
        name: "Alternate: Coated Iron Canister",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateCoatedIronCanister,
        outputs: OUT_AlternateCoatedIronCanister,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCoatedIronPlate,
        name: "Alternate: Coated Iron Plate",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateCoatedIronPlate,
        outputs: OUT_AlternateCoatedIronPlate,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCokeSteelIngot,
        name: "Alternate: Coke Steel Ingot",
        machine: crate::models::types::MachineType::Foundry,
        inputs: IN_AlternateCokeSteelIngot,
        outputs: OUT_AlternateCokeSteelIngot,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCompactedCoal,
        name: "Alternate: Compacted Coal",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateCompactedCoal,
        outputs: OUT_AlternateCompactedCoal,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCompactedSteelIngot,
        name: "Alternate: Compacted Steel Ingot",
        machine: crate::models::types::MachineType::Foundry,
        inputs: IN_AlternateCompactedSteelIngot,
        outputs: OUT_AlternateCompactedSteelIngot,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCoolingDevice,
        name: "Alternate: Cooling Device",
        machine: crate::models::types::MachineType::Blender,
        inputs: IN_AlternateCoolingDevice,
        outputs: OUT_AlternateCoolingDevice,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCopperAlloyIngot,
        name: "Alternate: Copper Alloy Ingot",
        machine: crate::models::types::MachineType::Foundry,
        inputs: IN_AlternateCopperAlloyIngot,
        outputs: OUT_AlternateCopperAlloyIngot,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCopperRotor,
        name: "Alternate: Copper Rotor",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateCopperRotor,
        outputs: OUT_AlternateCopperRotor,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCrystalBeacon,
        name: "Alternate: Crystal Beacon",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateCrystalBeacon,
        outputs: OUT_AlternateCrystalBeacon,
    },
    RecipeInfo {
        recipe: Recipe::AlternateCrystalComputer,
        name: "Alternate: Crystal Computer",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateCrystalComputer,
        outputs: OUT_AlternateCrystalComputer,
    },
    RecipeInfo {
        recipe: Recipe::AlternateDilutedFuel,
        name: "Alternate: Diluted Fuel",
        machine: crate::models::types::MachineType::Blender,
        inputs: IN_AlternateDilutedFuel,
        outputs: OUT_AlternateDilutedFuel,
    },
    RecipeInfo {
        recipe: Recipe::AlternateDilutedPackagedFuel,
        name: "Alternate: Diluted Packaged Fuel",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternateDilutedPackagedFuel,
        outputs: OUT_AlternateDilutedPackagedFuel,
    },
    RecipeInfo {
        recipe: Recipe::AlternateElectricMotor,
        name: "Alternate: Electric Motor",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateElectricMotor,
        outputs: OUT_AlternateElectricMotor,
    },
    RecipeInfo {
        recipe: Recipe::AlternateElectrodeAluminumScrap,
        name: "Alternate: Electrode Aluminum Scrap",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternateElectrodeAluminumScrap,
        outputs: OUT_AlternateElectrodeAluminumScrap,
    },
    RecipeInfo {
        recipe: Recipe::AlternateElectrodeCircuitBoard,
        name: "Alternate: Electrode Circuit Board",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateElectrodeCircuitBoard,
        outputs: OUT_AlternateElectrodeCircuitBoard,
    },
    RecipeInfo {
        recipe: Recipe::AlternateElectromagneticConnectionRod,
        name: "Alternate: Electromagnetic Connection Rod",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateElectromagneticConnectionRod,
        outputs: OUT_AlternateElectromagneticConnectionRod,
    },
    RecipeInfo {
        recipe: Recipe::AlternateEncasedIndustrialPipe,
        name: "Alternate: Encased Industrial Pipe",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateEncasedIndustrialPipe,
        outputs: OUT_AlternateEncasedIndustrialPipe,
    },
    RecipeInfo {
        recipe: Recipe::AlternateFertileUranium,
        name: "Alternate: Fertile Uranium",
        machine: crate::models::types::MachineType::Blender,
        inputs: IN_AlternateFertileUranium,
        outputs: OUT_AlternateFertileUranium,
    },
    RecipeInfo {
        recipe: Recipe::AlternateFineBlackPowder,
        name: "Alternate: Fine Black Powder",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateFineBlackPowder,
        outputs: OUT_AlternateFineBlackPowder,
    },
    RecipeInfo {
        recipe: Recipe::AlternateFineConcrete,
        name: "Alternate: Fine Concrete",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateFineConcrete,
        outputs: OUT_AlternateFineConcrete,
    },
    RecipeInfo {
        recipe: Recipe::AlternateFlexibleFramework,
        name: "Alternate: Flexible Framework",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateFlexibleFramework,
        outputs: OUT_AlternateFlexibleFramework,
    },
    RecipeInfo {
        recipe: Recipe::AlternateFusedQuickwire,
        name: "Alternate: Fused Quickwire",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateFusedQuickwire,
        outputs: OUT_AlternateFusedQuickwire,
    },
    RecipeInfo {
        recipe: Recipe::AlternateFusedWire,
        name: "Alternate: Fused Wire",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateFusedWire,
        outputs: OUT_AlternateFusedWire,
    },
    RecipeInfo {
        recipe: Recipe::AlternateHeatExchanger,
        name: "Alternate: Heat Exchanger",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateHeatExchanger,
        outputs: OUT_AlternateHeatExchanger,
    },
    RecipeInfo {
        recipe: Recipe::AlternateHeatFusedFrame,
        name: "Alternate: Heat-Fused Frame",
        machine: crate::models::types::MachineType::Blender,
        inputs: IN_AlternateHeatFusedFrame,
        outputs: OUT_AlternateHeatFusedFrame,
    },
    RecipeInfo {
        recipe: Recipe::AlternateHeavyEncasedFrame,
        name: "Alternate: Heavy Encased Frame",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateHeavyEncasedFrame,
        outputs: OUT_AlternateHeavyEncasedFrame,
    },
    RecipeInfo {
        recipe: Recipe::AlternateHeavyFlexibleFrame,
        name: "Alternate: Heavy Flexible Frame",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateHeavyFlexibleFrame,
        outputs: OUT_AlternateHeavyFlexibleFrame,
    },
    RecipeInfo {
        recipe: Recipe::AlternateHeavyOilResidue,
        name: "Alternate: Heavy Oil Residue",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternateHeavyOilResidue,
        outputs: OUT_AlternateHeavyOilResidue,
    },
    RecipeInfo {
        recipe: Recipe::AlternateInfusedUraniumCell,
        name: "Alternate: Infused Uranium Cell",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateInfusedUraniumCell,
        outputs: OUT_AlternateInfusedUraniumCell,
    },
    RecipeInfo {
        recipe: Recipe::AlternateInstantPlutoniumCell,
        name: "Alternate: Instant Plutonium Cell",
        machine: crate::models::types::MachineType::ParticleAccelerator,
        inputs: IN_AlternateInstantPlutoniumCell,
        outputs: OUT_AlternateInstantPlutoniumCell,
    },
    RecipeInfo {
        recipe: Recipe::AlternateInstantScrap,
        name: "Alternate: Instant Scrap",
        machine: crate::models::types::MachineType::Blender,
        inputs: IN_AlternateInstantScrap,
        outputs: OUT_AlternateInstantScrap,
    },
    RecipeInfo {
        recipe: Recipe::AlternateInsulatedCable,
        name: "Alternate: Insulated Cable",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateInsulatedCable,
        outputs: OUT_AlternateInsulatedCable,
    },
    RecipeInfo {
        recipe: Recipe::AlternateInsulatedCrystalOscillator,
        name: "Alternate: Insulated Crystal Oscillator",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateInsulatedCrystalOscillator,
        outputs: OUT_AlternateInsulatedCrystalOscillator,
    },
    RecipeInfo {
        recipe: Recipe::AlternateIronAlloyIngot,
        name: "Alternate: Iron Alloy Ingot",
        machine: crate::models::types::MachineType::Foundry,
        inputs: IN_AlternateIronAlloyIngot,
        outputs: OUT_AlternateIronAlloyIngot,
    },
    RecipeInfo {
        recipe: Recipe::AlternateIronWire,
        name: "Alternate: Iron Wire",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AlternateIronWire,
        outputs: OUT_AlternateIronWire,
    },
    RecipeInfo {
        recipe: Recipe::AlternateOCSupercomputer,
        name: "Alternate: OC Supercomputer",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateOCSupercomputer,
        outputs: OUT_AlternateOCSupercomputer,
    },
    RecipeInfo {
        recipe: Recipe::AlternatePlasticSmartPlating,
        name: "Alternate: Plastic Smart Plating",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternatePlasticSmartPlating,
        outputs: OUT_AlternatePlasticSmartPlating,
    },
    RecipeInfo {
        recipe: Recipe::AlternatePlutoniumFuelUnit,
        name: "Alternate: Plutonium Fuel Unit",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternatePlutoniumFuelUnit,
        outputs: OUT_AlternatePlutoniumFuelUnit,
    },
    RecipeInfo {
        recipe: Recipe::AlternatePolyesterFabric,
        name: "Alternate: Polyester Fabric",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternatePolyesterFabric,
        outputs: OUT_AlternatePolyesterFabric,
    },
    RecipeInfo {
        recipe: Recipe::AlternatePolymerResin,
        name: "Alternate: Polymer Resin",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternatePolymerResin,
        outputs: OUT_AlternatePolymerResin,
    },
    RecipeInfo {
        recipe: Recipe::AlternatePureAluminumIngot,
        name: "Alternate: Pure Aluminum Ingot",
        machine: crate::models::types::MachineType::Smelter,
        inputs: IN_AlternatePureAluminumIngot,
        outputs: OUT_AlternatePureAluminumIngot,
    },
    RecipeInfo {
        recipe: Recipe::AlternatePureCateriumIngot,
        name: "Alternate: Pure Caterium Ingot",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternatePureCateriumIngot,
        outputs: OUT_AlternatePureCateriumIngot,
    },
    RecipeInfo {
        recipe: Recipe::AlternatePureCopperIngot,
        name: "Alternate: Pure Copper Ingot",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternatePureCopperIngot,
        outputs: OUT_AlternatePureCopperIngot,
    },
    RecipeInfo {
        recipe: Recipe::AlternatePureIronIngot,
        name: "Alternate: Pure Iron Ingot",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternatePureIronIngot,
        outputs: OUT_AlternatePureIronIngot,
    },
    RecipeInfo {
        recipe: Recipe::AlternatePureQuartzCrystal,
        name: "Alternate: Pure Quartz Crystal",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternatePureQuartzCrystal,
        outputs: OUT_AlternatePureQuartzCrystal,
    },
    RecipeInfo {
        recipe: Recipe::AlternateQuickwireCable,
        name: "Alternate: Quickwire Cable",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateQuickwireCable,
        outputs: OUT_AlternateQuickwireCable,
    },
    RecipeInfo {
        recipe: Recipe::AlternateQuickwireStator,
        name: "Alternate: Quickwire Stator",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateQuickwireStator,
        outputs: OUT_AlternateQuickwireStator,
    },
    RecipeInfo {
        recipe: Recipe::AlternateRadioConnectionUnit,
        name: "Alternate: Radio Connection Unit",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateRadioConnectionUnit,
        outputs: OUT_AlternateRadioConnectionUnit,
    },
    RecipeInfo {
        recipe: Recipe::AlternateRadioControlSystem,
        name: "Alternate: Radio Control System",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateRadioControlSystem,
        outputs: OUT_AlternateRadioControlSystem,
    },
    RecipeInfo {
        recipe: Recipe::AlternateRecycledPlastic,
        name: "Alternate: Recycled Plastic",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternateRecycledPlastic,
        outputs: OUT_AlternateRecycledPlastic,
    },
    RecipeInfo {
        recipe: Recipe::AlternateRecycledRubber,
        name: "Alternate: Recycled Rubber",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternateRecycledRubber,
        outputs: OUT_AlternateRecycledRubber,
    },
    RecipeInfo {
        recipe: Recipe::AlternateRigourMotor,
        name: "Alternate: Rigour Motor",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateRigourMotor,
        outputs: OUT_AlternateRigourMotor,
    },
    RecipeInfo {
        recipe: Recipe::AlternateRubberConcrete,
        name: "Alternate: Rubber Concrete",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateRubberConcrete,
        outputs: OUT_AlternateRubberConcrete,
    },
    RecipeInfo {
        recipe: Recipe::AlternateSiliconCircuitBoard,
        name: "Alternate: Silicon Circuit Board",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateSiliconCircuitBoard,
        outputs: OUT_AlternateSiliconCircuitBoard,
    },
    RecipeInfo {
        recipe: Recipe::AlternateSiliconHighSpeedConnector,
        name: "Alternate: Silicon High-Speed Connector",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateSiliconHighSpeedConnector,
        outputs: OUT_AlternateSiliconHighSpeedConnector,
    },
    RecipeInfo {
        recipe: Recipe::AlternateSloppyAlumina,
        name: "Alternate: Sloppy Alumina",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternateSloppyAlumina,
        outputs: OUT_AlternateSloppyAlumina,
    },
    RecipeInfo {
        recipe: Recipe::AlternateSolidSteelIngot,
        name: "Alternate: Solid Steel Ingot",
        machine: crate::models::types::MachineType::Foundry,
        inputs: IN_AlternateSolidSteelIngot,
        outputs: OUT_AlternateSolidSteelIngot,
    },
    RecipeInfo {
        recipe: Recipe::AlternateSteamedCopperSheet,
        name: "Alternate: Steamed Copper Sheet",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternateSteamedCopperSheet,
        outputs: OUT_AlternateSteamedCopperSheet,
    },
    RecipeInfo {
        recipe: Recipe::AlternateSteelCanister,
        name: "Alternate: Steel Canister",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AlternateSteelCanister,
        outputs: OUT_AlternateSteelCanister,
    },
    RecipeInfo {
        recipe: Recipe::AlternateSteelCoatedPlate,
        name: "Alternate: Steel Coated Plate",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateSteelCoatedPlate,
        outputs: OUT_AlternateSteelCoatedPlate,
    },
    RecipeInfo {
        recipe: Recipe::AlternateSteelRod,
        name: "Alternate: Steel Rod",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AlternateSteelRod,
        outputs: OUT_AlternateSteelRod,
    },
    RecipeInfo {
        recipe: Recipe::AlternateSteelRotor,
        name: "Alternate: Steel Rotor",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateSteelRotor,
        outputs: OUT_AlternateSteelRotor,
    },
    RecipeInfo {
        recipe: Recipe::AlternateSteelScrew,
        name: "Alternate: Steel Screw",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AlternateSteelScrew,
        outputs: OUT_AlternateSteelScrew,
    },
    RecipeInfo {
        recipe: Recipe::AlternateSteeledFrame,
        name: "Alternate: Steeled Frame",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateSteeledFrame,
        outputs: OUT_AlternateSteeledFrame,
    },
    RecipeInfo {
        recipe: Recipe::AlternateStitchedIronPlate,
        name: "Alternate: Stitched Iron Plate",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AlternateStitchedIronPlate,
        outputs: OUT_AlternateStitchedIronPlate,
    },
    RecipeInfo {
        recipe: Recipe::AlternateSuperStateComputer,
        name: "Alternate: Super-State Computer",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateSuperStateComputer,
        outputs: OUT_AlternateSuperStateComputer,
    },
    RecipeInfo {
        recipe: Recipe::AlternateTurboBlendFuel,
        name: "Alternate: Turbo Blend Fuel",
        machine: crate::models::types::MachineType::Blender,
        inputs: IN_AlternateTurboBlendFuel,
        outputs: OUT_AlternateTurboBlendFuel,
    },
    RecipeInfo {
        recipe: Recipe::AlternateTurboElectricMotor,
        name: "Alternate: Turbo Electric Motor",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateTurboElectricMotor,
        outputs: OUT_AlternateTurboElectricMotor,
    },
    RecipeInfo {
        recipe: Recipe::AlternateTurboHeavyFuel,
        name: "Alternate: Turbo Heavy Fuel",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternateTurboHeavyFuel,
        outputs: OUT_AlternateTurboHeavyFuel,
    },
    RecipeInfo {
        recipe: Recipe::AlternateTurboPressureMotor,
        name: "Alternate: Turbo Pressure Motor",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateTurboPressureMotor,
        outputs: OUT_AlternateTurboPressureMotor,
    },
    RecipeInfo {
        recipe: Recipe::AlternateUraniumFuelUnit,
        name: "Alternate: Uranium Fuel Unit",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_AlternateUraniumFuelUnit,
        outputs: OUT_AlternateUraniumFuelUnit,
    },
    RecipeInfo {
        recipe: Recipe::AlternateWetConcrete,
        name: "Alternate: Wet Concrete",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AlternateWetConcrete,
        outputs: OUT_AlternateWetConcrete,
    },
    RecipeInfo {
        recipe: Recipe::AluminaSolution,
        name: "Alumina Solution",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AluminaSolution,
        outputs: OUT_AluminaSolution,
    },
    RecipeInfo {
        recipe: Recipe::AluminumCasing,
        name: "Aluminum Casing",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AluminumCasing,
        outputs: OUT_AluminumCasing,
    },
    RecipeInfo {
        recipe: Recipe::AluminumIngot,
        name: "Aluminum Ingot",
        machine: crate::models::types::MachineType::Foundry,
        inputs: IN_AluminumIngot,
        outputs: OUT_AluminumIngot,
    },
    RecipeInfo {
        recipe: Recipe::AluminumScrap,
        name: "Aluminum Scrap",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_AluminumScrap,
        outputs: OUT_AluminumScrap,
    },
    RecipeInfo {
        recipe: Recipe::Assembler,
        name: "Assembler",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Assembler,
        outputs: OUT_Assembler,
    },
    RecipeInfo {
        recipe: Recipe::AssemblyDirectorSystem,
        name: "Assembly Director System",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AssemblyDirectorSystem,
        outputs: OUT_AssemblyDirectorSystem,
    },
    RecipeInfo {
        recipe: Recipe::AutomatedGate,
        name: "Automated Gate",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_AutomatedGate,
        outputs: OUT_AutomatedGate,
    },
    RecipeInfo {
        recipe: Recipe::AutomatedWiring,
        name: "Automated Wiring",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_AutomatedWiring,
        outputs: OUT_AutomatedWiring,
    },
    RecipeInfo {
        recipe: Recipe::BasicWall1m,
        name: "Basic Wall 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BasicWall1m,
        outputs: OUT_BasicWall1m,
    },
    RecipeInfo {
        recipe: Recipe::BasicWall4m,
        name: "Basic Wall 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BasicWall4m,
        outputs: OUT_BasicWall4m,
    },
    RecipeInfo {
        recipe: Recipe::Battery,
        name: "Battery",
        machine: crate::models::types::MachineType::Blender,
        inputs: IN_Battery,
        outputs: OUT_Battery,
    },
    RecipeInfo {
        recipe: Recipe::Beacon,
        name: "Beacon",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_Beacon,
        outputs: OUT_Beacon,
    },
    RecipeInfo {
        recipe: Recipe::BeamConnector,
        name: "Beam Connector",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BeamConnector,
        outputs: OUT_BeamConnector,
    },
    RecipeInfo {
        recipe: Recipe::BeamConnectorDouble,
        name: "Beam Connector Double",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BeamConnectorDouble,
        outputs: OUT_BeamConnectorDouble,
    },
    RecipeInfo {
        recipe: Recipe::BeamSupport,
        name: "Beam Support",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BeamSupport,
        outputs: OUT_BeamSupport,
    },
    RecipeInfo {
        recipe: Recipe::BigConcretePillar,
        name: "Big Concrete Pillar",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BigConcretePillar,
        outputs: OUT_BigConcretePillar,
    },
    RecipeInfo {
        recipe: Recipe::BigFramePillar,
        name: "Big Frame Pillar",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BigFramePillar,
        outputs: OUT_BigFramePillar,
    },
    RecipeInfo {
        recipe: Recipe::BigMetalPillar,
        name: "Big Metal Pillar",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BigMetalPillar,
        outputs: OUT_BigMetalPillar,
    },
    RecipeInfo {
        recipe: Recipe::BigPillarSupport,
        name: "Big Pillar Support",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BigPillarSupport,
        outputs: OUT_BigPillarSupport,
    },
    RecipeInfo {
        recipe: Recipe::BiomassAlienProtein,
        name: "Biomass (Alien Protein)",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BiomassAlienProtein,
        outputs: OUT_BiomassAlienProtein,
    },
    RecipeInfo {
        recipe: Recipe::BiomassBurner,
        name: "Biomass Burner",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BiomassBurner,
        outputs: OUT_BiomassBurner,
    },
    RecipeInfo {
        recipe: Recipe::BiomassLeaves,
        name: "Biomass (Leaves)",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BiomassLeaves,
        outputs: OUT_BiomassLeaves,
    },
    RecipeInfo {
        recipe: Recipe::BiomassMycelia,
        name: "Biomass (Mycelia)",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BiomassMycelia,
        outputs: OUT_BiomassMycelia,
    },
    RecipeInfo {
        recipe: Recipe::BiomassWood,
        name: "Biomass (Wood)",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BiomassWood,
        outputs: OUT_BiomassWood,
    },
    RecipeInfo {
        recipe: Recipe::BlackPowder,
        name: "Black Powder",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_BlackPowder,
        outputs: OUT_BlackPowder,
    },
    RecipeInfo {
        recipe: Recipe::BladeRunners,
        name: "Blade Runners",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BladeRunners,
        outputs: OUT_BladeRunners,
    },
    RecipeInfo {
        recipe: Recipe::Blender,
        name: "Blender",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Blender,
        outputs: OUT_Blender,
    },
    RecipeInfo {
        recipe: Recipe::BlockSignal,
        name: "Block Signal",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BlockSignal,
        outputs: OUT_BlockSignal,
    },
    RecipeInfo {
        recipe: Recipe::BlueprintDesigner,
        name: "Blueprint Designer",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_BlueprintDesigner,
        outputs: OUT_BlueprintDesigner,
    },
    RecipeInfo {
        recipe: Recipe::Cable,
        name: "Cable",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Cable,
        outputs: OUT_Cable,
    },
    RecipeInfo {
        recipe: Recipe::CandyCane,
        name: "Candy Cane",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CandyCane,
        outputs: OUT_CandyCane,
    },
    RecipeInfo {
        recipe: Recipe::CateriumIngot,
        name: "Caterium Ingot",
        machine: crate::models::types::MachineType::Smelter,
        inputs: IN_CateriumIngot,
        outputs: OUT_CateriumIngot,
    },
    RecipeInfo {
        recipe: Recipe::CatwalkCorner,
        name: "Catwalk Corner",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CatwalkCorner,
        outputs: OUT_CatwalkCorner,
    },
    RecipeInfo {
        recipe: Recipe::CatwalkCrossing,
        name: "Catwalk Crossing",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CatwalkCrossing,
        outputs: OUT_CatwalkCrossing,
    },
    RecipeInfo {
        recipe: Recipe::CatwalkRamp,
        name: "Catwalk Ramp",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CatwalkRamp,
        outputs: OUT_CatwalkRamp,
    },
    RecipeInfo {
        recipe: Recipe::CatwalkStairs,
        name: "Catwalk Stairs",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CatwalkStairs,
        outputs: OUT_CatwalkStairs,
    },
    RecipeInfo {
        recipe: Recipe::CatwalkStraight,
        name: "Catwalk Straight",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CatwalkStraight,
        outputs: OUT_CatwalkStraight,
    },
    RecipeInfo {
        recipe: Recipe::CatwalkTCrossing,
        name: "Catwalk T-Crossing",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CatwalkTCrossing,
        outputs: OUT_CatwalkTCrossing,
    },
    RecipeInfo {
        recipe: Recipe::CeilingLight,
        name: "Ceiling Light",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CeilingLight,
        outputs: OUT_CeilingLight,
    },
    RecipeInfo {
        recipe: Recipe::CenterDoorWall,
        name: "Center Door Wall",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CenterDoorWall,
        outputs: OUT_CenterDoorWall,
    },
    RecipeInfo {
        recipe: Recipe::Chainsaw,
        name: "Chainsaw",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Chainsaw,
        outputs: OUT_Chainsaw,
    },
    RecipeInfo {
        recipe: Recipe::CircuitBoard,
        name: "Circuit Board",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_CircuitBoard,
        outputs: OUT_CircuitBoard,
    },
    RecipeInfo {
        recipe: Recipe::ClusterNobelisk,
        name: "Cluster Nobelisk",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_ClusterNobelisk,
        outputs: OUT_ClusterNobelisk,
    },
    RecipeInfo {
        recipe: Recipe::CoalGenerator,
        name: "Coal Generator",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CoalGenerator,
        outputs: OUT_CoalGenerator,
    },
    RecipeInfo {
        recipe: Recipe::ColorCartridge,
        name: "Color Cartridge",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ColorCartridge,
        outputs: OUT_ColorCartridge,
    },
    RecipeInfo {
        recipe: Recipe::Computer,
        name: "Computer",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_Computer,
        outputs: OUT_Computer,
    },
    RecipeInfo {
        recipe: Recipe::Concrete,
        name: "Concrete",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Concrete,
        outputs: OUT_Concrete,
    },
    RecipeInfo {
        recipe: Recipe::Constructor,
        name: "Constructor",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Constructor,
        outputs: OUT_Constructor,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorBeltMk1,
        name: "Conveyor Belt Mk.1",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorBeltMk1,
        outputs: OUT_ConveyorBeltMk1,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorBeltMk2,
        name: "Conveyor Belt Mk.2",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorBeltMk2,
        outputs: OUT_ConveyorBeltMk2,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorBeltMk3,
        name: "Conveyor Belt Mk.3",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorBeltMk3,
        outputs: OUT_ConveyorBeltMk3,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorBeltMk4,
        name: "Conveyor Belt Mk.4",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorBeltMk4,
        outputs: OUT_ConveyorBeltMk4,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorBeltMk5,
        name: "Conveyor Belt Mk.5",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorBeltMk5,
        outputs: OUT_ConveyorBeltMk5,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorCeilingMount,
        name: "Conveyor Ceiling Mount",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorCeilingMount,
        outputs: OUT_ConveyorCeilingMount,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorLiftFloorHole,
        name: "Conveyor Lift Floor Hole",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorLiftFloorHole,
        outputs: OUT_ConveyorLiftFloorHole,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorLiftMk1,
        name: "Conveyor Lift Mk.1",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorLiftMk1,
        outputs: OUT_ConveyorLiftMk1,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorLiftMk2,
        name: "Conveyor Lift Mk.2",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorLiftMk2,
        outputs: OUT_ConveyorLiftMk2,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorLiftMk3,
        name: "Conveyor Lift Mk.3",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorLiftMk3,
        outputs: OUT_ConveyorLiftMk3,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorLiftMk4,
        name: "Conveyor Lift Mk.4",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorLiftMk4,
        outputs: OUT_ConveyorLiftMk4,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorLiftMk5,
        name: "Conveyor Lift Mk.5",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorLiftMk5,
        outputs: OUT_ConveyorLiftMk5,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorMerger,
        name: "Conveyor Merger",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorMerger,
        outputs: OUT_ConveyorMerger,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorPole,
        name: "Conveyor Pole",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorPole,
        outputs: OUT_ConveyorPole,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorSplitter,
        name: "Conveyor Splitter",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorSplitter,
        outputs: OUT_ConveyorSplitter,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorWallMount,
        name: "Conveyor Wall Mount",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorWallMount,
        outputs: OUT_ConveyorWallMount,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorWallX1,
        name: "Conveyor Wall x1",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorWallX1,
        outputs: OUT_ConveyorWallX1,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorWallX2,
        name: "Conveyor Wall x2",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorWallX2,
        outputs: OUT_ConveyorWallX2,
    },
    RecipeInfo {
        recipe: Recipe::ConveyorWallX3,
        name: "Conveyor Wall x3",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ConveyorWallX3,
        outputs: OUT_ConveyorWallX3,
    },
    RecipeInfo {
        recipe: Recipe::CoolingSystem,
        name: "Cooling System",
        machine: crate::models::types::MachineType::Blender,
        inputs: IN_CoolingSystem,
        outputs: OUT_CoolingSystem,
    },
    RecipeInfo {
        recipe: Recipe::CopperIngot,
        name: "Copper Ingot",
        machine: crate::models::types::MachineType::Smelter,
        inputs: IN_CopperIngot,
        outputs: OUT_CopperIngot,
    },
    RecipeInfo {
        recipe: Recipe::CopperPowder,
        name: "Copper Powder",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CopperPowder,
        outputs: OUT_CopperPowder,
    },
    RecipeInfo {
        recipe: Recipe::CopperSheet,
        name: "Copper Sheet",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CopperSheet,
        outputs: OUT_CopperSheet,
    },
    RecipeInfo {
        recipe: Recipe::CraftBench,
        name: "Craft Bench",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CraftBench,
        outputs: OUT_CraftBench,
    },
    RecipeInfo {
        recipe: Recipe::CrystalOscillator,
        name: "Crystal Oscillator",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_CrystalOscillator,
        outputs: OUT_CrystalOscillator,
    },
    RecipeInfo {
        recipe: Recipe::CyberWagon,
        name: "Cyber Wagon",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_CyberWagon,
        outputs: OUT_CyberWagon,
    },
    RecipeInfo {
        recipe: Recipe::DisplaySign,
        name: "Display Sign",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_DisplaySign,
        outputs: OUT_DisplaySign,
    },
    RecipeInfo {
        recipe: Recipe::DoubleRamp2m,
        name: "Double Ramp 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_DoubleRamp2m,
        outputs: OUT_DoubleRamp2m,
    },
    RecipeInfo {
        recipe: Recipe::DoubleRamp4m,
        name: "Double Ramp 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_DoubleRamp4m,
        outputs: OUT_DoubleRamp4m,
    },
    RecipeInfo {
        recipe: Recipe::DoubleRamp8m,
        name: "Double Ramp 8m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_DoubleRamp8m,
        outputs: OUT_DoubleRamp8m,
    },
    RecipeInfo {
        recipe: Recipe::DoubleWallOutletMk1,
        name: "Double Wall Outlet Mk.1",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_DoubleWallOutletMk1,
        outputs: OUT_DoubleWallOutletMk1,
    },
    RecipeInfo {
        recipe: Recipe::DoubleWallOutletMk2,
        name: "Double Wall Outlet Mk.2",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_DoubleWallOutletMk2,
        outputs: OUT_DoubleWallOutletMk2,
    },
    RecipeInfo {
        recipe: Recipe::DoubleWallOutletMk3,
        name: "Double Wall Outlet Mk.3",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_DoubleWallOutletMk3,
        outputs: OUT_DoubleWallOutletMk3,
    },
    RecipeInfo {
        recipe: Recipe::DownCornerRamp1m,
        name: "Down Corner Ramp 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_DownCornerRamp1m,
        outputs: OUT_DownCornerRamp1m,
    },
    RecipeInfo {
        recipe: Recipe::DownCornerRamp2m,
        name: "Down Corner Ramp 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_DownCornerRamp2m,
        outputs: OUT_DownCornerRamp2m,
    },
    RecipeInfo {
        recipe: Recipe::DownCornerRamp4m,
        name: "Down Corner Ramp 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_DownCornerRamp4m,
        outputs: OUT_DownCornerRamp4m,
    },
    RecipeInfo {
        recipe: Recipe::Drone,
        name: "Drone",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Drone,
        outputs: OUT_Drone,
    },
    RecipeInfo {
        recipe: Recipe::DronePort,
        name: "Drone Port",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_DronePort,
        outputs: OUT_DronePort,
    },
    RecipeInfo {
        recipe: Recipe::ElectricLocomotive,
        name: "Electric Locomotive",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ElectricLocomotive,
        outputs: OUT_ElectricLocomotive,
    },
    RecipeInfo {
        recipe: Recipe::ElectromagneticControlRod,
        name: "Electromagnetic Control Rod",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_ElectromagneticControlRod,
        outputs: OUT_ElectromagneticControlRod,
    },
    RecipeInfo {
        recipe: Recipe::EmptyCanister,
        name: "Empty Canister",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_EmptyCanister,
        outputs: OUT_EmptyCanister,
    },
    RecipeInfo {
        recipe: Recipe::EmptyFluidTank,
        name: "Empty Fluid Tank",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_EmptyFluidTank,
        outputs: OUT_EmptyFluidTank,
    },
    RecipeInfo {
        recipe: Recipe::EmptyPlatform,
        name: "Empty Platform",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_EmptyPlatform,
        outputs: OUT_EmptyPlatform,
    },
    RecipeInfo {
        recipe: Recipe::EmptyPlatformWithCatwalk,
        name: "Empty Platform With Catwalk",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_EmptyPlatformWithCatwalk,
        outputs: OUT_EmptyPlatformWithCatwalk,
    },
    RecipeInfo {
        recipe: Recipe::EncasedIndustrialBeam,
        name: "Encased Industrial Beam",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_EncasedIndustrialBeam,
        outputs: OUT_EncasedIndustrialBeam,
    },
    RecipeInfo {
        recipe: Recipe::EncasedPlutoniumCell,
        name: "Encased Plutonium Cell",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_EncasedPlutoniumCell,
        outputs: OUT_EncasedPlutoniumCell,
    },
    RecipeInfo {
        recipe: Recipe::EncasedUraniumCell,
        name: "Encased Uranium Cell",
        machine: crate::models::types::MachineType::Blender,
        inputs: IN_EncasedUraniumCell,
        outputs: OUT_EncasedUraniumCell,
    },
    RecipeInfo {
        recipe: Recipe::EquipmentWorkshop,
        name: "Equipment Workshop",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_EquipmentWorkshop,
        outputs: OUT_EquipmentWorkshop,
    },
    RecipeInfo {
        recipe: Recipe::Explorer,
        name: "Explorer",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Explorer,
        outputs: OUT_Explorer,
    },
    RecipeInfo {
        recipe: Recipe::ExplosiveRebar,
        name: "Explosive Rebar",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_ExplosiveRebar,
        outputs: OUT_ExplosiveRebar,
    },
    RecipeInfo {
        recipe: Recipe::FICSMASGiftTree,
        name: "FICSMAS Gift Tree",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FICSMASGiftTree,
        outputs: OUT_FICSMASGiftTree,
    },
    RecipeInfo {
        recipe: Recipe::FICSMASPowerLight,
        name: "FICSMAS Power Light",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FICSMASPowerLight,
        outputs: OUT_FICSMASPowerLight,
    },
    RecipeInfo {
        recipe: Recipe::FICSMASSnowDispenser,
        name: "FICSMAS Snow Dispenser",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FICSMASSnowDispenser,
        outputs: OUT_FICSMASSnowDispenser,
    },
    RecipeInfo {
        recipe: Recipe::FICSMASWreath,
        name: "FICSMAS Wreath",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FICSMASWreath,
        outputs: OUT_FICSMASWreath,
    },
    RecipeInfo {
        recipe: Recipe::Fabric,
        name: "Fabric",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_Fabric,
        outputs: OUT_Fabric,
    },
    RecipeInfo {
        recipe: Recipe::FactoryCart,
        name: "Factory Cart™",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FactoryCart,
        outputs: OUT_FactoryCart,
    },
    RecipeInfo {
        recipe: Recipe::FloodLightTower,
        name: "Flood Light Tower",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FloodLightTower,
        outputs: OUT_FloodLightTower,
    },
    RecipeInfo {
        recipe: Recipe::FluidBuffer,
        name: "Fluid Buffer",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FluidBuffer,
        outputs: OUT_FluidBuffer,
    },
    RecipeInfo {
        recipe: Recipe::FluidFreightPlatform,
        name: "Fluid Freight Platform",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FluidFreightPlatform,
        outputs: OUT_FluidFreightPlatform,
    },
    RecipeInfo {
        recipe: Recipe::Foundation1m,
        name: "Foundation 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Foundation1m,
        outputs: OUT_Foundation1m,
    },
    RecipeInfo {
        recipe: Recipe::Foundation2m,
        name: "Foundation 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Foundation2m,
        outputs: OUT_Foundation2m,
    },
    RecipeInfo {
        recipe: Recipe::Foundation4m,
        name: "Foundation 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Foundation4m,
        outputs: OUT_Foundation4m,
    },
    RecipeInfo {
        recipe: Recipe::Foundry,
        name: "Foundry",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Foundry,
        outputs: OUT_Foundry,
    },
    RecipeInfo {
        recipe: Recipe::FrameFloor,
        name: "Frame Floor",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FrameFloor,
        outputs: OUT_FrameFloor,
    },
    RecipeInfo {
        recipe: Recipe::FrameFoundation,
        name: "Frame Foundation",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FrameFoundation,
        outputs: OUT_FrameFoundation,
    },
    RecipeInfo {
        recipe: Recipe::FrameRamp,
        name: "Frame Ramp",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FrameRamp,
        outputs: OUT_FrameRamp,
    },
    RecipeInfo {
        recipe: Recipe::FrameWall,
        name: "Frame Wall",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FrameWall,
        outputs: OUT_FrameWall,
    },
    RecipeInfo {
        recipe: Recipe::FrameWindow,
        name: "Frame Window",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FrameWindow,
        outputs: OUT_FrameWindow,
    },
    RecipeInfo {
        recipe: Recipe::FreightCar,
        name: "Freight Car",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FreightCar,
        outputs: OUT_FreightCar,
    },
    RecipeInfo {
        recipe: Recipe::FreightPlatform,
        name: "Freight Platform",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FreightPlatform,
        outputs: OUT_FreightPlatform,
    },
    RecipeInfo {
        recipe: Recipe::Fuel,
        name: "Fuel",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_Fuel,
        outputs: OUT_Fuel,
    },
    RecipeInfo {
        recipe: Recipe::FuelGenerator,
        name: "Fuel Generator",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FuelGenerator,
        outputs: OUT_FuelGenerator,
    },
    RecipeInfo {
        recipe: Recipe::FullFrameWindow,
        name: "Full Frame Window",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_FullFrameWindow,
        outputs: OUT_FullFrameWindow,
    },
    RecipeInfo {
        recipe: Recipe::FusedModularFrame,
        name: "Fused Modular Frame",
        machine: crate::models::types::MachineType::Blender,
        inputs: IN_FusedModularFrame,
        outputs: OUT_FusedModularFrame,
    },
    RecipeInfo {
        recipe: Recipe::GasFilter,
        name: "Gas Filter",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_GasFilter,
        outputs: OUT_GasFilter,
    },
    RecipeInfo {
        recipe: Recipe::GasMask,
        name: "Gas Mask",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_GasMask,
        outputs: OUT_GasMask,
    },
    RecipeInfo {
        recipe: Recipe::GasNobelisk,
        name: "Gas Nobelisk",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_GasNobelisk,
        outputs: OUT_GasNobelisk,
    },
    RecipeInfo {
        recipe: Recipe::GateHoleWall,
        name: "Gate Hole Wall",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_GateHoleWall,
        outputs: OUT_GateHoleWall,
    },
    RecipeInfo {
        recipe: Recipe::GeothermalGenerator,
        name: "Geothermal Generator",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_GeothermalGenerator,
        outputs: OUT_GeothermalGenerator,
    },
    RecipeInfo {
        recipe: Recipe::GiantFICSMASTree,
        name: "Giant FICSMAS Tree",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_GiantFICSMASTree,
        outputs: OUT_GiantFICSMASTree,
    },
    RecipeInfo {
        recipe: Recipe::GlassFrameFoundation,
        name: "Glass Frame Foundation",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_GlassFrameFoundation,
        outputs: OUT_GlassFrameFoundation,
    },
    RecipeInfo {
        recipe: Recipe::GoldenFactoryCart,
        name: "Golden Factory Cart™",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_GoldenFactoryCart,
        outputs: OUT_GoldenFactoryCart,
    },
    RecipeInfo {
        recipe: Recipe::Half1mFoundation,
        name: "Half 1m Foundation",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Half1mFoundation,
        outputs: OUT_Half1mFoundation,
    },
    RecipeInfo {
        recipe: Recipe::Half2mFoundation,
        name: "Half 2m Foundation",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Half2mFoundation,
        outputs: OUT_Half2mFoundation,
    },
    RecipeInfo {
        recipe: Recipe::Half4mFoundation,
        name: "Half 4m Foundation",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Half4mFoundation,
        outputs: OUT_Half4mFoundation,
    },
    RecipeInfo {
        recipe: Recipe::HatcherProtein,
        name: "Hatcher Protein",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_HatcherProtein,
        outputs: OUT_HatcherProtein,
    },
    RecipeInfo {
        recipe: Recipe::HazardStorageBox,
        name: "Hazard Storage Box",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_HazardStorageBox,
        outputs: OUT_HazardStorageBox,
    },
    RecipeInfo {
        recipe: Recipe::HazmatSuit,
        name: "Hazmat Suit",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_HazmatSuit,
        outputs: OUT_HazmatSuit,
    },
    RecipeInfo {
        recipe: Recipe::HeatSink,
        name: "Heat Sink",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_HeatSink,
        outputs: OUT_HeatSink,
    },
    RecipeInfo {
        recipe: Recipe::HeavyModularFrame,
        name: "Heavy Modular Frame",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_HeavyModularFrame,
        outputs: OUT_HeavyModularFrame,
    },
    RecipeInfo {
        recipe: Recipe::HexFrameWindow,
        name: "Hex Frame Window",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_HexFrameWindow,
        outputs: OUT_HexFrameWindow,
    },
    RecipeInfo {
        recipe: Recipe::HighSpeedConnector,
        name: "High-Speed Connector",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_HighSpeedConnector,
        outputs: OUT_HighSpeedConnector,
    },
    RecipeInfo {
        recipe: Recipe::HogProtein,
        name: "Hog Protein",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_HogProtein,
        outputs: OUT_HogProtein,
    },
    RecipeInfo {
        recipe: Recipe::HomingRifleAmmo,
        name: "Homing Rifle Ammo",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_HomingRifleAmmo,
        outputs: OUT_HomingRifleAmmo,
    },
    RecipeInfo {
        recipe: Recipe::HoverPack,
        name: "Hover Pack",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_HoverPack,
        outputs: OUT_HoverPack,
    },
    RecipeInfo {
        recipe: Recipe::Hypertube,
        name: "Hypertube",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Hypertube,
        outputs: OUT_Hypertube,
    },
    RecipeInfo {
        recipe: Recipe::HypertubeEntrance,
        name: "Hypertube Entrance",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_HypertubeEntrance,
        outputs: OUT_HypertubeEntrance,
    },
    RecipeInfo {
        recipe: Recipe::HypertubeFloorHole,
        name: "Hypertube Floor Hole",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_HypertubeFloorHole,
        outputs: OUT_HypertubeFloorHole,
    },
    RecipeInfo {
        recipe: Recipe::HypertubeSupport,
        name: "Hypertube Support",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_HypertubeSupport,
        outputs: OUT_HypertubeSupport,
    },
    RecipeInfo {
        recipe: Recipe::HypertubeWallHole,
        name: "Hypertube Wall Hole",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_HypertubeWallHole,
        outputs: OUT_HypertubeWallHole,
    },
    RecipeInfo {
        recipe: Recipe::HypertubeWallSupport,
        name: "Hypertube Wall Support",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_HypertubeWallSupport,
        outputs: OUT_HypertubeWallSupport,
    },
    RecipeInfo {
        recipe: Recipe::IndustrialFluidBuffer,
        name: "Industrial Fluid Buffer",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_IndustrialFluidBuffer,
        outputs: OUT_IndustrialFluidBuffer,
    },
    RecipeInfo {
        recipe: Recipe::IndustrialStorageContainer,
        name: "Industrial Storage Container",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_IndustrialStorageContainer,
        outputs: OUT_IndustrialStorageContainer,
    },
    RecipeInfo {
        recipe: Recipe::InnerCornerExtension1m,
        name: "Inner Corner Extension 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InnerCornerExtension1m,
        outputs: OUT_InnerCornerExtension1m,
    },
    RecipeInfo {
        recipe: Recipe::InnerCornerExtension2m,
        name: "Inner Corner Extension 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InnerCornerExtension2m,
        outputs: OUT_InnerCornerExtension2m,
    },
    RecipeInfo {
        recipe: Recipe::InnerCornerExtension4m,
        name: "Inner Corner Extension 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InnerCornerExtension4m,
        outputs: OUT_InnerCornerExtension4m,
    },
    RecipeInfo {
        recipe: Recipe::InnerCornerQuarterPipe,
        name: "Inner Corner Quarter Pipe",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InnerCornerQuarterPipe,
        outputs: OUT_InnerCornerQuarterPipe,
    },
    RecipeInfo {
        recipe: Recipe::InnerCornerRoof1m,
        name: "Inner Corner Roof 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InnerCornerRoof1m,
        outputs: OUT_InnerCornerRoof1m,
    },
    RecipeInfo {
        recipe: Recipe::InnerCornerRoof2m,
        name: "Inner Corner Roof 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InnerCornerRoof2m,
        outputs: OUT_InnerCornerRoof2m,
    },
    RecipeInfo {
        recipe: Recipe::InnerCornerRoof4m,
        name: "Inner Corner Roof 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InnerCornerRoof4m,
        outputs: OUT_InnerCornerRoof4m,
    },
    RecipeInfo {
        recipe: Recipe::InvDownCorner1m,
        name: "Inv. Down Corner 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvDownCorner1m,
        outputs: OUT_InvDownCorner1m,
    },
    RecipeInfo {
        recipe: Recipe::InvDownCorner2m,
        name: "Inv. Down Corner 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvDownCorner2m,
        outputs: OUT_InvDownCorner2m,
    },
    RecipeInfo {
        recipe: Recipe::InvDownCorner4m,
        name: "Inv. Down Corner 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvDownCorner4m,
        outputs: OUT_InvDownCorner4m,
    },
    RecipeInfo {
        recipe: Recipe::InvRamp1m,
        name: "Inv. Ramp 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvRamp1m,
        outputs: OUT_InvRamp1m,
    },
    RecipeInfo {
        recipe: Recipe::InvRamp2m,
        name: "Inv. Ramp 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvRamp2m,
        outputs: OUT_InvRamp2m,
    },
    RecipeInfo {
        recipe: Recipe::InvRamp4m,
        name: "Inv. Ramp 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvRamp4m,
        outputs: OUT_InvRamp4m,
    },
    RecipeInfo {
        recipe: Recipe::InvRampWall1m,
        name: "Inv. Ramp Wall 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvRampWall1m,
        outputs: OUT_InvRampWall1m,
    },
    RecipeInfo {
        recipe: Recipe::InvRampWall2m,
        name: "Inv. Ramp Wall 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvRampWall2m,
        outputs: OUT_InvRampWall2m,
    },
    RecipeInfo {
        recipe: Recipe::InvRampWall4m,
        name: "Inv. Ramp Wall 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvRampWall4m,
        outputs: OUT_InvRampWall4m,
    },
    RecipeInfo {
        recipe: Recipe::InvRampWall8m,
        name: "Inv. Ramp Wall 8m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvRampWall8m,
        outputs: OUT_InvRampWall8m,
    },
    RecipeInfo {
        recipe: Recipe::InvUpCorner1m,
        name: "Inv. Up Corner 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvUpCorner1m,
        outputs: OUT_InvUpCorner1m,
    },
    RecipeInfo {
        recipe: Recipe::InvUpCorner2m,
        name: "Inv. Up Corner 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvUpCorner2m,
        outputs: OUT_InvUpCorner2m,
    },
    RecipeInfo {
        recipe: Recipe::InvUpCorner4m,
        name: "Inv. Up Corner 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvUpCorner4m,
        outputs: OUT_InvUpCorner4m,
    },
    RecipeInfo {
        recipe: Recipe::InvertedFrameRamp,
        name: "Inverted Frame Ramp",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvertedFrameRamp,
        outputs: OUT_InvertedFrameRamp,
    },
    RecipeInfo {
        recipe: Recipe::InvertedInnerCornerQuarterPipe,
        name: "Inverted Inner Corner Quarter Pipe",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvertedInnerCornerQuarterPipe,
        outputs: OUT_InvertedInnerCornerQuarterPipe,
    },
    RecipeInfo {
        recipe: Recipe::InvertedOuterCornerQuarterPipe,
        name: "Inverted Outer Corner Quarter Pipe",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvertedOuterCornerQuarterPipe,
        outputs: OUT_InvertedOuterCornerQuarterPipe,
    },
    RecipeInfo {
        recipe: Recipe::InvertedQuarterPipe,
        name: "Inverted Quarter Pipe",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_InvertedQuarterPipe,
        outputs: OUT_InvertedQuarterPipe,
    },
    RecipeInfo {
        recipe: Recipe::IodineInfusedFilter,
        name: "Iodine Infused Filter",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_IodineInfusedFilter,
        outputs: OUT_IodineInfusedFilter,
    },
    RecipeInfo {
        recipe: Recipe::IronIngot,
        name: "Iron Ingot",
        machine: crate::models::types::MachineType::Smelter,
        inputs: IN_IronIngot,
        outputs: OUT_IronIngot,
    },
    RecipeInfo {
        recipe: Recipe::IronPlate,
        name: "Iron Plate",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_IronPlate,
        outputs: OUT_IronPlate,
    },
    RecipeInfo {
        recipe: Recipe::IronRebar,
        name: "Iron Rebar",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_IronRebar,
        outputs: OUT_IronRebar,
    },
    RecipeInfo {
        recipe: Recipe::IronRod,
        name: "Iron Rod",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_IronRod,
        outputs: OUT_IronRod,
    },
    RecipeInfo {
        recipe: Recipe::Jetpack,
        name: "Jetpack",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Jetpack,
        outputs: OUT_Jetpack,
    },
    RecipeInfo {
        recipe: Recipe::JumpPad,
        name: "Jump Pad",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_JumpPad,
        outputs: OUT_JumpPad,
    },
    RecipeInfo {
        recipe: Recipe::LabelSign2m,
        name: "Label Sign 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_LabelSign2m,
        outputs: OUT_LabelSign2m,
    },
    RecipeInfo {
        recipe: Recipe::LabelSign3m,
        name: "Label Sign 3m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_LabelSign3m,
        outputs: OUT_LabelSign3m,
    },
    RecipeInfo {
        recipe: Recipe::LabelSign4m,
        name: "Label Sign 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_LabelSign4m,
        outputs: OUT_LabelSign4m,
    },
    RecipeInfo {
        recipe: Recipe::Ladder,
        name: "Ladder",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Ladder,
        outputs: OUT_Ladder,
    },
    RecipeInfo {
        recipe: Recipe::LargeBillboard,
        name: "Large Billboard",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_LargeBillboard,
        outputs: OUT_LargeBillboard,
    },
    RecipeInfo {
        recipe: Recipe::LightsControlPanel,
        name: "Lights Control Panel",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_LightsControlPanel,
        outputs: OUT_LightsControlPanel,
    },
    RecipeInfo {
        recipe: Recipe::LiquidBiofuel,
        name: "Liquid Biofuel",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_LiquidBiofuel,
        outputs: OUT_LiquidBiofuel,
    },
    RecipeInfo {
        recipe: Recipe::LookoutTower,
        name: "Lookout Tower",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_LookoutTower,
        outputs: OUT_LookoutTower,
    },
    RecipeInfo {
        recipe: Recipe::MAM,
        name: "MAM",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_MAM,
        outputs: OUT_MAM,
    },
    RecipeInfo {
        recipe: Recipe::MagneticFieldGenerator,
        name: "Magnetic Field Generator",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_MagneticFieldGenerator,
        outputs: OUT_MagneticFieldGenerator,
    },
    RecipeInfo {
        recipe: Recipe::Manufacturer,
        name: "Manufacturer",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Manufacturer,
        outputs: OUT_Manufacturer,
    },
    RecipeInfo {
        recipe: Recipe::MedicalStorageBox,
        name: "Medical Storage Box",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_MedicalStorageBox,
        outputs: OUT_MedicalStorageBox,
    },
    RecipeInfo {
        recipe: Recipe::MetalBeam,
        name: "Metal Beam",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_MetalBeam,
        outputs: OUT_MetalBeam,
    },
    RecipeInfo {
        recipe: Recipe::MinerMk1,
        name: "Miner Mk.1",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_MinerMk1,
        outputs: OUT_MinerMk1,
    },
    RecipeInfo {
        recipe: Recipe::MinerMk2,
        name: "Miner Mk.2",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_MinerMk2,
        outputs: OUT_MinerMk2,
    },
    RecipeInfo {
        recipe: Recipe::MinerMk3,
        name: "Miner Mk.3",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_MinerMk3,
        outputs: OUT_MinerMk3,
    },
    RecipeInfo {
        recipe: Recipe::ModernRailing,
        name: "Modern Railing",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ModernRailing,
        outputs: OUT_ModernRailing,
    },
    RecipeInfo {
        recipe: Recipe::ModularEngine,
        name: "Modular Engine",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_ModularEngine,
        outputs: OUT_ModularEngine,
    },
    RecipeInfo {
        recipe: Recipe::ModularFrame,
        name: "Modular Frame",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_ModularFrame,
        outputs: OUT_ModularFrame,
    },
    RecipeInfo {
        recipe: Recipe::Motor,
        name: "Motor",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_Motor,
        outputs: OUT_Motor,
    },
    RecipeInfo {
        recipe: Recipe::NitricAcid,
        name: "Nitric Acid",
        machine: crate::models::types::MachineType::Blender,
        inputs: IN_NitricAcid,
        outputs: OUT_NitricAcid,
    },
    RecipeInfo {
        recipe: Recipe::Nobelisk,
        name: "Nobelisk",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_Nobelisk,
        outputs: OUT_Nobelisk,
    },
    RecipeInfo {
        recipe: Recipe::NobeliskDetonator,
        name: "Nobelisk Detonator",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_NobeliskDetonator,
        outputs: OUT_NobeliskDetonator,
    },
    RecipeInfo {
        recipe: Recipe::NonFissileUranium,
        name: "Non-fissile Uranium",
        machine: crate::models::types::MachineType::Blender,
        inputs: IN_NonFissileUranium,
        outputs: OUT_NonFissileUranium,
    },
    RecipeInfo {
        recipe: Recipe::NuclearPasta,
        name: "Nuclear Pasta",
        machine: crate::models::types::MachineType::ParticleAccelerator,
        inputs: IN_NuclearPasta,
        outputs: OUT_NuclearPasta,
    },
    RecipeInfo {
        recipe: Recipe::NuclearPowerPlant,
        name: "Nuclear Power Plant",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_NuclearPowerPlant,
        outputs: OUT_NuclearPowerPlant,
    },
    RecipeInfo {
        recipe: Recipe::NukeNobelisk,
        name: "Nuke Nobelisk",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_NukeNobelisk,
        outputs: OUT_NukeNobelisk,
    },
    RecipeInfo {
        recipe: Recipe::NutritionalInhaler,
        name: "Nutritional Inhaler",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_NutritionalInhaler,
        outputs: OUT_NutritionalInhaler,
    },
    RecipeInfo {
        recipe: Recipe::ObjectScanner,
        name: "Object Scanner",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ObjectScanner,
        outputs: OUT_ObjectScanner,
    },
    RecipeInfo {
        recipe: Recipe::OilExtractor,
        name: "Oil Extractor",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_OilExtractor,
        outputs: OUT_OilExtractor,
    },
    RecipeInfo {
        recipe: Recipe::OuterCornerExtension1m,
        name: "Outer Corner Extension 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_OuterCornerExtension1m,
        outputs: OUT_OuterCornerExtension1m,
    },
    RecipeInfo {
        recipe: Recipe::OuterCornerExtension2m,
        name: "Outer Corner Extension 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_OuterCornerExtension2m,
        outputs: OUT_OuterCornerExtension2m,
    },
    RecipeInfo {
        recipe: Recipe::OuterCornerExtension4m,
        name: "Outer Corner Extension 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_OuterCornerExtension4m,
        outputs: OUT_OuterCornerExtension4m,
    },
    RecipeInfo {
        recipe: Recipe::OuterCornerQuarterPipe,
        name: "Outer Corner Quarter Pipe",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_OuterCornerQuarterPipe,
        outputs: OUT_OuterCornerQuarterPipe,
    },
    RecipeInfo {
        recipe: Recipe::OuterCornerRoof1m,
        name: "Outer Corner Roof 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_OuterCornerRoof1m,
        outputs: OUT_OuterCornerRoof1m,
    },
    RecipeInfo {
        recipe: Recipe::OuterCornerRoof2m,
        name: "Outer Corner Roof 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_OuterCornerRoof2m,
        outputs: OUT_OuterCornerRoof2m,
    },
    RecipeInfo {
        recipe: Recipe::OuterCornerRoof4m,
        name: "Outer Corner Roof 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_OuterCornerRoof4m,
        outputs: OUT_OuterCornerRoof4m,
    },
    RecipeInfo {
        recipe: Recipe::PackagedAluminaSolution,
        name: "Packaged Alumina Solution",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_PackagedAluminaSolution,
        outputs: OUT_PackagedAluminaSolution,
    },
    RecipeInfo {
        recipe: Recipe::PackagedFuel,
        name: "Packaged Fuel",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_PackagedFuel,
        outputs: OUT_PackagedFuel,
    },
    RecipeInfo {
        recipe: Recipe::PackagedHeavyOilResidue,
        name: "Packaged Heavy Oil Residue",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_PackagedHeavyOilResidue,
        outputs: OUT_PackagedHeavyOilResidue,
    },
    RecipeInfo {
        recipe: Recipe::PackagedLiquidBiofuel,
        name: "Packaged Liquid Biofuel",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_PackagedLiquidBiofuel,
        outputs: OUT_PackagedLiquidBiofuel,
    },
    RecipeInfo {
        recipe: Recipe::PackagedNitricAcid,
        name: "Packaged Nitric Acid",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_PackagedNitricAcid,
        outputs: OUT_PackagedNitricAcid,
    },
    RecipeInfo {
        recipe: Recipe::PackagedNitrogenGas,
        name: "Packaged Nitrogen Gas",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_PackagedNitrogenGas,
        outputs: OUT_PackagedNitrogenGas,
    },
    RecipeInfo {
        recipe: Recipe::PackagedOil,
        name: "Packaged Oil",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_PackagedOil,
        outputs: OUT_PackagedOil,
    },
    RecipeInfo {
        recipe: Recipe::PackagedSulfuricAcid,
        name: "Packaged Sulfuric Acid",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_PackagedSulfuricAcid,
        outputs: OUT_PackagedSulfuricAcid,
    },
    RecipeInfo {
        recipe: Recipe::PackagedTurbofuel,
        name: "Packaged Turbofuel",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_PackagedTurbofuel,
        outputs: OUT_PackagedTurbofuel,
    },
    RecipeInfo {
        recipe: Recipe::PackagedWater,
        name: "Packaged Water",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_PackagedWater,
        outputs: OUT_PackagedWater,
    },
    RecipeInfo {
        recipe: Recipe::Packager,
        name: "Packager",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Packager,
        outputs: OUT_Packager,
    },
    RecipeInfo {
        recipe: Recipe::PaintedBeam,
        name: "Painted Beam",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PaintedBeam,
        outputs: OUT_PaintedBeam,
    },
    RecipeInfo {
        recipe: Recipe::PanelWindow,
        name: "Panel Window",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PanelWindow,
        outputs: OUT_PanelWindow,
    },
    RecipeInfo {
        recipe: Recipe::Parachute,
        name: "Parachute",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Parachute,
        outputs: OUT_Parachute,
    },
    RecipeInfo {
        recipe: Recipe::ParticleAccelerator,
        name: "Particle Accelerator",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ParticleAccelerator,
        outputs: OUT_ParticleAccelerator,
    },
    RecipeInfo {
        recipe: Recipe::PathSignal,
        name: "Path Signal",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PathSignal,
        outputs: OUT_PathSignal,
    },
    RecipeInfo {
        recipe: Recipe::PersonalStorageBox,
        name: "Personal Storage Box",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PersonalStorageBox,
        outputs: OUT_PersonalStorageBox,
    },
    RecipeInfo {
        recipe: Recipe::PetroleumCoke,
        name: "Petroleum Coke",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_PetroleumCoke,
        outputs: OUT_PetroleumCoke,
    },
    RecipeInfo {
        recipe: Recipe::PipelineFloorHole,
        name: "Pipeline Floor Hole",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PipelineFloorHole,
        outputs: OUT_PipelineFloorHole,
    },
    RecipeInfo {
        recipe: Recipe::PipelineJunctionCross,
        name: "Pipeline Junction Cross",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PipelineJunctionCross,
        outputs: OUT_PipelineJunctionCross,
    },
    RecipeInfo {
        recipe: Recipe::PipelineMk1,
        name: "Pipeline Mk.1",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PipelineMk1,
        outputs: OUT_PipelineMk1,
    },
    RecipeInfo {
        recipe: Recipe::PipelineMk1NoIndicator,
        name: "Pipeline Mk.1 (No Indicator)",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PipelineMk1NoIndicator,
        outputs: OUT_PipelineMk1NoIndicator,
    },
    RecipeInfo {
        recipe: Recipe::PipelineMk2,
        name: "Pipeline Mk.2",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PipelineMk2,
        outputs: OUT_PipelineMk2,
    },
    RecipeInfo {
        recipe: Recipe::PipelineMk2NoIndicator,
        name: "Pipeline Mk.2 (No Indicator)",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PipelineMk2NoIndicator,
        outputs: OUT_PipelineMk2NoIndicator,
    },
    RecipeInfo {
        recipe: Recipe::PipelinePumpMk1,
        name: "Pipeline Pump Mk.1",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PipelinePumpMk1,
        outputs: OUT_PipelinePumpMk1,
    },
    RecipeInfo {
        recipe: Recipe::PipelinePumpMk2,
        name: "Pipeline Pump Mk.2",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PipelinePumpMk2,
        outputs: OUT_PipelinePumpMk2,
    },
    RecipeInfo {
        recipe: Recipe::PipelineSupport,
        name: "Pipeline Support",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PipelineSupport,
        outputs: OUT_PipelineSupport,
    },
    RecipeInfo {
        recipe: Recipe::PipelineWallHole,
        name: "Pipeline Wall Hole",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PipelineWallHole,
        outputs: OUT_PipelineWallHole,
    },
    RecipeInfo {
        recipe: Recipe::PipelineWallSupport,
        name: "Pipeline Wall Support",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PipelineWallSupport,
        outputs: OUT_PipelineWallSupport,
    },
    RecipeInfo {
        recipe: Recipe::Plastic,
        name: "Plastic",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_Plastic,
        outputs: OUT_Plastic,
    },
    RecipeInfo {
        recipe: Recipe::PlutoniumFuelRod,
        name: "Plutonium Fuel Rod",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_PlutoniumFuelRod,
        outputs: OUT_PlutoniumFuelRod,
    },
    RecipeInfo {
        recipe: Recipe::PlutoniumPellet,
        name: "Plutonium Pellet",
        machine: crate::models::types::MachineType::ParticleAccelerator,
        inputs: IN_PlutoniumPellet,
        outputs: OUT_PlutoniumPellet,
    },
    RecipeInfo {
        recipe: Recipe::PortableMiner,
        name: "Portable Miner",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PortableMiner,
        outputs: OUT_PortableMiner,
    },
    RecipeInfo {
        recipe: Recipe::PortraitSign,
        name: "Portrait Sign",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PortraitSign,
        outputs: OUT_PortraitSign,
    },
    RecipeInfo {
        recipe: Recipe::PowerLine,
        name: "Power Line",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PowerLine,
        outputs: OUT_PowerLine,
    },
    RecipeInfo {
        recipe: Recipe::PowerPoleMk1,
        name: "Power Pole Mk.1",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PowerPoleMk1,
        outputs: OUT_PowerPoleMk1,
    },
    RecipeInfo {
        recipe: Recipe::PowerPoleMk2,
        name: "Power Pole Mk.2",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PowerPoleMk2,
        outputs: OUT_PowerPoleMk2,
    },
    RecipeInfo {
        recipe: Recipe::PowerPoleMk3,
        name: "Power Pole Mk.3",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PowerPoleMk3,
        outputs: OUT_PowerPoleMk3,
    },
    RecipeInfo {
        recipe: Recipe::PowerShard1,
        name: "Power Shard (1)",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PowerShard1,
        outputs: OUT_PowerShard1,
    },
    RecipeInfo {
        recipe: Recipe::PowerShard2,
        name: "Power Shard (2)",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PowerShard2,
        outputs: OUT_PowerShard2,
    },
    RecipeInfo {
        recipe: Recipe::PowerShard5,
        name: "Power Shard (5)",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PowerShard5,
        outputs: OUT_PowerShard5,
    },
    RecipeInfo {
        recipe: Recipe::PowerStorage,
        name: "Power Storage",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PowerStorage,
        outputs: OUT_PowerStorage,
    },
    RecipeInfo {
        recipe: Recipe::PowerSwitch,
        name: "Power Switch",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PowerSwitch,
        outputs: OUT_PowerSwitch,
    },
    RecipeInfo {
        recipe: Recipe::PowerTower,
        name: "Power Tower",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PowerTower,
        outputs: OUT_PowerTower,
    },
    RecipeInfo {
        recipe: Recipe::PowerTowerPlatform,
        name: "Power Tower Platform",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PowerTowerPlatform,
        outputs: OUT_PowerTowerPlatform,
    },
    RecipeInfo {
        recipe: Recipe::PressureConversionCube,
        name: "Pressure Conversion Cube",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_PressureConversionCube,
        outputs: OUT_PressureConversionCube,
    },
    RecipeInfo {
        recipe: Recipe::PriorityPowerSwitch,
        name: "Priority Power Switch",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_PriorityPowerSwitch,
        outputs: OUT_PriorityPowerSwitch,
    },
    RecipeInfo {
        recipe: Recipe::ProgrammableSplitter,
        name: "Programmable Splitter",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ProgrammableSplitter,
        outputs: OUT_ProgrammableSplitter,
    },
    RecipeInfo {
        recipe: Recipe::ProteinInhaler,
        name: "Protein Inhaler",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ProteinInhaler,
        outputs: OUT_ProteinInhaler,
    },
    RecipeInfo {
        recipe: Recipe::PulseNobelisk,
        name: "Pulse Nobelisk",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_PulseNobelisk,
        outputs: OUT_PulseNobelisk,
    },
    RecipeInfo {
        recipe: Recipe::QuarterPipe,
        name: "Quarter Pipe",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_QuarterPipe,
        outputs: OUT_QuarterPipe,
    },
    RecipeInfo {
        recipe: Recipe::QuartzCrystal,
        name: "Quartz Crystal",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_QuartzCrystal,
        outputs: OUT_QuartzCrystal,
    },
    RecipeInfo {
        recipe: Recipe::Quickwire,
        name: "Quickwire",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Quickwire,
        outputs: OUT_Quickwire,
    },
    RecipeInfo {
        recipe: Recipe::RadarTower,
        name: "Radar Tower",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_RadarTower,
        outputs: OUT_RadarTower,
    },
    RecipeInfo {
        recipe: Recipe::RadioControlUnit,
        name: "Radio Control Unit",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_RadioControlUnit,
        outputs: OUT_RadioControlUnit,
    },
    RecipeInfo {
        recipe: Recipe::Railway,
        name: "Railway",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Railway,
        outputs: OUT_Railway,
    },
    RecipeInfo {
        recipe: Recipe::Ramp1m,
        name: "Ramp 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Ramp1m,
        outputs: OUT_Ramp1m,
    },
    RecipeInfo {
        recipe: Recipe::Ramp2m,
        name: "Ramp 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Ramp2m,
        outputs: OUT_Ramp2m,
    },
    RecipeInfo {
        recipe: Recipe::Ramp4m,
        name: "Ramp 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Ramp4m,
        outputs: OUT_Ramp4m,
    },
    RecipeInfo {
        recipe: Recipe::RampWall1m,
        name: "Ramp Wall 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_RampWall1m,
        outputs: OUT_RampWall1m,
    },
    RecipeInfo {
        recipe: Recipe::RampWall2m,
        name: "Ramp Wall 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_RampWall2m,
        outputs: OUT_RampWall2m,
    },
    RecipeInfo {
        recipe: Recipe::RampWall4m,
        name: "Ramp Wall 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_RampWall4m,
        outputs: OUT_RampWall4m,
    },
    RecipeInfo {
        recipe: Recipe::RampWall8m,
        name: "Ramp Wall 8m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_RampWall8m,
        outputs: OUT_RampWall8m,
    },
    RecipeInfo {
        recipe: Recipe::RebarGun,
        name: "Rebar Gun",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_RebarGun,
        outputs: OUT_RebarGun,
    },
    RecipeInfo {
        recipe: Recipe::Refinery,
        name: "Refinery",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Refinery,
        outputs: OUT_Refinery,
    },
    RecipeInfo {
        recipe: Recipe::ReinforcedIronPlate,
        name: "Reinforced Iron Plate",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_ReinforcedIronPlate,
        outputs: OUT_ReinforcedIronPlate,
    },
    RecipeInfo {
        recipe: Recipe::ReinforcedWindow,
        name: "Reinforced Window",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ReinforcedWindow,
        outputs: OUT_ReinforcedWindow,
    },
    RecipeInfo {
        recipe: Recipe::ResidualFuel,
        name: "Residual Fuel",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_ResidualFuel,
        outputs: OUT_ResidualFuel,
    },
    RecipeInfo {
        recipe: Recipe::ResidualPlastic,
        name: "Residual Plastic",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_ResidualPlastic,
        outputs: OUT_ResidualPlastic,
    },
    RecipeInfo {
        recipe: Recipe::ResidualRubber,
        name: "Residual Rubber",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_ResidualRubber,
        outputs: OUT_ResidualRubber,
    },
    RecipeInfo {
        recipe: Recipe::ResourceWellExtractor,
        name: "Resource Well Extractor",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ResourceWellExtractor,
        outputs: OUT_ResourceWellExtractor,
    },
    RecipeInfo {
        recipe: Recipe::ResourceWellPressurizer,
        name: "Resource Well Pressurizer",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_ResourceWellPressurizer,
        outputs: OUT_ResourceWellPressurizer,
    },
    RecipeInfo {
        recipe: Recipe::Rifle,
        name: "Rifle",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Rifle,
        outputs: OUT_Rifle,
    },
    RecipeInfo {
        recipe: Recipe::RifleAmmo,
        name: "Rifle Ammo",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_RifleAmmo,
        outputs: OUT_RifleAmmo,
    },
    RecipeInfo {
        recipe: Recipe::RoadBarrier,
        name: "Road Barrier",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_RoadBarrier,
        outputs: OUT_RoadBarrier,
    },
    RecipeInfo {
        recipe: Recipe::Roof1m,
        name: "Roof 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Roof1m,
        outputs: OUT_Roof1m,
    },
    RecipeInfo {
        recipe: Recipe::Roof2m,
        name: "Roof 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Roof2m,
        outputs: OUT_Roof2m,
    },
    RecipeInfo {
        recipe: Recipe::Roof4m,
        name: "Roof 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Roof4m,
        outputs: OUT_Roof4m,
    },
    RecipeInfo {
        recipe: Recipe::RoofFlat,
        name: "Roof Flat",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_RoofFlat,
        outputs: OUT_RoofFlat,
    },
    RecipeInfo {
        recipe: Recipe::Rotor,
        name: "Rotor",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_Rotor,
        outputs: OUT_Rotor,
    },
    RecipeInfo {
        recipe: Recipe::Rubber,
        name: "Rubber",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_Rubber,
        outputs: OUT_Rubber,
    },
    RecipeInfo {
        recipe: Recipe::Screw,
        name: "Screw",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Screw,
        outputs: OUT_Screw,
    },
    RecipeInfo {
        recipe: Recipe::ShatterRebar,
        name: "Shatter Rebar",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_ShatterRebar,
        outputs: OUT_ShatterRebar,
    },
    RecipeInfo {
        recipe: Recipe::SideDoorWall,
        name: "Side Door Wall",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SideDoorWall,
        outputs: OUT_SideDoorWall,
    },
    RecipeInfo {
        recipe: Recipe::Silica,
        name: "Silica",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Silica,
        outputs: OUT_Silica,
    },
    RecipeInfo {
        recipe: Recipe::SingleWindow,
        name: "Single Window",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SingleWindow,
        outputs: OUT_SingleWindow,
    },
    RecipeInfo {
        recipe: Recipe::SmallBillboard,
        name: "Small Billboard",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SmallBillboard,
        outputs: OUT_SmallBillboard,
    },
    RecipeInfo {
        recipe: Recipe::SmallConcretePillar,
        name: "Small Concrete Pillar",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SmallConcretePillar,
        outputs: OUT_SmallConcretePillar,
    },
    RecipeInfo {
        recipe: Recipe::SmallFramePillar,
        name: "Small Frame Pillar",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SmallFramePillar,
        outputs: OUT_SmallFramePillar,
    },
    RecipeInfo {
        recipe: Recipe::SmallMetalPillar,
        name: "Small Metal Pillar",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SmallMetalPillar,
        outputs: OUT_SmallMetalPillar,
    },
    RecipeInfo {
        recipe: Recipe::SmallPillarSupport,
        name: "Small Pillar Support",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SmallPillarSupport,
        outputs: OUT_SmallPillarSupport,
    },
    RecipeInfo {
        recipe: Recipe::SmartPlating,
        name: "Smart Plating",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_SmartPlating,
        outputs: OUT_SmartPlating,
    },
    RecipeInfo {
        recipe: Recipe::SmartSplitter,
        name: "Smart Splitter",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SmartSplitter,
        outputs: OUT_SmartSplitter,
    },
    RecipeInfo {
        recipe: Recipe::Smelter,
        name: "Smelter",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Smelter,
        outputs: OUT_Smelter,
    },
    RecipeInfo {
        recipe: Recipe::SmokelessPowder,
        name: "Smokeless Powder",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_SmokelessPowder,
        outputs: OUT_SmokelessPowder,
    },
    RecipeInfo {
        recipe: Recipe::Snowman,
        name: "Snowman",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Snowman,
        outputs: OUT_Snowman,
    },
    RecipeInfo {
        recipe: Recipe::SolidBiofuel,
        name: "Solid Biofuel",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SolidBiofuel,
        outputs: OUT_SolidBiofuel,
    },
    RecipeInfo {
        recipe: Recipe::SpaceElevator,
        name: "Space Elevator",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SpaceElevator,
        outputs: OUT_SpaceElevator,
    },
    RecipeInfo {
        recipe: Recipe::SpitterProtein,
        name: "Spitter Protein",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SpitterProtein,
        outputs: OUT_SpitterProtein,
    },
    RecipeInfo {
        recipe: Recipe::SquareSign05m,
        name: "Square Sign 0.5m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SquareSign05m,
        outputs: OUT_SquareSign05m,
    },
    RecipeInfo {
        recipe: Recipe::SquareSign1m,
        name: "Square Sign 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SquareSign1m,
        outputs: OUT_SquareSign1m,
    },
    RecipeInfo {
        recipe: Recipe::SquareSign2m,
        name: "Square Sign 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SquareSign2m,
        outputs: OUT_SquareSign2m,
    },
    RecipeInfo {
        recipe: Recipe::StackableConveyorPole,
        name: "Stackable Conveyor Pole",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_StackableConveyorPole,
        outputs: OUT_StackableConveyorPole,
    },
    RecipeInfo {
        recipe: Recipe::StackableHypertubeSupport,
        name: "Stackable Hypertube Support",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_StackableHypertubeSupport,
        outputs: OUT_StackableHypertubeSupport,
    },
    RecipeInfo {
        recipe: Recipe::StackablePipelineSupport,
        name: "Stackable Pipeline Support",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_StackablePipelineSupport,
        outputs: OUT_StackablePipelineSupport,
    },
    RecipeInfo {
        recipe: Recipe::StairsLeft,
        name: "Stairs Left",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_StairsLeft,
        outputs: OUT_StairsLeft,
    },
    RecipeInfo {
        recipe: Recipe::StairsRight,
        name: "Stairs Right",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_StairsRight,
        outputs: OUT_StairsRight,
    },
    RecipeInfo {
        recipe: Recipe::Stator,
        name: "Stator",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_Stator,
        outputs: OUT_Stator,
    },
    RecipeInfo {
        recipe: Recipe::SteelBeam,
        name: "Steel Beam",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SteelBeam,
        outputs: OUT_SteelBeam,
    },
    RecipeInfo {
        recipe: Recipe::SteelIngot,
        name: "Steel Ingot",
        machine: crate::models::types::MachineType::Foundry,
        inputs: IN_SteelIngot,
        outputs: OUT_SteelIngot,
    },
    RecipeInfo {
        recipe: Recipe::SteelPipe,
        name: "Steel Pipe",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_SteelPipe,
        outputs: OUT_SteelPipe,
    },
    RecipeInfo {
        recipe: Recipe::StingerProtein,
        name: "Stinger Protein",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_StingerProtein,
        outputs: OUT_StingerProtein,
    },
    RecipeInfo {
        recipe: Recipe::StorageContainer,
        name: "Storage Container",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_StorageContainer,
        outputs: OUT_StorageContainer,
    },
    RecipeInfo {
        recipe: Recipe::StreetLight,
        name: "Street Light",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_StreetLight,
        outputs: OUT_StreetLight,
    },
    RecipeInfo {
        recipe: Recipe::StunRebar,
        name: "Stun Rebar",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_StunRebar,
        outputs: OUT_StunRebar,
    },
    RecipeInfo {
        recipe: Recipe::SulfuricAcid,
        name: "Sulfuric Acid",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_SulfuricAcid,
        outputs: OUT_SulfuricAcid,
    },
    RecipeInfo {
        recipe: Recipe::Supercomputer,
        name: "Supercomputer",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_Supercomputer,
        outputs: OUT_Supercomputer,
    },
    RecipeInfo {
        recipe: Recipe::TheHUB,
        name: "The HUB",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_TheHUB,
        outputs: OUT_TheHUB,
    },
    RecipeInfo {
        recipe: Recipe::TherapeuticInhaler,
        name: "Therapeutic Inhaler",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_TherapeuticInhaler,
        outputs: OUT_TherapeuticInhaler,
    },
    RecipeInfo {
        recipe: Recipe::ThermalPropulsionRocket,
        name: "Thermal Propulsion Rocket",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_ThermalPropulsionRocket,
        outputs: OUT_ThermalPropulsionRocket,
    },
    RecipeInfo {
        recipe: Recipe::TiltedConcaveWall4m,
        name: "Tilted Concave Wall 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_TiltedConcaveWall4m,
        outputs: OUT_TiltedConcaveWall4m,
    },
    RecipeInfo {
        recipe: Recipe::TiltedConcaveWall8m,
        name: "Tilted Concave Wall 8m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_TiltedConcaveWall8m,
        outputs: OUT_TiltedConcaveWall8m,
    },
    RecipeInfo {
        recipe: Recipe::TiltedCornerWall4m,
        name: "Tilted Corner Wall 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_TiltedCornerWall4m,
        outputs: OUT_TiltedCornerWall4m,
    },
    RecipeInfo {
        recipe: Recipe::TiltedCornerWall8m,
        name: "Tilted Corner Wall 8m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_TiltedCornerWall8m,
        outputs: OUT_TiltedCornerWall8m,
    },
    RecipeInfo {
        recipe: Recipe::TiltedWall4m,
        name: "Tilted Wall 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_TiltedWall4m,
        outputs: OUT_TiltedWall4m,
    },
    RecipeInfo {
        recipe: Recipe::TiltedWall8m,
        name: "Tilted Wall 8m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_TiltedWall8m,
        outputs: OUT_TiltedWall8m,
    },
    RecipeInfo {
        recipe: Recipe::Tractor,
        name: "Tractor",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Tractor,
        outputs: OUT_Tractor,
    },
    RecipeInfo {
        recipe: Recipe::TrainStation,
        name: "Train Station",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_TrainStation,
        outputs: OUT_TrainStation,
    },
    RecipeInfo {
        recipe: Recipe::Truck,
        name: "Truck",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Truck,
        outputs: OUT_Truck,
    },
    RecipeInfo {
        recipe: Recipe::TruckStation,
        name: "Truck Station",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_TruckStation,
        outputs: OUT_TruckStation,
    },
    RecipeInfo {
        recipe: Recipe::TurboMotor,
        name: "Turbo Motor",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_TurboMotor,
        outputs: OUT_TurboMotor,
    },
    RecipeInfo {
        recipe: Recipe::TurboRifleAmmo,
        name: "Turbo Rifle Ammo",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_TurboRifleAmmo,
        outputs: OUT_TurboRifleAmmo,
    },
    RecipeInfo {
        recipe: Recipe::Turbofuel,
        name: "Turbofuel",
        machine: crate::models::types::MachineType::Refinery,
        inputs: IN_Turbofuel,
        outputs: OUT_Turbofuel,
    },
    RecipeInfo {
        recipe: Recipe::UJellyLandingPad,
        name: "U-Jelly Landing Pad",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_UJellyLandingPad,
        outputs: OUT_UJellyLandingPad,
    },
    RecipeInfo {
        recipe: Recipe::UnpackageAluminaSolution,
        name: "Unpackage Alumina Solution",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_UnpackageAluminaSolution,
        outputs: OUT_UnpackageAluminaSolution,
    },
    RecipeInfo {
        recipe: Recipe::UnpackageFuel,
        name: "Unpackage Fuel",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_UnpackageFuel,
        outputs: OUT_UnpackageFuel,
    },
    RecipeInfo {
        recipe: Recipe::UnpackageHeavyOilResidue,
        name: "Unpackage Heavy Oil Residue",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_UnpackageHeavyOilResidue,
        outputs: OUT_UnpackageHeavyOilResidue,
    },
    RecipeInfo {
        recipe: Recipe::UnpackageLiquidBiofuel,
        name: "Unpackage Liquid Biofuel",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_UnpackageLiquidBiofuel,
        outputs: OUT_UnpackageLiquidBiofuel,
    },
    RecipeInfo {
        recipe: Recipe::UnpackageNitricAcid,
        name: "Unpackage Nitric Acid",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_UnpackageNitricAcid,
        outputs: OUT_UnpackageNitricAcid,
    },
    RecipeInfo {
        recipe: Recipe::UnpackageNitrogenGas,
        name: "Unpackage Nitrogen Gas",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_UnpackageNitrogenGas,
        outputs: OUT_UnpackageNitrogenGas,
    },
    RecipeInfo {
        recipe: Recipe::UnpackageOil,
        name: "Unpackage Oil",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_UnpackageOil,
        outputs: OUT_UnpackageOil,
    },
    RecipeInfo {
        recipe: Recipe::UnpackageSulfuricAcid,
        name: "Unpackage Sulfuric Acid",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_UnpackageSulfuricAcid,
        outputs: OUT_UnpackageSulfuricAcid,
    },
    RecipeInfo {
        recipe: Recipe::UnpackageTurbofuel,
        name: "Unpackage Turbofuel",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_UnpackageTurbofuel,
        outputs: OUT_UnpackageTurbofuel,
    },
    RecipeInfo {
        recipe: Recipe::UnpackageWater,
        name: "Unpackage Water",
        machine: crate::models::types::MachineType::Packager,
        inputs: IN_UnpackageWater,
        outputs: OUT_UnpackageWater,
    },
    RecipeInfo {
        recipe: Recipe::UpCornerRamp1m,
        name: "Up Corner Ramp 1m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_UpCornerRamp1m,
        outputs: OUT_UpCornerRamp1m,
    },
    RecipeInfo {
        recipe: Recipe::UpCornerRamp2m,
        name: "Up Corner Ramp 2m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_UpCornerRamp2m,
        outputs: OUT_UpCornerRamp2m,
    },
    RecipeInfo {
        recipe: Recipe::UpCornerRamp4m,
        name: "Up Corner Ramp 4m",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_UpCornerRamp4m,
        outputs: OUT_UpCornerRamp4m,
    },
    RecipeInfo {
        recipe: Recipe::UraniumFuelRod,
        name: "Uranium Fuel Rod",
        machine: crate::models::types::MachineType::Manufacturer,
        inputs: IN_UraniumFuelRod,
        outputs: OUT_UraniumFuelRod,
    },
    RecipeInfo {
        recipe: Recipe::Valve,
        name: "Valve",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Valve,
        outputs: OUT_Valve,
    },
    RecipeInfo {
        recipe: Recipe::VersatileFramework,
        name: "Versatile Framework",
        machine: crate::models::types::MachineType::Assembler,
        inputs: IN_VersatileFramework,
        outputs: OUT_VersatileFramework,
    },
    RecipeInfo {
        recipe: Recipe::VitaminInhaler,
        name: "Vitamin Inhaler",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_VitaminInhaler,
        outputs: OUT_VitaminInhaler,
    },
    RecipeInfo {
        recipe: Recipe::Wall1a,
        name: "Wall 1a",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Wall1a,
        outputs: OUT_Wall1a,
    },
    RecipeInfo {
        recipe: Recipe::WallMountedFloodLight,
        name: "Wall Mounted Flood Light",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_WallMountedFloodLight,
        outputs: OUT_WallMountedFloodLight,
    },
    RecipeInfo {
        recipe: Recipe::WallOutletMk1,
        name: "Wall Outlet Mk.1",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_WallOutletMk1,
        outputs: OUT_WallOutletMk1,
    },
    RecipeInfo {
        recipe: Recipe::WallOutletMk2,
        name: "Wall Outlet Mk.2",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_WallOutletMk2,
        outputs: OUT_WallOutletMk2,
    },
    RecipeInfo {
        recipe: Recipe::WallOutletMk3,
        name: "Wall Outlet Mk.3",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_WallOutletMk3,
        outputs: OUT_WallOutletMk3,
    },
    RecipeInfo {
        recipe: Recipe::WaterExtractor,
        name: "Water Extractor",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_WaterExtractor,
        outputs: OUT_WaterExtractor,
    },
    RecipeInfo {
        recipe: Recipe::Wire,
        name: "Wire",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Wire,
        outputs: OUT_Wire,
    },
    RecipeInfo {
        recipe: Recipe::XenoBasher,
        name: "Xeno-Basher",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_XenoBasher,
        outputs: OUT_XenoBasher,
    },
    RecipeInfo {
        recipe: Recipe::XenoZapper,
        name: "Xeno-Zapper",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_XenoZapper,
        outputs: OUT_XenoZapper,
    },
    RecipeInfo {
        recipe: Recipe::Zipline,
        name: "Zipline",
        machine: crate::models::types::MachineType::Constructor,
        inputs: IN_Zipline,
        outputs: OUT_Zipline,
    },
];

pub fn recipe_by_name(name: &str) -> Option<Recipe> {
    match name {
        "AI Limiter" => Some(Recipe::AILimiter),
        "AWESOME Shop" => Some(Recipe::AWESOMEShop),
        "AWESOME Sink" => Some(Recipe::AWESOMESink),
        "Adaptive Control Unit" => Some(Recipe::AdaptiveControlUnit),
        "Alclad Aluminum Sheet" => Some(Recipe::AlcladAluminumSheet),
        "Alien DNA Capsule" => Some(Recipe::AlienDNACapsule),
        "Alternate: Adhered Iron Plate" => Some(Recipe::AlternateAdheredIronPlate),
        "Alternate: Alclad Casing" => Some(Recipe::AlternateAlcladCasing),
        "Alternate: Automated Miner" => Some(Recipe::AlternateAutomatedMiner),
        "Alternate: Automated Speed Wiring" => Some(Recipe::AlternateAutomatedSpeedWiring),
        "Alternate: Biocoal" => Some(Recipe::AlternateBiocoal),
        "Alternate: Bolted Frame" => Some(Recipe::AlternateBoltedFrame),
        "Alternate: Bolted Iron Plate" => Some(Recipe::AlternateBoltedIronPlate),
        "Alternate: Cast Screw" => Some(Recipe::AlternateCastScrew),
        "Alternate: Caterium Circuit Board" => Some(Recipe::AlternateCateriumCircuitBoard),
        "Alternate: Caterium Computer" => Some(Recipe::AlternateCateriumComputer),
        "Alternate: Caterium Wire" => Some(Recipe::AlternateCateriumWire),
        "Alternate: Charcoal" => Some(Recipe::AlternateCharcoal),
        "Alternate: Cheap Silica" => Some(Recipe::AlternateCheapSilica),
        "Alternate: Classic Battery" => Some(Recipe::AlternateClassicBattery),
        "Alternate: Coated Cable" => Some(Recipe::AlternateCoatedCable),
        "Alternate: Coated Iron Canister" => Some(Recipe::AlternateCoatedIronCanister),
        "Alternate: Coated Iron Plate" => Some(Recipe::AlternateCoatedIronPlate),
        "Alternate: Coke Steel Ingot" => Some(Recipe::AlternateCokeSteelIngot),
        "Alternate: Compacted Coal" => Some(Recipe::AlternateCompactedCoal),
        "Alternate: Compacted Steel Ingot" => Some(Recipe::AlternateCompactedSteelIngot),
        "Alternate: Cooling Device" => Some(Recipe::AlternateCoolingDevice),
        "Alternate: Copper Alloy Ingot" => Some(Recipe::AlternateCopperAlloyIngot),
        "Alternate: Copper Rotor" => Some(Recipe::AlternateCopperRotor),
        "Alternate: Crystal Beacon" => Some(Recipe::AlternateCrystalBeacon),
        "Alternate: Crystal Computer" => Some(Recipe::AlternateCrystalComputer),
        "Alternate: Diluted Fuel" => Some(Recipe::AlternateDilutedFuel),
        "Alternate: Diluted Packaged Fuel" => Some(Recipe::AlternateDilutedPackagedFuel),
        "Alternate: Electric Motor" => Some(Recipe::AlternateElectricMotor),
        "Alternate: Electrode Aluminum Scrap" => Some(Recipe::AlternateElectrodeAluminumScrap),
        "Alternate: Electrode Circuit Board" => Some(Recipe::AlternateElectrodeCircuitBoard),
        "Alternate: Electromagnetic Connection Rod" => {
            Some(Recipe::AlternateElectromagneticConnectionRod)
        }
        "Alternate: Encased Industrial Pipe" => Some(Recipe::AlternateEncasedIndustrialPipe),
        "Alternate: Fertile Uranium" => Some(Recipe::AlternateFertileUranium),
        "Alternate: Fine Black Powder" => Some(Recipe::AlternateFineBlackPowder),
        "Alternate: Fine Concrete" => Some(Recipe::AlternateFineConcrete),
        "Alternate: Flexible Framework" => Some(Recipe::AlternateFlexibleFramework),
        "Alternate: Fused Quickwire" => Some(Recipe::AlternateFusedQuickwire),
        "Alternate: Fused Wire" => Some(Recipe::AlternateFusedWire),
        "Alternate: Heat Exchanger" => Some(Recipe::AlternateHeatExchanger),
        "Alternate: Heat-Fused Frame" => Some(Recipe::AlternateHeatFusedFrame),
        "Alternate: Heavy Encased Frame" => Some(Recipe::AlternateHeavyEncasedFrame),
        "Alternate: Heavy Flexible Frame" => Some(Recipe::AlternateHeavyFlexibleFrame),
        "Alternate: Heavy Oil Residue" => Some(Recipe::AlternateHeavyOilResidue),
        "Alternate: Infused Uranium Cell" => Some(Recipe::AlternateInfusedUraniumCell),
        "Alternate: Instant Plutonium Cell" => Some(Recipe::AlternateInstantPlutoniumCell),
        "Alternate: Instant Scrap" => Some(Recipe::AlternateInstantScrap),
        "Alternate: Insulated Cable" => Some(Recipe::AlternateInsulatedCable),
        "Alternate: Insulated Crystal Oscillator" => {
            Some(Recipe::AlternateInsulatedCrystalOscillator)
        }
        "Alternate: Iron Alloy Ingot" => Some(Recipe::AlternateIronAlloyIngot),
        "Alternate: Iron Wire" => Some(Recipe::AlternateIronWire),
        "Alternate: OC Supercomputer" => Some(Recipe::AlternateOCSupercomputer),
        "Alternate: Plastic Smart Plating" => Some(Recipe::AlternatePlasticSmartPlating),
        "Alternate: Plutonium Fuel Unit" => Some(Recipe::AlternatePlutoniumFuelUnit),
        "Alternate: Polyester Fabric" => Some(Recipe::AlternatePolyesterFabric),
        "Alternate: Polymer Resin" => Some(Recipe::AlternatePolymerResin),
        "Alternate: Pure Aluminum Ingot" => Some(Recipe::AlternatePureAluminumIngot),
        "Alternate: Pure Caterium Ingot" => Some(Recipe::AlternatePureCateriumIngot),
        "Alternate: Pure Copper Ingot" => Some(Recipe::AlternatePureCopperIngot),
        "Alternate: Pure Iron Ingot" => Some(Recipe::AlternatePureIronIngot),
        "Alternate: Pure Quartz Crystal" => Some(Recipe::AlternatePureQuartzCrystal),
        "Alternate: Quickwire Cable" => Some(Recipe::AlternateQuickwireCable),
        "Alternate: Quickwire Stator" => Some(Recipe::AlternateQuickwireStator),
        "Alternate: Radio Connection Unit" => Some(Recipe::AlternateRadioConnectionUnit),
        "Alternate: Radio Control System" => Some(Recipe::AlternateRadioControlSystem),
        "Alternate: Recycled Plastic" => Some(Recipe::AlternateRecycledPlastic),
        "Alternate: Recycled Rubber" => Some(Recipe::AlternateRecycledRubber),
        "Alternate: Rigour Motor" => Some(Recipe::AlternateRigourMotor),
        "Alternate: Rubber Concrete" => Some(Recipe::AlternateRubberConcrete),
        "Alternate: Silicon Circuit Board" => Some(Recipe::AlternateSiliconCircuitBoard),
        "Alternate: Silicon High-Speed Connector" => {
            Some(Recipe::AlternateSiliconHighSpeedConnector)
        }
        "Alternate: Sloppy Alumina" => Some(Recipe::AlternateSloppyAlumina),
        "Alternate: Solid Steel Ingot" => Some(Recipe::AlternateSolidSteelIngot),
        "Alternate: Steamed Copper Sheet" => Some(Recipe::AlternateSteamedCopperSheet),
        "Alternate: Steel Canister" => Some(Recipe::AlternateSteelCanister),
        "Alternate: Steel Coated Plate" => Some(Recipe::AlternateSteelCoatedPlate),
        "Alternate: Steel Rod" => Some(Recipe::AlternateSteelRod),
        "Alternate: Steel Rotor" => Some(Recipe::AlternateSteelRotor),
        "Alternate: Steel Screw" => Some(Recipe::AlternateSteelScrew),
        "Alternate: Steeled Frame" => Some(Recipe::AlternateSteeledFrame),
        "Alternate: Stitched Iron Plate" => Some(Recipe::AlternateStitchedIronPlate),
        "Alternate: Super-State Computer" => Some(Recipe::AlternateSuperStateComputer),
        "Alternate: Turbo Blend Fuel" => Some(Recipe::AlternateTurboBlendFuel),
        "Alternate: Turbo Electric Motor" => Some(Recipe::AlternateTurboElectricMotor),
        "Alternate: Turbo Heavy Fuel" => Some(Recipe::AlternateTurboHeavyFuel),
        "Alternate: Turbo Pressure Motor" => Some(Recipe::AlternateTurboPressureMotor),
        "Alternate: Uranium Fuel Unit" => Some(Recipe::AlternateUraniumFuelUnit),
        "Alternate: Wet Concrete" => Some(Recipe::AlternateWetConcrete),
        "Alumina Solution" => Some(Recipe::AluminaSolution),
        "Aluminum Casing" => Some(Recipe::AluminumCasing),
        "Aluminum Ingot" => Some(Recipe::AluminumIngot),
        "Aluminum Scrap" => Some(Recipe::AluminumScrap),
        "Assembler" => Some(Recipe::Assembler),
        "Assembly Director System" => Some(Recipe::AssemblyDirectorSystem),
        "Automated Gate" => Some(Recipe::AutomatedGate),
        "Automated Wiring" => Some(Recipe::AutomatedWiring),
        "Basic Wall 1m" => Some(Recipe::BasicWall1m),
        "Basic Wall 4m" => Some(Recipe::BasicWall4m),
        "Battery" => Some(Recipe::Battery),
        "Beacon" => Some(Recipe::Beacon),
        "Beam Connector" => Some(Recipe::BeamConnector),
        "Beam Connector Double" => Some(Recipe::BeamConnectorDouble),
        "Beam Support" => Some(Recipe::BeamSupport),
        "Big Concrete Pillar" => Some(Recipe::BigConcretePillar),
        "Big Frame Pillar" => Some(Recipe::BigFramePillar),
        "Big Metal Pillar" => Some(Recipe::BigMetalPillar),
        "Big Pillar Support" => Some(Recipe::BigPillarSupport),
        "Biomass (Alien Protein)" => Some(Recipe::BiomassAlienProtein),
        "Biomass Burner" => Some(Recipe::BiomassBurner),
        "Biomass (Leaves)" => Some(Recipe::BiomassLeaves),
        "Biomass (Mycelia)" => Some(Recipe::BiomassMycelia),
        "Biomass (Wood)" => Some(Recipe::BiomassWood),
        "Black Powder" => Some(Recipe::BlackPowder),
        "Blade Runners" => Some(Recipe::BladeRunners),
        "Blender" => Some(Recipe::Blender),
        "Block Signal" => Some(Recipe::BlockSignal),
        "Blueprint Designer" => Some(Recipe::BlueprintDesigner),
        "Cable" => Some(Recipe::Cable),
        "Candy Cane" => Some(Recipe::CandyCane),
        "Caterium Ingot" => Some(Recipe::CateriumIngot),
        "Catwalk Corner" => Some(Recipe::CatwalkCorner),
        "Catwalk Crossing" => Some(Recipe::CatwalkCrossing),
        "Catwalk Ramp" => Some(Recipe::CatwalkRamp),
        "Catwalk Stairs" => Some(Recipe::CatwalkStairs),
        "Catwalk Straight" => Some(Recipe::CatwalkStraight),
        "Catwalk T-Crossing" => Some(Recipe::CatwalkTCrossing),
        "Ceiling Light" => Some(Recipe::CeilingLight),
        "Center Door Wall" => Some(Recipe::CenterDoorWall),
        "Chainsaw" => Some(Recipe::Chainsaw),
        "Circuit Board" => Some(Recipe::CircuitBoard),
        "Cluster Nobelisk" => Some(Recipe::ClusterNobelisk),
        "Coal Generator" => Some(Recipe::CoalGenerator),
        "Color Cartridge" => Some(Recipe::ColorCartridge),
        "Computer" => Some(Recipe::Computer),
        "Concrete" => Some(Recipe::Concrete),
        "Constructor" => Some(Recipe::Constructor),
        "Conveyor Belt Mk.1" => Some(Recipe::ConveyorBeltMk1),
        "Conveyor Belt Mk.2" => Some(Recipe::ConveyorBeltMk2),
        "Conveyor Belt Mk.3" => Some(Recipe::ConveyorBeltMk3),
        "Conveyor Belt Mk.4" => Some(Recipe::ConveyorBeltMk4),
        "Conveyor Belt Mk.5" => Some(Recipe::ConveyorBeltMk5),
        "Conveyor Ceiling Mount" => Some(Recipe::ConveyorCeilingMount),
        "Conveyor Lift Floor Hole" => Some(Recipe::ConveyorLiftFloorHole),
        "Conveyor Lift Mk.1" => Some(Recipe::ConveyorLiftMk1),
        "Conveyor Lift Mk.2" => Some(Recipe::ConveyorLiftMk2),
        "Conveyor Lift Mk.3" => Some(Recipe::ConveyorLiftMk3),
        "Conveyor Lift Mk.4" => Some(Recipe::ConveyorLiftMk4),
        "Conveyor Lift Mk.5" => Some(Recipe::ConveyorLiftMk5),
        "Conveyor Merger" => Some(Recipe::ConveyorMerger),
        "Conveyor Pole" => Some(Recipe::ConveyorPole),
        "Conveyor Splitter" => Some(Recipe::ConveyorSplitter),
        "Conveyor Wall Mount" => Some(Recipe::ConveyorWallMount),
        "Conveyor Wall x1" => Some(Recipe::ConveyorWallX1),
        "Conveyor Wall x2" => Some(Recipe::ConveyorWallX2),
        "Conveyor Wall x3" => Some(Recipe::ConveyorWallX3),
        "Cooling System" => Some(Recipe::CoolingSystem),
        "Copper Ingot" => Some(Recipe::CopperIngot),
        "Copper Powder" => Some(Recipe::CopperPowder),
        "Copper Sheet" => Some(Recipe::CopperSheet),
        "Craft Bench" => Some(Recipe::CraftBench),
        "Crystal Oscillator" => Some(Recipe::CrystalOscillator),
        "Cyber Wagon" => Some(Recipe::CyberWagon),
        "Display Sign" => Some(Recipe::DisplaySign),
        "Double Ramp 2m" => Some(Recipe::DoubleRamp2m),
        "Double Ramp 4m" => Some(Recipe::DoubleRamp4m),
        "Double Ramp 8m" => Some(Recipe::DoubleRamp8m),
        "Double Wall Outlet Mk.1" => Some(Recipe::DoubleWallOutletMk1),
        "Double Wall Outlet Mk.2" => Some(Recipe::DoubleWallOutletMk2),
        "Double Wall Outlet Mk.3" => Some(Recipe::DoubleWallOutletMk3),
        "Down Corner Ramp 1m" => Some(Recipe::DownCornerRamp1m),
        "Down Corner Ramp 2m" => Some(Recipe::DownCornerRamp2m),
        "Down Corner Ramp 4m" => Some(Recipe::DownCornerRamp4m),
        "Drone" => Some(Recipe::Drone),
        "Drone Port" => Some(Recipe::DronePort),
        "Electric Locomotive" => Some(Recipe::ElectricLocomotive),
        "Electromagnetic Control Rod" => Some(Recipe::ElectromagneticControlRod),
        "Empty Canister" => Some(Recipe::EmptyCanister),
        "Empty Fluid Tank" => Some(Recipe::EmptyFluidTank),
        "Empty Platform" => Some(Recipe::EmptyPlatform),
        "Empty Platform With Catwalk" => Some(Recipe::EmptyPlatformWithCatwalk),
        "Encased Industrial Beam" => Some(Recipe::EncasedIndustrialBeam),
        "Encased Plutonium Cell" => Some(Recipe::EncasedPlutoniumCell),
        "Encased Uranium Cell" => Some(Recipe::EncasedUraniumCell),
        "Equipment Workshop" => Some(Recipe::EquipmentWorkshop),
        "Explorer" => Some(Recipe::Explorer),
        "Explosive Rebar" => Some(Recipe::ExplosiveRebar),
        "FICSMAS Gift Tree" => Some(Recipe::FICSMASGiftTree),
        "FICSMAS Power Light" => Some(Recipe::FICSMASPowerLight),
        "FICSMAS Snow Dispenser" => Some(Recipe::FICSMASSnowDispenser),
        "FICSMAS Wreath" => Some(Recipe::FICSMASWreath),
        "Fabric" => Some(Recipe::Fabric),
        "Factory Cart™" => Some(Recipe::FactoryCart),
        "Flood Light Tower" => Some(Recipe::FloodLightTower),
        "Fluid Buffer" => Some(Recipe::FluidBuffer),
        "Fluid Freight Platform" => Some(Recipe::FluidFreightPlatform),
        "Foundation 1m" => Some(Recipe::Foundation1m),
        "Foundation 2m" => Some(Recipe::Foundation2m),
        "Foundation 4m" => Some(Recipe::Foundation4m),
        "Foundry" => Some(Recipe::Foundry),
        "Frame Floor" => Some(Recipe::FrameFloor),
        "Frame Foundation" => Some(Recipe::FrameFoundation),
        "Frame Ramp" => Some(Recipe::FrameRamp),
        "Frame Wall" => Some(Recipe::FrameWall),
        "Frame Window" => Some(Recipe::FrameWindow),
        "Freight Car" => Some(Recipe::FreightCar),
        "Freight Platform" => Some(Recipe::FreightPlatform),
        "Fuel" => Some(Recipe::Fuel),
        "Fuel Generator" => Some(Recipe::FuelGenerator),
        "Full Frame Window" => Some(Recipe::FullFrameWindow),
        "Fused Modular Frame" => Some(Recipe::FusedModularFrame),
        "Gas Filter" => Some(Recipe::GasFilter),
        "Gas Mask" => Some(Recipe::GasMask),
        "Gas Nobelisk" => Some(Recipe::GasNobelisk),
        "Gate Hole Wall" => Some(Recipe::GateHoleWall),
        "Geothermal Generator" => Some(Recipe::GeothermalGenerator),
        "Giant FICSMAS Tree" => Some(Recipe::GiantFICSMASTree),
        "Glass Frame Foundation" => Some(Recipe::GlassFrameFoundation),
        "Golden Factory Cart™" => Some(Recipe::GoldenFactoryCart),
        "Half 1m Foundation" => Some(Recipe::Half1mFoundation),
        "Half 2m Foundation" => Some(Recipe::Half2mFoundation),
        "Half 4m Foundation" => Some(Recipe::Half4mFoundation),
        "Hatcher Protein" => Some(Recipe::HatcherProtein),
        "Hazard Storage Box" => Some(Recipe::HazardStorageBox),
        "Hazmat Suit" => Some(Recipe::HazmatSuit),
        "Heat Sink" => Some(Recipe::HeatSink),
        "Heavy Modular Frame" => Some(Recipe::HeavyModularFrame),
        "Hex Frame Window" => Some(Recipe::HexFrameWindow),
        "High-Speed Connector" => Some(Recipe::HighSpeedConnector),
        "Hog Protein" => Some(Recipe::HogProtein),
        "Homing Rifle Ammo" => Some(Recipe::HomingRifleAmmo),
        "Hover Pack" => Some(Recipe::HoverPack),
        "Hypertube" => Some(Recipe::Hypertube),
        "Hypertube Entrance" => Some(Recipe::HypertubeEntrance),
        "Hypertube Floor Hole" => Some(Recipe::HypertubeFloorHole),
        "Hypertube Support" => Some(Recipe::HypertubeSupport),
        "Hypertube Wall Hole" => Some(Recipe::HypertubeWallHole),
        "Hypertube Wall Support" => Some(Recipe::HypertubeWallSupport),
        "Industrial Fluid Buffer" => Some(Recipe::IndustrialFluidBuffer),
        "Industrial Storage Container" => Some(Recipe::IndustrialStorageContainer),
        "Inner Corner Extension 1m" => Some(Recipe::InnerCornerExtension1m),
        "Inner Corner Extension 2m" => Some(Recipe::InnerCornerExtension2m),
        "Inner Corner Extension 4m" => Some(Recipe::InnerCornerExtension4m),
        "Inner Corner Quarter Pipe" => Some(Recipe::InnerCornerQuarterPipe),
        "Inner Corner Roof 1m" => Some(Recipe::InnerCornerRoof1m),
        "Inner Corner Roof 2m" => Some(Recipe::InnerCornerRoof2m),
        "Inner Corner Roof 4m" => Some(Recipe::InnerCornerRoof4m),
        "Inv. Down Corner 1m" => Some(Recipe::InvDownCorner1m),
        "Inv. Down Corner 2m" => Some(Recipe::InvDownCorner2m),
        "Inv. Down Corner 4m" => Some(Recipe::InvDownCorner4m),
        "Inv. Ramp 1m" => Some(Recipe::InvRamp1m),
        "Inv. Ramp 2m" => Some(Recipe::InvRamp2m),
        "Inv. Ramp 4m" => Some(Recipe::InvRamp4m),
        "Inv. Ramp Wall 1m" => Some(Recipe::InvRampWall1m),
        "Inv. Ramp Wall 2m" => Some(Recipe::InvRampWall2m),
        "Inv. Ramp Wall 4m" => Some(Recipe::InvRampWall4m),
        "Inv. Ramp Wall 8m" => Some(Recipe::InvRampWall8m),
        "Inv. Up Corner 1m" => Some(Recipe::InvUpCorner1m),
        "Inv. Up Corner 2m" => Some(Recipe::InvUpCorner2m),
        "Inv. Up Corner 4m" => Some(Recipe::InvUpCorner4m),
        "Inverted Frame Ramp" => Some(Recipe::InvertedFrameRamp),
        "Inverted Inner Corner Quarter Pipe" => Some(Recipe::InvertedInnerCornerQuarterPipe),
        "Inverted Outer Corner Quarter Pipe" => Some(Recipe::InvertedOuterCornerQuarterPipe),
        "Inverted Quarter Pipe" => Some(Recipe::InvertedQuarterPipe),
        "Iodine Infused Filter" => Some(Recipe::IodineInfusedFilter),
        "Iron Ingot" => Some(Recipe::IronIngot),
        "Iron Plate" => Some(Recipe::IronPlate),
        "Iron Rebar" => Some(Recipe::IronRebar),
        "Iron Rod" => Some(Recipe::IronRod),
        "Jetpack" => Some(Recipe::Jetpack),
        "Jump Pad" => Some(Recipe::JumpPad),
        "Label Sign 2m" => Some(Recipe::LabelSign2m),
        "Label Sign 3m" => Some(Recipe::LabelSign3m),
        "Label Sign 4m" => Some(Recipe::LabelSign4m),
        "Ladder" => Some(Recipe::Ladder),
        "Large Billboard" => Some(Recipe::LargeBillboard),
        "Lights Control Panel" => Some(Recipe::LightsControlPanel),
        "Liquid Biofuel" => Some(Recipe::LiquidBiofuel),
        "Lookout Tower" => Some(Recipe::LookoutTower),
        "MAM" => Some(Recipe::MAM),
        "Magnetic Field Generator" => Some(Recipe::MagneticFieldGenerator),
        "Manufacturer" => Some(Recipe::Manufacturer),
        "Medical Storage Box" => Some(Recipe::MedicalStorageBox),
        "Metal Beam" => Some(Recipe::MetalBeam),
        "Miner Mk.1" => Some(Recipe::MinerMk1),
        "Miner Mk.2" => Some(Recipe::MinerMk2),
        "Miner Mk.3" => Some(Recipe::MinerMk3),
        "Modern Railing" => Some(Recipe::ModernRailing),
        "Modular Engine" => Some(Recipe::ModularEngine),
        "Modular Frame" => Some(Recipe::ModularFrame),
        "Motor" => Some(Recipe::Motor),
        "Nitric Acid" => Some(Recipe::NitricAcid),
        "Nobelisk" => Some(Recipe::Nobelisk),
        "Nobelisk Detonator" => Some(Recipe::NobeliskDetonator),
        "Non-fissile Uranium" => Some(Recipe::NonFissileUranium),
        "Nuclear Pasta" => Some(Recipe::NuclearPasta),
        "Nuclear Power Plant" => Some(Recipe::NuclearPowerPlant),
        "Nuke Nobelisk" => Some(Recipe::NukeNobelisk),
        "Nutritional Inhaler" => Some(Recipe::NutritionalInhaler),
        "Object Scanner" => Some(Recipe::ObjectScanner),
        "Oil Extractor" => Some(Recipe::OilExtractor),
        "Outer Corner Extension 1m" => Some(Recipe::OuterCornerExtension1m),
        "Outer Corner Extension 2m" => Some(Recipe::OuterCornerExtension2m),
        "Outer Corner Extension 4m" => Some(Recipe::OuterCornerExtension4m),
        "Outer Corner Quarter Pipe" => Some(Recipe::OuterCornerQuarterPipe),
        "Outer Corner Roof 1m" => Some(Recipe::OuterCornerRoof1m),
        "Outer Corner Roof 2m" => Some(Recipe::OuterCornerRoof2m),
        "Outer Corner Roof 4m" => Some(Recipe::OuterCornerRoof4m),
        "Packaged Alumina Solution" => Some(Recipe::PackagedAluminaSolution),
        "Packaged Fuel" => Some(Recipe::PackagedFuel),
        "Packaged Heavy Oil Residue" => Some(Recipe::PackagedHeavyOilResidue),
        "Packaged Liquid Biofuel" => Some(Recipe::PackagedLiquidBiofuel),
        "Packaged Nitric Acid" => Some(Recipe::PackagedNitricAcid),
        "Packaged Nitrogen Gas" => Some(Recipe::PackagedNitrogenGas),
        "Packaged Oil" => Some(Recipe::PackagedOil),
        "Packaged Sulfuric Acid" => Some(Recipe::PackagedSulfuricAcid),
        "Packaged Turbofuel" => Some(Recipe::PackagedTurbofuel),
        "Packaged Water" => Some(Recipe::PackagedWater),
        "Packager" => Some(Recipe::Packager),
        "Painted Beam" => Some(Recipe::PaintedBeam),
        "Panel Window" => Some(Recipe::PanelWindow),
        "Parachute" => Some(Recipe::Parachute),
        "Particle Accelerator" => Some(Recipe::ParticleAccelerator),
        "Path Signal" => Some(Recipe::PathSignal),
        "Personal Storage Box" => Some(Recipe::PersonalStorageBox),
        "Petroleum Coke" => Some(Recipe::PetroleumCoke),
        "Pipeline Floor Hole" => Some(Recipe::PipelineFloorHole),
        "Pipeline Junction Cross" => Some(Recipe::PipelineJunctionCross),
        "Pipeline Mk.1" => Some(Recipe::PipelineMk1),
        "Pipeline Mk.1 (No Indicator)" => Some(Recipe::PipelineMk1NoIndicator),
        "Pipeline Mk.2" => Some(Recipe::PipelineMk2),
        "Pipeline Mk.2 (No Indicator)" => Some(Recipe::PipelineMk2NoIndicator),
        "Pipeline Pump Mk.1" => Some(Recipe::PipelinePumpMk1),
        "Pipeline Pump Mk.2" => Some(Recipe::PipelinePumpMk2),
        "Pipeline Support" => Some(Recipe::PipelineSupport),
        "Pipeline Wall Hole" => Some(Recipe::PipelineWallHole),
        "Pipeline Wall Support" => Some(Recipe::PipelineWallSupport),
        "Plastic" => Some(Recipe::Plastic),
        "Plutonium Fuel Rod" => Some(Recipe::PlutoniumFuelRod),
        "Plutonium Pellet" => Some(Recipe::PlutoniumPellet),
        "Portable Miner" => Some(Recipe::PortableMiner),
        "Portrait Sign" => Some(Recipe::PortraitSign),
        "Power Line" => Some(Recipe::PowerLine),
        "Power Pole Mk.1" => Some(Recipe::PowerPoleMk1),
        "Power Pole Mk.2" => Some(Recipe::PowerPoleMk2),
        "Power Pole Mk.3" => Some(Recipe::PowerPoleMk3),
        "Power Shard (1)" => Some(Recipe::PowerShard1),
        "Power Shard (2)" => Some(Recipe::PowerShard2),
        "Power Shard (5)" => Some(Recipe::PowerShard5),
        "Power Storage" => Some(Recipe::PowerStorage),
        "Power Switch" => Some(Recipe::PowerSwitch),
        "Power Tower" => Some(Recipe::PowerTower),
        "Power Tower Platform" => Some(Recipe::PowerTowerPlatform),
        "Pressure Conversion Cube" => Some(Recipe::PressureConversionCube),
        "Priority Power Switch" => Some(Recipe::PriorityPowerSwitch),
        "Programmable Splitter" => Some(Recipe::ProgrammableSplitter),
        "Protein Inhaler" => Some(Recipe::ProteinInhaler),
        "Pulse Nobelisk" => Some(Recipe::PulseNobelisk),
        "Quarter Pipe" => Some(Recipe::QuarterPipe),
        "Quartz Crystal" => Some(Recipe::QuartzCrystal),
        "Quickwire" => Some(Recipe::Quickwire),
        "Radar Tower" => Some(Recipe::RadarTower),
        "Radio Control Unit" => Some(Recipe::RadioControlUnit),
        "Railway" => Some(Recipe::Railway),
        "Ramp 1m" => Some(Recipe::Ramp1m),
        "Ramp 2m" => Some(Recipe::Ramp2m),
        "Ramp 4m" => Some(Recipe::Ramp4m),
        "Ramp Wall 1m" => Some(Recipe::RampWall1m),
        "Ramp Wall 2m" => Some(Recipe::RampWall2m),
        "Ramp Wall 4m" => Some(Recipe::RampWall4m),
        "Ramp Wall 8m" => Some(Recipe::RampWall8m),
        "Rebar Gun" => Some(Recipe::RebarGun),
        "Refinery" => Some(Recipe::Refinery),
        "Reinforced Iron Plate" => Some(Recipe::ReinforcedIronPlate),
        "Reinforced Window" => Some(Recipe::ReinforcedWindow),
        "Residual Fuel" => Some(Recipe::ResidualFuel),
        "Residual Plastic" => Some(Recipe::ResidualPlastic),
        "Residual Rubber" => Some(Recipe::ResidualRubber),
        "Resource Well Extractor" => Some(Recipe::ResourceWellExtractor),
        "Resource Well Pressurizer" => Some(Recipe::ResourceWellPressurizer),
        "Rifle" => Some(Recipe::Rifle),
        "Rifle Ammo" => Some(Recipe::RifleAmmo),
        "Road Barrier" => Some(Recipe::RoadBarrier),
        "Roof 1m" => Some(Recipe::Roof1m),
        "Roof 2m" => Some(Recipe::Roof2m),
        "Roof 4m" => Some(Recipe::Roof4m),
        "Roof Flat" => Some(Recipe::RoofFlat),
        "Rotor" => Some(Recipe::Rotor),
        "Rubber" => Some(Recipe::Rubber),
        "Screw" => Some(Recipe::Screw),
        "Shatter Rebar" => Some(Recipe::ShatterRebar),
        "Side Door Wall" => Some(Recipe::SideDoorWall),
        "Silica" => Some(Recipe::Silica),
        "Single Window" => Some(Recipe::SingleWindow),
        "Small Billboard" => Some(Recipe::SmallBillboard),
        "Small Concrete Pillar" => Some(Recipe::SmallConcretePillar),
        "Small Frame Pillar" => Some(Recipe::SmallFramePillar),
        "Small Metal Pillar" => Some(Recipe::SmallMetalPillar),
        "Small Pillar Support" => Some(Recipe::SmallPillarSupport),
        "Smart Plating" => Some(Recipe::SmartPlating),
        "Smart Splitter" => Some(Recipe::SmartSplitter),
        "Smelter" => Some(Recipe::Smelter),
        "Smokeless Powder" => Some(Recipe::SmokelessPowder),
        "Snowman" => Some(Recipe::Snowman),
        "Solid Biofuel" => Some(Recipe::SolidBiofuel),
        "Space Elevator" => Some(Recipe::SpaceElevator),
        "Spitter Protein" => Some(Recipe::SpitterProtein),
        "Square Sign 0.5m" => Some(Recipe::SquareSign05m),
        "Square Sign 1m" => Some(Recipe::SquareSign1m),
        "Square Sign 2m" => Some(Recipe::SquareSign2m),
        "Stackable Conveyor Pole" => Some(Recipe::StackableConveyorPole),
        "Stackable Hypertube Support" => Some(Recipe::StackableHypertubeSupport),
        "Stackable Pipeline Support" => Some(Recipe::StackablePipelineSupport),
        "Stairs Left" => Some(Recipe::StairsLeft),
        "Stairs Right" => Some(Recipe::StairsRight),
        "Stator" => Some(Recipe::Stator),
        "Steel Beam" => Some(Recipe::SteelBeam),
        "Steel Ingot" => Some(Recipe::SteelIngot),
        "Steel Pipe" => Some(Recipe::SteelPipe),
        "Stinger Protein" => Some(Recipe::StingerProtein),
        "Storage Container" => Some(Recipe::StorageContainer),
        "Street Light" => Some(Recipe::StreetLight),
        "Stun Rebar" => Some(Recipe::StunRebar),
        "Sulfuric Acid" => Some(Recipe::SulfuricAcid),
        "Supercomputer" => Some(Recipe::Supercomputer),
        "The HUB" => Some(Recipe::TheHUB),
        "Therapeutic Inhaler" => Some(Recipe::TherapeuticInhaler),
        "Thermal Propulsion Rocket" => Some(Recipe::ThermalPropulsionRocket),
        "Tilted Concave Wall 4m" => Some(Recipe::TiltedConcaveWall4m),
        "Tilted Concave Wall 8m" => Some(Recipe::TiltedConcaveWall8m),
        "Tilted Corner Wall 4m" => Some(Recipe::TiltedCornerWall4m),
        "Tilted Corner Wall 8m" => Some(Recipe::TiltedCornerWall8m),
        "Tilted Wall 4m" => Some(Recipe::TiltedWall4m),
        "Tilted Wall 8m" => Some(Recipe::TiltedWall8m),
        "Tractor" => Some(Recipe::Tractor),
        "Train Station" => Some(Recipe::TrainStation),
        "Truck" => Some(Recipe::Truck),
        "Truck Station" => Some(Recipe::TruckStation),
        "Turbo Motor" => Some(Recipe::TurboMotor),
        "Turbo Rifle Ammo" => Some(Recipe::TurboRifleAmmo),
        "Turbofuel" => Some(Recipe::Turbofuel),
        "U-Jelly Landing Pad" => Some(Recipe::UJellyLandingPad),
        "Unpackage Alumina Solution" => Some(Recipe::UnpackageAluminaSolution),
        "Unpackage Fuel" => Some(Recipe::UnpackageFuel),
        "Unpackage Heavy Oil Residue" => Some(Recipe::UnpackageHeavyOilResidue),
        "Unpackage Liquid Biofuel" => Some(Recipe::UnpackageLiquidBiofuel),
        "Unpackage Nitric Acid" => Some(Recipe::UnpackageNitricAcid),
        "Unpackage Nitrogen Gas" => Some(Recipe::UnpackageNitrogenGas),
        "Unpackage Oil" => Some(Recipe::UnpackageOil),
        "Unpackage Sulfuric Acid" => Some(Recipe::UnpackageSulfuricAcid),
        "Unpackage Turbofuel" => Some(Recipe::UnpackageTurbofuel),
        "Unpackage Water" => Some(Recipe::UnpackageWater),
        "Up Corner Ramp 1m" => Some(Recipe::UpCornerRamp1m),
        "Up Corner Ramp 2m" => Some(Recipe::UpCornerRamp2m),
        "Up Corner Ramp 4m" => Some(Recipe::UpCornerRamp4m),
        "Uranium Fuel Rod" => Some(Recipe::UraniumFuelRod),
        "Valve" => Some(Recipe::Valve),
        "Versatile Framework" => Some(Recipe::VersatileFramework),
        "Vitamin Inhaler" => Some(Recipe::VitaminInhaler),
        "Wall 1a" => Some(Recipe::Wall1a),
        "Wall Mounted Flood Light" => Some(Recipe::WallMountedFloodLight),
        "Wall Outlet Mk.1" => Some(Recipe::WallOutletMk1),
        "Wall Outlet Mk.2" => Some(Recipe::WallOutletMk2),
        "Wall Outlet Mk.3" => Some(Recipe::WallOutletMk3),
        "Water Extractor" => Some(Recipe::WaterExtractor),
        "Wire" => Some(Recipe::Wire),
        "Xeno-Basher" => Some(Recipe::XenoBasher),
        "Xeno-Zapper" => Some(Recipe::XenoZapper),
        "Zipline" => Some(Recipe::Zipline),
        _ => None,
    }
}

pub fn recipe_name(r: Recipe) -> &'static str {
    match r {
        Recipe::AILimiter => "AI Limiter",
        Recipe::AWESOMEShop => "AWESOME Shop",
        Recipe::AWESOMESink => "AWESOME Sink",
        Recipe::AdaptiveControlUnit => "Adaptive Control Unit",
        Recipe::AlcladAluminumSheet => "Alclad Aluminum Sheet",
        Recipe::AlienDNACapsule => "Alien DNA Capsule",
        Recipe::AlternateAdheredIronPlate => "Alternate: Adhered Iron Plate",
        Recipe::AlternateAlcladCasing => "Alternate: Alclad Casing",
        Recipe::AlternateAutomatedMiner => "Alternate: Automated Miner",
        Recipe::AlternateAutomatedSpeedWiring => "Alternate: Automated Speed Wiring",
        Recipe::AlternateBiocoal => "Alternate: Biocoal",
        Recipe::AlternateBoltedFrame => "Alternate: Bolted Frame",
        Recipe::AlternateBoltedIronPlate => "Alternate: Bolted Iron Plate",
        Recipe::AlternateCastScrew => "Alternate: Cast Screw",
        Recipe::AlternateCateriumCircuitBoard => "Alternate: Caterium Circuit Board",
        Recipe::AlternateCateriumComputer => "Alternate: Caterium Computer",
        Recipe::AlternateCateriumWire => "Alternate: Caterium Wire",
        Recipe::AlternateCharcoal => "Alternate: Charcoal",
        Recipe::AlternateCheapSilica => "Alternate: Cheap Silica",
        Recipe::AlternateClassicBattery => "Alternate: Classic Battery",
        Recipe::AlternateCoatedCable => "Alternate: Coated Cable",
        Recipe::AlternateCoatedIronCanister => "Alternate: Coated Iron Canister",
        Recipe::AlternateCoatedIronPlate => "Alternate: Coated Iron Plate",
        Recipe::AlternateCokeSteelIngot => "Alternate: Coke Steel Ingot",
        Recipe::AlternateCompactedCoal => "Alternate: Compacted Coal",
        Recipe::AlternateCompactedSteelIngot => "Alternate: Compacted Steel Ingot",
        Recipe::AlternateCoolingDevice => "Alternate: Cooling Device",
        Recipe::AlternateCopperAlloyIngot => "Alternate: Copper Alloy Ingot",
        Recipe::AlternateCopperRotor => "Alternate: Copper Rotor",
        Recipe::AlternateCrystalBeacon => "Alternate: Crystal Beacon",
        Recipe::AlternateCrystalComputer => "Alternate: Crystal Computer",
        Recipe::AlternateDilutedFuel => "Alternate: Diluted Fuel",
        Recipe::AlternateDilutedPackagedFuel => "Alternate: Diluted Packaged Fuel",
        Recipe::AlternateElectricMotor => "Alternate: Electric Motor",
        Recipe::AlternateElectrodeAluminumScrap => "Alternate: Electrode Aluminum Scrap",
        Recipe::AlternateElectrodeCircuitBoard => "Alternate: Electrode Circuit Board",
        Recipe::AlternateElectromagneticConnectionRod => {
            "Alternate: Electromagnetic Connection Rod"
        }
        Recipe::AlternateEncasedIndustrialPipe => "Alternate: Encased Industrial Pipe",
        Recipe::AlternateFertileUranium => "Alternate: Fertile Uranium",
        Recipe::AlternateFineBlackPowder => "Alternate: Fine Black Powder",
        Recipe::AlternateFineConcrete => "Alternate: Fine Concrete",
        Recipe::AlternateFlexibleFramework => "Alternate: Flexible Framework",
        Recipe::AlternateFusedQuickwire => "Alternate: Fused Quickwire",
        Recipe::AlternateFusedWire => "Alternate: Fused Wire",
        Recipe::AlternateHeatExchanger => "Alternate: Heat Exchanger",
        Recipe::AlternateHeatFusedFrame => "Alternate: Heat-Fused Frame",
        Recipe::AlternateHeavyEncasedFrame => "Alternate: Heavy Encased Frame",
        Recipe::AlternateHeavyFlexibleFrame => "Alternate: Heavy Flexible Frame",
        Recipe::AlternateHeavyOilResidue => "Alternate: Heavy Oil Residue",
        Recipe::AlternateInfusedUraniumCell => "Alternate: Infused Uranium Cell",
        Recipe::AlternateInstantPlutoniumCell => "Alternate: Instant Plutonium Cell",
        Recipe::AlternateInstantScrap => "Alternate: Instant Scrap",
        Recipe::AlternateInsulatedCable => "Alternate: Insulated Cable",
        Recipe::AlternateInsulatedCrystalOscillator => "Alternate: Insulated Crystal Oscillator",
        Recipe::AlternateIronAlloyIngot => "Alternate: Iron Alloy Ingot",
        Recipe::AlternateIronWire => "Alternate: Iron Wire",
        Recipe::AlternateOCSupercomputer => "Alternate: OC Supercomputer",
        Recipe::AlternatePlasticSmartPlating => "Alternate: Plastic Smart Plating",
        Recipe::AlternatePlutoniumFuelUnit => "Alternate: Plutonium Fuel Unit",
        Recipe::AlternatePolyesterFabric => "Alternate: Polyester Fabric",
        Recipe::AlternatePolymerResin => "Alternate: Polymer Resin",
        Recipe::AlternatePureAluminumIngot => "Alternate: Pure Aluminum Ingot",
        Recipe::AlternatePureCateriumIngot => "Alternate: Pure Caterium Ingot",
        Recipe::AlternatePureCopperIngot => "Alternate: Pure Copper Ingot",
        Recipe::AlternatePureIronIngot => "Alternate: Pure Iron Ingot",
        Recipe::AlternatePureQuartzCrystal => "Alternate: Pure Quartz Crystal",
        Recipe::AlternateQuickwireCable => "Alternate: Quickwire Cable",
        Recipe::AlternateQuickwireStator => "Alternate: Quickwire Stator",
        Recipe::AlternateRadioConnectionUnit => "Alternate: Radio Connection Unit",
        Recipe::AlternateRadioControlSystem => "Alternate: Radio Control System",
        Recipe::AlternateRecycledPlastic => "Alternate: Recycled Plastic",
        Recipe::AlternateRecycledRubber => "Alternate: Recycled Rubber",
        Recipe::AlternateRigourMotor => "Alternate: Rigour Motor",
        Recipe::AlternateRubberConcrete => "Alternate: Rubber Concrete",
        Recipe::AlternateSiliconCircuitBoard => "Alternate: Silicon Circuit Board",
        Recipe::AlternateSiliconHighSpeedConnector => "Alternate: Silicon High-Speed Connector",
        Recipe::AlternateSloppyAlumina => "Alternate: Sloppy Alumina",
        Recipe::AlternateSolidSteelIngot => "Alternate: Solid Steel Ingot",
        Recipe::AlternateSteamedCopperSheet => "Alternate: Steamed Copper Sheet",
        Recipe::AlternateSteelCanister => "Alternate: Steel Canister",
        Recipe::AlternateSteelCoatedPlate => "Alternate: Steel Coated Plate",
        Recipe::AlternateSteelRod => "Alternate: Steel Rod",
        Recipe::AlternateSteelRotor => "Alternate: Steel Rotor",
        Recipe::AlternateSteelScrew => "Alternate: Steel Screw",
        Recipe::AlternateSteeledFrame => "Alternate: Steeled Frame",
        Recipe::AlternateStitchedIronPlate => "Alternate: Stitched Iron Plate",
        Recipe::AlternateSuperStateComputer => "Alternate: Super-State Computer",
        Recipe::AlternateTurboBlendFuel => "Alternate: Turbo Blend Fuel",
        Recipe::AlternateTurboElectricMotor => "Alternate: Turbo Electric Motor",
        Recipe::AlternateTurboHeavyFuel => "Alternate: Turbo Heavy Fuel",
        Recipe::AlternateTurboPressureMotor => "Alternate: Turbo Pressure Motor",
        Recipe::AlternateUraniumFuelUnit => "Alternate: Uranium Fuel Unit",
        Recipe::AlternateWetConcrete => "Alternate: Wet Concrete",
        Recipe::AluminaSolution => "Alumina Solution",
        Recipe::AluminumCasing => "Aluminum Casing",
        Recipe::AluminumIngot => "Aluminum Ingot",
        Recipe::AluminumScrap => "Aluminum Scrap",
        Recipe::Assembler => "Assembler",
        Recipe::AssemblyDirectorSystem => "Assembly Director System",
        Recipe::AutomatedGate => "Automated Gate",
        Recipe::AutomatedWiring => "Automated Wiring",
        Recipe::BasicWall1m => "Basic Wall 1m",
        Recipe::BasicWall4m => "Basic Wall 4m",
        Recipe::Battery => "Battery",
        Recipe::Beacon => "Beacon",
        Recipe::BeamConnector => "Beam Connector",
        Recipe::BeamConnectorDouble => "Beam Connector Double",
        Recipe::BeamSupport => "Beam Support",
        Recipe::BigConcretePillar => "Big Concrete Pillar",
        Recipe::BigFramePillar => "Big Frame Pillar",
        Recipe::BigMetalPillar => "Big Metal Pillar",
        Recipe::BigPillarSupport => "Big Pillar Support",
        Recipe::BiomassAlienProtein => "Biomass (Alien Protein)",
        Recipe::BiomassBurner => "Biomass Burner",
        Recipe::BiomassLeaves => "Biomass (Leaves)",
        Recipe::BiomassMycelia => "Biomass (Mycelia)",
        Recipe::BiomassWood => "Biomass (Wood)",
        Recipe::BlackPowder => "Black Powder",
        Recipe::BladeRunners => "Blade Runners",
        Recipe::Blender => "Blender",
        Recipe::BlockSignal => "Block Signal",
        Recipe::BlueprintDesigner => "Blueprint Designer",
        Recipe::Cable => "Cable",
        Recipe::CandyCane => "Candy Cane",
        Recipe::CateriumIngot => "Caterium Ingot",
        Recipe::CatwalkCorner => "Catwalk Corner",
        Recipe::CatwalkCrossing => "Catwalk Crossing",
        Recipe::CatwalkRamp => "Catwalk Ramp",
        Recipe::CatwalkStairs => "Catwalk Stairs",
        Recipe::CatwalkStraight => "Catwalk Straight",
        Recipe::CatwalkTCrossing => "Catwalk T-Crossing",
        Recipe::CeilingLight => "Ceiling Light",
        Recipe::CenterDoorWall => "Center Door Wall",
        Recipe::Chainsaw => "Chainsaw",
        Recipe::CircuitBoard => "Circuit Board",
        Recipe::ClusterNobelisk => "Cluster Nobelisk",
        Recipe::CoalGenerator => "Coal Generator",
        Recipe::ColorCartridge => "Color Cartridge",
        Recipe::Computer => "Computer",
        Recipe::Concrete => "Concrete",
        Recipe::Constructor => "Constructor",
        Recipe::ConveyorBeltMk1 => "Conveyor Belt Mk.1",
        Recipe::ConveyorBeltMk2 => "Conveyor Belt Mk.2",
        Recipe::ConveyorBeltMk3 => "Conveyor Belt Mk.3",
        Recipe::ConveyorBeltMk4 => "Conveyor Belt Mk.4",
        Recipe::ConveyorBeltMk5 => "Conveyor Belt Mk.5",
        Recipe::ConveyorCeilingMount => "Conveyor Ceiling Mount",
        Recipe::ConveyorLiftFloorHole => "Conveyor Lift Floor Hole",
        Recipe::ConveyorLiftMk1 => "Conveyor Lift Mk.1",
        Recipe::ConveyorLiftMk2 => "Conveyor Lift Mk.2",
        Recipe::ConveyorLiftMk3 => "Conveyor Lift Mk.3",
        Recipe::ConveyorLiftMk4 => "Conveyor Lift Mk.4",
        Recipe::ConveyorLiftMk5 => "Conveyor Lift Mk.5",
        Recipe::ConveyorMerger => "Conveyor Merger",
        Recipe::ConveyorPole => "Conveyor Pole",
        Recipe::ConveyorSplitter => "Conveyor Splitter",
        Recipe::ConveyorWallMount => "Conveyor Wall Mount",
        Recipe::ConveyorWallX1 => "Conveyor Wall x1",
        Recipe::ConveyorWallX2 => "Conveyor Wall x2",
        Recipe::ConveyorWallX3 => "Conveyor Wall x3",
        Recipe::CoolingSystem => "Cooling System",
        Recipe::CopperIngot => "Copper Ingot",
        Recipe::CopperPowder => "Copper Powder",
        Recipe::CopperSheet => "Copper Sheet",
        Recipe::CraftBench => "Craft Bench",
        Recipe::CrystalOscillator => "Crystal Oscillator",
        Recipe::CyberWagon => "Cyber Wagon",
        Recipe::DisplaySign => "Display Sign",
        Recipe::DoubleRamp2m => "Double Ramp 2m",
        Recipe::DoubleRamp4m => "Double Ramp 4m",
        Recipe::DoubleRamp8m => "Double Ramp 8m",
        Recipe::DoubleWallOutletMk1 => "Double Wall Outlet Mk.1",
        Recipe::DoubleWallOutletMk2 => "Double Wall Outlet Mk.2",
        Recipe::DoubleWallOutletMk3 => "Double Wall Outlet Mk.3",
        Recipe::DownCornerRamp1m => "Down Corner Ramp 1m",
        Recipe::DownCornerRamp2m => "Down Corner Ramp 2m",
        Recipe::DownCornerRamp4m => "Down Corner Ramp 4m",
        Recipe::Drone => "Drone",
        Recipe::DronePort => "Drone Port",
        Recipe::ElectricLocomotive => "Electric Locomotive",
        Recipe::ElectromagneticControlRod => "Electromagnetic Control Rod",
        Recipe::EmptyCanister => "Empty Canister",
        Recipe::EmptyFluidTank => "Empty Fluid Tank",
        Recipe::EmptyPlatform => "Empty Platform",
        Recipe::EmptyPlatformWithCatwalk => "Empty Platform With Catwalk",
        Recipe::EncasedIndustrialBeam => "Encased Industrial Beam",
        Recipe::EncasedPlutoniumCell => "Encased Plutonium Cell",
        Recipe::EncasedUraniumCell => "Encased Uranium Cell",
        Recipe::EquipmentWorkshop => "Equipment Workshop",
        Recipe::Explorer => "Explorer",
        Recipe::ExplosiveRebar => "Explosive Rebar",
        Recipe::FICSMASGiftTree => "FICSMAS Gift Tree",
        Recipe::FICSMASPowerLight => "FICSMAS Power Light",
        Recipe::FICSMASSnowDispenser => "FICSMAS Snow Dispenser",
        Recipe::FICSMASWreath => "FICSMAS Wreath",
        Recipe::Fabric => "Fabric",
        Recipe::FactoryCart => "Factory Cart™",
        Recipe::FloodLightTower => "Flood Light Tower",
        Recipe::FluidBuffer => "Fluid Buffer",
        Recipe::FluidFreightPlatform => "Fluid Freight Platform",
        Recipe::Foundation1m => "Foundation 1m",
        Recipe::Foundation2m => "Foundation 2m",
        Recipe::Foundation4m => "Foundation 4m",
        Recipe::Foundry => "Foundry",
        Recipe::FrameFloor => "Frame Floor",
        Recipe::FrameFoundation => "Frame Foundation",
        Recipe::FrameRamp => "Frame Ramp",
        Recipe::FrameWall => "Frame Wall",
        Recipe::FrameWindow => "Frame Window",
        Recipe::FreightCar => "Freight Car",
        Recipe::FreightPlatform => "Freight Platform",
        Recipe::Fuel => "Fuel",
        Recipe::FuelGenerator => "Fuel Generator",
        Recipe::FullFrameWindow => "Full Frame Window",
        Recipe::FusedModularFrame => "Fused Modular Frame",
        Recipe::GasFilter => "Gas Filter",
        Recipe::GasMask => "Gas Mask",
        Recipe::GasNobelisk => "Gas Nobelisk",
        Recipe::GateHoleWall => "Gate Hole Wall",
        Recipe::GeothermalGenerator => "Geothermal Generator",
        Recipe::GiantFICSMASTree => "Giant FICSMAS Tree",
        Recipe::GlassFrameFoundation => "Glass Frame Foundation",
        Recipe::GoldenFactoryCart => "Golden Factory Cart™",
        Recipe::Half1mFoundation => "Half 1m Foundation",
        Recipe::Half2mFoundation => "Half 2m Foundation",
        Recipe::Half4mFoundation => "Half 4m Foundation",
        Recipe::HatcherProtein => "Hatcher Protein",
        Recipe::HazardStorageBox => "Hazard Storage Box",
        Recipe::HazmatSuit => "Hazmat Suit",
        Recipe::HeatSink => "Heat Sink",
        Recipe::HeavyModularFrame => "Heavy Modular Frame",
        Recipe::HexFrameWindow => "Hex Frame Window",
        Recipe::HighSpeedConnector => "High-Speed Connector",
        Recipe::HogProtein => "Hog Protein",
        Recipe::HomingRifleAmmo => "Homing Rifle Ammo",
        Recipe::HoverPack => "Hover Pack",
        Recipe::Hypertube => "Hypertube",
        Recipe::HypertubeEntrance => "Hypertube Entrance",
        Recipe::HypertubeFloorHole => "Hypertube Floor Hole",
        Recipe::HypertubeSupport => "Hypertube Support",
        Recipe::HypertubeWallHole => "Hypertube Wall Hole",
        Recipe::HypertubeWallSupport => "Hypertube Wall Support",
        Recipe::IndustrialFluidBuffer => "Industrial Fluid Buffer",
        Recipe::IndustrialStorageContainer => "Industrial Storage Container",
        Recipe::InnerCornerExtension1m => "Inner Corner Extension 1m",
        Recipe::InnerCornerExtension2m => "Inner Corner Extension 2m",
        Recipe::InnerCornerExtension4m => "Inner Corner Extension 4m",
        Recipe::InnerCornerQuarterPipe => "Inner Corner Quarter Pipe",
        Recipe::InnerCornerRoof1m => "Inner Corner Roof 1m",
        Recipe::InnerCornerRoof2m => "Inner Corner Roof 2m",
        Recipe::InnerCornerRoof4m => "Inner Corner Roof 4m",
        Recipe::InvDownCorner1m => "Inv. Down Corner 1m",
        Recipe::InvDownCorner2m => "Inv. Down Corner 2m",
        Recipe::InvDownCorner4m => "Inv. Down Corner 4m",
        Recipe::InvRamp1m => "Inv. Ramp 1m",
        Recipe::InvRamp2m => "Inv. Ramp 2m",
        Recipe::InvRamp4m => "Inv. Ramp 4m",
        Recipe::InvRampWall1m => "Inv. Ramp Wall 1m",
        Recipe::InvRampWall2m => "Inv. Ramp Wall 2m",
        Recipe::InvRampWall4m => "Inv. Ramp Wall 4m",
        Recipe::InvRampWall8m => "Inv. Ramp Wall 8m",
        Recipe::InvUpCorner1m => "Inv. Up Corner 1m",
        Recipe::InvUpCorner2m => "Inv. Up Corner 2m",
        Recipe::InvUpCorner4m => "Inv. Up Corner 4m",
        Recipe::InvertedFrameRamp => "Inverted Frame Ramp",
        Recipe::InvertedInnerCornerQuarterPipe => "Inverted Inner Corner Quarter Pipe",
        Recipe::InvertedOuterCornerQuarterPipe => "Inverted Outer Corner Quarter Pipe",
        Recipe::InvertedQuarterPipe => "Inverted Quarter Pipe",
        Recipe::IodineInfusedFilter => "Iodine Infused Filter",
        Recipe::IronIngot => "Iron Ingot",
        Recipe::IronPlate => "Iron Plate",
        Recipe::IronRebar => "Iron Rebar",
        Recipe::IronRod => "Iron Rod",
        Recipe::Jetpack => "Jetpack",
        Recipe::JumpPad => "Jump Pad",
        Recipe::LabelSign2m => "Label Sign 2m",
        Recipe::LabelSign3m => "Label Sign 3m",
        Recipe::LabelSign4m => "Label Sign 4m",
        Recipe::Ladder => "Ladder",
        Recipe::LargeBillboard => "Large Billboard",
        Recipe::LightsControlPanel => "Lights Control Panel",
        Recipe::LiquidBiofuel => "Liquid Biofuel",
        Recipe::LookoutTower => "Lookout Tower",
        Recipe::MAM => "MAM",
        Recipe::MagneticFieldGenerator => "Magnetic Field Generator",
        Recipe::Manufacturer => "Manufacturer",
        Recipe::MedicalStorageBox => "Medical Storage Box",
        Recipe::MetalBeam => "Metal Beam",
        Recipe::MinerMk1 => "Miner Mk.1",
        Recipe::MinerMk2 => "Miner Mk.2",
        Recipe::MinerMk3 => "Miner Mk.3",
        Recipe::ModernRailing => "Modern Railing",
        Recipe::ModularEngine => "Modular Engine",
        Recipe::ModularFrame => "Modular Frame",
        Recipe::Motor => "Motor",
        Recipe::NitricAcid => "Nitric Acid",
        Recipe::Nobelisk => "Nobelisk",
        Recipe::NobeliskDetonator => "Nobelisk Detonator",
        Recipe::NonFissileUranium => "Non-fissile Uranium",
        Recipe::NuclearPasta => "Nuclear Pasta",
        Recipe::NuclearPowerPlant => "Nuclear Power Plant",
        Recipe::NukeNobelisk => "Nuke Nobelisk",
        Recipe::NutritionalInhaler => "Nutritional Inhaler",
        Recipe::ObjectScanner => "Object Scanner",
        Recipe::OilExtractor => "Oil Extractor",
        Recipe::OuterCornerExtension1m => "Outer Corner Extension 1m",
        Recipe::OuterCornerExtension2m => "Outer Corner Extension 2m",
        Recipe::OuterCornerExtension4m => "Outer Corner Extension 4m",
        Recipe::OuterCornerQuarterPipe => "Outer Corner Quarter Pipe",
        Recipe::OuterCornerRoof1m => "Outer Corner Roof 1m",
        Recipe::OuterCornerRoof2m => "Outer Corner Roof 2m",
        Recipe::OuterCornerRoof4m => "Outer Corner Roof 4m",
        Recipe::PackagedAluminaSolution => "Packaged Alumina Solution",
        Recipe::PackagedFuel => "Packaged Fuel",
        Recipe::PackagedHeavyOilResidue => "Packaged Heavy Oil Residue",
        Recipe::PackagedLiquidBiofuel => "Packaged Liquid Biofuel",
        Recipe::PackagedNitricAcid => "Packaged Nitric Acid",
        Recipe::PackagedNitrogenGas => "Packaged Nitrogen Gas",
        Recipe::PackagedOil => "Packaged Oil",
        Recipe::PackagedSulfuricAcid => "Packaged Sulfuric Acid",
        Recipe::PackagedTurbofuel => "Packaged Turbofuel",
        Recipe::PackagedWater => "Packaged Water",
        Recipe::Packager => "Packager",
        Recipe::PaintedBeam => "Painted Beam",
        Recipe::PanelWindow => "Panel Window",
        Recipe::Parachute => "Parachute",
        Recipe::ParticleAccelerator => "Particle Accelerator",
        Recipe::PathSignal => "Path Signal",
        Recipe::PersonalStorageBox => "Personal Storage Box",
        Recipe::PetroleumCoke => "Petroleum Coke",
        Recipe::PipelineFloorHole => "Pipeline Floor Hole",
        Recipe::PipelineJunctionCross => "Pipeline Junction Cross",
        Recipe::PipelineMk1 => "Pipeline Mk.1",
        Recipe::PipelineMk1NoIndicator => "Pipeline Mk.1 (No Indicator)",
        Recipe::PipelineMk2 => "Pipeline Mk.2",
        Recipe::PipelineMk2NoIndicator => "Pipeline Mk.2 (No Indicator)",
        Recipe::PipelinePumpMk1 => "Pipeline Pump Mk.1",
        Recipe::PipelinePumpMk2 => "Pipeline Pump Mk.2",
        Recipe::PipelineSupport => "Pipeline Support",
        Recipe::PipelineWallHole => "Pipeline Wall Hole",
        Recipe::PipelineWallSupport => "Pipeline Wall Support",
        Recipe::Plastic => "Plastic",
        Recipe::PlutoniumFuelRod => "Plutonium Fuel Rod",
        Recipe::PlutoniumPellet => "Plutonium Pellet",
        Recipe::PortableMiner => "Portable Miner",
        Recipe::PortraitSign => "Portrait Sign",
        Recipe::PowerLine => "Power Line",
        Recipe::PowerPoleMk1 => "Power Pole Mk.1",
        Recipe::PowerPoleMk2 => "Power Pole Mk.2",
        Recipe::PowerPoleMk3 => "Power Pole Mk.3",
        Recipe::PowerShard1 => "Power Shard (1)",
        Recipe::PowerShard2 => "Power Shard (2)",
        Recipe::PowerShard5 => "Power Shard (5)",
        Recipe::PowerStorage => "Power Storage",
        Recipe::PowerSwitch => "Power Switch",
        Recipe::PowerTower => "Power Tower",
        Recipe::PowerTowerPlatform => "Power Tower Platform",
        Recipe::PressureConversionCube => "Pressure Conversion Cube",
        Recipe::PriorityPowerSwitch => "Priority Power Switch",
        Recipe::ProgrammableSplitter => "Programmable Splitter",
        Recipe::ProteinInhaler => "Protein Inhaler",
        Recipe::PulseNobelisk => "Pulse Nobelisk",
        Recipe::QuarterPipe => "Quarter Pipe",
        Recipe::QuartzCrystal => "Quartz Crystal",
        Recipe::Quickwire => "Quickwire",
        Recipe::RadarTower => "Radar Tower",
        Recipe::RadioControlUnit => "Radio Control Unit",
        Recipe::Railway => "Railway",
        Recipe::Ramp1m => "Ramp 1m",
        Recipe::Ramp2m => "Ramp 2m",
        Recipe::Ramp4m => "Ramp 4m",
        Recipe::RampWall1m => "Ramp Wall 1m",
        Recipe::RampWall2m => "Ramp Wall 2m",
        Recipe::RampWall4m => "Ramp Wall 4m",
        Recipe::RampWall8m => "Ramp Wall 8m",
        Recipe::RebarGun => "Rebar Gun",
        Recipe::Refinery => "Refinery",
        Recipe::ReinforcedIronPlate => "Reinforced Iron Plate",
        Recipe::ReinforcedWindow => "Reinforced Window",
        Recipe::ResidualFuel => "Residual Fuel",
        Recipe::ResidualPlastic => "Residual Plastic",
        Recipe::ResidualRubber => "Residual Rubber",
        Recipe::ResourceWellExtractor => "Resource Well Extractor",
        Recipe::ResourceWellPressurizer => "Resource Well Pressurizer",
        Recipe::Rifle => "Rifle",
        Recipe::RifleAmmo => "Rifle Ammo",
        Recipe::RoadBarrier => "Road Barrier",
        Recipe::Roof1m => "Roof 1m",
        Recipe::Roof2m => "Roof 2m",
        Recipe::Roof4m => "Roof 4m",
        Recipe::RoofFlat => "Roof Flat",
        Recipe::Rotor => "Rotor",
        Recipe::Rubber => "Rubber",
        Recipe::Screw => "Screw",
        Recipe::ShatterRebar => "Shatter Rebar",
        Recipe::SideDoorWall => "Side Door Wall",
        Recipe::Silica => "Silica",
        Recipe::SingleWindow => "Single Window",
        Recipe::SmallBillboard => "Small Billboard",
        Recipe::SmallConcretePillar => "Small Concrete Pillar",
        Recipe::SmallFramePillar => "Small Frame Pillar",
        Recipe::SmallMetalPillar => "Small Metal Pillar",
        Recipe::SmallPillarSupport => "Small Pillar Support",
        Recipe::SmartPlating => "Smart Plating",
        Recipe::SmartSplitter => "Smart Splitter",
        Recipe::Smelter => "Smelter",
        Recipe::SmokelessPowder => "Smokeless Powder",
        Recipe::Snowman => "Snowman",
        Recipe::SolidBiofuel => "Solid Biofuel",
        Recipe::SpaceElevator => "Space Elevator",
        Recipe::SpitterProtein => "Spitter Protein",
        Recipe::SquareSign05m => "Square Sign 0.5m",
        Recipe::SquareSign1m => "Square Sign 1m",
        Recipe::SquareSign2m => "Square Sign 2m",
        Recipe::StackableConveyorPole => "Stackable Conveyor Pole",
        Recipe::StackableHypertubeSupport => "Stackable Hypertube Support",
        Recipe::StackablePipelineSupport => "Stackable Pipeline Support",
        Recipe::StairsLeft => "Stairs Left",
        Recipe::StairsRight => "Stairs Right",
        Recipe::Stator => "Stator",
        Recipe::SteelBeam => "Steel Beam",
        Recipe::SteelIngot => "Steel Ingot",
        Recipe::SteelPipe => "Steel Pipe",
        Recipe::StingerProtein => "Stinger Protein",
        Recipe::StorageContainer => "Storage Container",
        Recipe::StreetLight => "Street Light",
        Recipe::StunRebar => "Stun Rebar",
        Recipe::SulfuricAcid => "Sulfuric Acid",
        Recipe::Supercomputer => "Supercomputer",
        Recipe::TheHUB => "The HUB",
        Recipe::TherapeuticInhaler => "Therapeutic Inhaler",
        Recipe::ThermalPropulsionRocket => "Thermal Propulsion Rocket",
        Recipe::TiltedConcaveWall4m => "Tilted Concave Wall 4m",
        Recipe::TiltedConcaveWall8m => "Tilted Concave Wall 8m",
        Recipe::TiltedCornerWall4m => "Tilted Corner Wall 4m",
        Recipe::TiltedCornerWall8m => "Tilted Corner Wall 8m",
        Recipe::TiltedWall4m => "Tilted Wall 4m",
        Recipe::TiltedWall8m => "Tilted Wall 8m",
        Recipe::Tractor => "Tractor",
        Recipe::TrainStation => "Train Station",
        Recipe::Truck => "Truck",
        Recipe::TruckStation => "Truck Station",
        Recipe::TurboMotor => "Turbo Motor",
        Recipe::TurboRifleAmmo => "Turbo Rifle Ammo",
        Recipe::Turbofuel => "Turbofuel",
        Recipe::UJellyLandingPad => "U-Jelly Landing Pad",
        Recipe::UnpackageAluminaSolution => "Unpackage Alumina Solution",
        Recipe::UnpackageFuel => "Unpackage Fuel",
        Recipe::UnpackageHeavyOilResidue => "Unpackage Heavy Oil Residue",
        Recipe::UnpackageLiquidBiofuel => "Unpackage Liquid Biofuel",
        Recipe::UnpackageNitricAcid => "Unpackage Nitric Acid",
        Recipe::UnpackageNitrogenGas => "Unpackage Nitrogen Gas",
        Recipe::UnpackageOil => "Unpackage Oil",
        Recipe::UnpackageSulfuricAcid => "Unpackage Sulfuric Acid",
        Recipe::UnpackageTurbofuel => "Unpackage Turbofuel",
        Recipe::UnpackageWater => "Unpackage Water",
        Recipe::UpCornerRamp1m => "Up Corner Ramp 1m",
        Recipe::UpCornerRamp2m => "Up Corner Ramp 2m",
        Recipe::UpCornerRamp4m => "Up Corner Ramp 4m",
        Recipe::UraniumFuelRod => "Uranium Fuel Rod",
        Recipe::Valve => "Valve",
        Recipe::VersatileFramework => "Versatile Framework",
        Recipe::VitaminInhaler => "Vitamin Inhaler",
        Recipe::Wall1a => "Wall 1a",
        Recipe::WallMountedFloodLight => "Wall Mounted Flood Light",
        Recipe::WallOutletMk1 => "Wall Outlet Mk.1",
        Recipe::WallOutletMk2 => "Wall Outlet Mk.2",
        Recipe::WallOutletMk3 => "Wall Outlet Mk.3",
        Recipe::WaterExtractor => "Water Extractor",
        Recipe::Wire => "Wire",
        Recipe::XenoBasher => "Xeno-Basher",
        Recipe::XenoZapper => "Xeno-Zapper",
        Recipe::Zipline => "Zipline",
    }
}
