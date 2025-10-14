use std::{fmt, str::FromStr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemParseError {
    invalid_name: String,
}

impl ItemParseError {
    pub fn new(name: &str) -> Self {
        Self {
            invalid_name: name.to_string(),
        }
    }

    pub fn invalid_name(&self) -> &str {
        &self.invalid_name
    }
}

impl fmt::Display for ItemParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown item name: {}", self.invalid_name)
    }
}

impl std::error::Error for ItemParseError {}

include!("items_data.inc");

macro_rules! for_each_item_name {
    ($macro:ident) => {
        item_name_data!($macro);
    };
}

macro_rules! generate_item_name_pairs {
    ($($variant:ident => $name:expr),+ $(,)?) => {
        pub const ITEM_NAME_PAIRS: &[(Item, &str)] = &[
            $( (Item::$variant, $name), )+
        ];
    };
}

macro_rules! generate_item_name_fn {
    ($($variant:ident => $name:expr),+ $(,)?) => {
        pub fn item_name(item: Item) -> &'static str {
            match item {
                $( Item::$variant => $name, )+
            }
        }
    };
}

macro_rules! generate_item_by_name_fn {
    ($($variant:ident => $name:expr),+ $(,)?) => {
        pub fn item_by_name(name: &str) -> Option<Item> {
            match name {
                $( $name => Some(Item::$variant), )+
                _ => None,
            }
        }
    };
}

for_each_item_name!(generate_item_name_pairs);
for_each_item_name!(generate_item_name_fn);
for_each_item_name!(generate_item_by_name_fn);

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(item_name(*self))
    }
}

impl TryFrom<&str> for Item {
    type Error = ItemParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        item_by_name(value).ok_or_else(|| ItemParseError::new(value))
    }
}

impl TryFrom<String> for Item {
    type Error = ItemParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Item::try_from(value.as_str())
    }
}

impl FromStr for Item {
    type Err = ItemParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Item::try_from(s)
    }
}

pub fn all_items() -> &'static [(Item, &'static str)] {
    ITEM_NAME_PAIRS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_matches_item_name() {
        let item = Item::Computer;
        assert_eq!(item.to_string(), item_name(item));
    }

    #[test]
    fn try_from_known_name_succeeds() {
        let item = Item::try_from("Circuit Board").expect("known item");
        assert_eq!(item, Item::CircuitBoard);
    }

    #[test]
    fn try_from_unknown_name_produces_error() {
        let err = Item::try_from("Not A Real Item").expect_err("invalid item should fail");
        assert_eq!(err.invalid_name(), "Not A Real Item");
    }
}
