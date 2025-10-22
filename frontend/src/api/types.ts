// Satisflow API Type Definitions
// This file contains TypeScript types that match the backend API contract

// ============================================================================
// Core Engine Types
// ============================================================================

// Item type - string union of all Satisfactory items
export type Item =
  | "IronOre"
  | "CopperOre"
  | "IronIngot"
  | "CopperIngot"
  | "IronPlate"
  | "CopperSheet"
  | "Wire"
  | "Cable"
  | "Concrete"
  | "Coal"
  | "Biomass"
  | "Fuel"
  | "Turbofuel"
  | "CrudeOil"
  | "Water"
  | "NitrogenGas"
  | "UraniumFuelRod"
  | "UraniumWaste"
  | "Limestone"
  | "RawQuartz"
  | "QuartzCrystal"
  | "CateriumOre"
  | "CateriumIngot"
  | "Sulfur"
  | "AluminumScrap"
  | "AluminumIngot"
  | "AluminumCasing"
  | "Silica"
  | "Plastic"
  | "Rubber"
  | "Fuel"
  | "Turbofuel"
  | "NuclearWaste"
  | "PlutoniumPellet"
  | "PlutoniumFuelRod"
  | "NonFissileUranium"
  | "Uranium"
  | "UraniumFuelRod"
  | "EncasedUraniumCell"
  | "EncasedPlutoniumCell"
  | "Ficsonium"
  | "FicsoniumFuelRod"
  | "EmptyFluidTank"
  | "PackagedWater"
  | "PackagedOil"
  | "PackagedFuel"
  | "PackagedTurbofuel"
  | "HeavyOilResidue"
  | "LiquidBiofuel"
  | "NitricAcid"
  | "EmptyCanister"
  | "CompactedCoal"
  | "PetroleumCoke"
  | "NuclearPasta"
  | "ColorCartridge"
  | "PowerShard"
  | "PowerCrystal"
  | "FicsiteIngot"
  | "FicsiteTrigon"
  | "DarkMatterCrystal"
  | "Biomass"
  | "Leaves"
  | "Mycelia"
  | "Wood"
  | "AlienProtein"
  | "AlienDNA"
  | "FlowerPetals"
  | "BaconAgaric"
  | "BerylNut"
  | "Pollen"
  | "SmartPlating"
  | "ModularFrame"
  | "ReinforcedIronPlate"
  | "Rotor"
  | "Stator"
  | "Motor"
  | "Screw"
  | "Screws"
  | "IronRod"
  | "SteelPipe"
  | "SteelBeam"
  | "EncasedIndustrialBeam"
  | "CircuitBoard"
  | "Computer"
  | "AdaptiveControlUnit"
  | "ElectromagneticControlRod"
  | "HighSpeedConnector"
  | "AutomatedWiring"
  | "Supercomputer"
  | "AIExpansionServer"
  | "HeatSink"
  | "CoolingSystem"
  | "Battery"
  | "ModularEngine"
  | "TurboMotor"
  | "FusedModularFrame"
  | "HeavyModularFrame"
  | "PressureConversionCube"
  | "AssemblyDirectorSystem"
  | "CrystalOscillator"
  | "MagneticFieldGenerator"
  | "RadioControlUnit"
  | "BallisticWarpDrive"
  | "SpaceElevatorModular"
  | "ProjectAssembly"
  | "Turbofuel"
  | "RocketFuel"
  | "NuclearFuelRod"
  | "IonizedFuel"
  | "Beacon"
  | "ExplosiveRebar"
  | "Nobelisk"
  | "NobeliskDetonator"
  | "GasFilter"
  | "GasMask"
  | "IodineInfusedFilter"
  | "HazmatSuit"
  | "Jetpack"
  | "Hoverpack"
  | "BladeRunners"
  | "XenoZapper"
  | "RebarGun"
  | "ObjectScanner"
  | "Zapper"
  | "StunSpear"
  | "Rifle"
  | "RifleAmmo"
  | "SpitterRemains"
  | "SpitterGland"
  | "HogRemains"
  | "HogGland"
  | "StingerRemains"
  | "StingerGland"
  | "AlienOrgans"
  | "Cartridge"
  | "SolidBiofuel"
  | "LiquidBiofuel"
  | "PackagedLiquidBiofuel"
  | "CateriumIngot"
  | "Quickwire"
  | "FICSCoupon"
  | "TimeCrystal"
  | "Somersloop"
  | "MAMCircuitBoard"
  | "MAMCircuitFabricator"
  | "MAMDataHub"
  | "MAMResearchCore"
  | "MAMSchematic"
  | "MAMSchematic1"
  | "MAMSchematic2"
  | "MAMSchematic3"
  | "MAMSchematic4"
  | "MAMSchematic5"
  | "MAMSchematic6"
  | "MAMSchematic7"
  | "MAMSchematic8"
  | "MAMSchematic9"
  | "MAMSchematic10"
  | "MAMSchematic11"
  | "MAMSchematic12"
  | "MAMSchematic13"
  | "MAMSchematic14"
  | "MAMSchematic15"
  | "MAMSchematic16"
  | "MAMSchematic17"
  | "MAMSchematic18"
  | "MAMSchematic19"
  | "MAMSchematic20"
  | "MAMSchematic21"
  | "MAMSchematic22"
  | "MAMSchematic23"
  | "MAMSchematic24"
  | "MAMSchematic25"
  | "MAMSchematic26"
  | "MAMSchematic27"
  | "MAMSchematic28"
  | "MAMSchematic29"
  | "MAMSchematic30"
  | "MAMSchematic31"
  | "MAMSchematic32"
  | "MAMSchematic33"
  | "MAMSchematic34"
  | "MAMSchematic35"
  | "MAMSchematic36"
  | "MAMSchematic37"
  | "MAMSchematic38"
  | "MAMSchematic39"
  | "MAMSchematic40"
  | "MAMSchematic41"
  | "MAMSchematic42"
  | "MAMSchematic43"
  | "MAMSchematic44"
  | "MAMSchematic45"
  | "MAMSchematic46"
  | "MAMSchematic47"
  | "MAMSchematic48"
  | "MAMSchematic49"
  | "MAMSchematic50";

// Purity type for resource extraction
export type Purity = "Impure" | "Normal" | "Pure";

// Extractor types
export type ExtractorType =
  | "MinerMk1"
  | "MinerMk2"
  | "MinerMk3"
  | "WaterExtractor"
  | "OilExtractor"
  | "ResourceWellExtractor";

// Generator types
export type GeneratorType =
  | "Biomass"
  | "Coal"
  | "Fuel"
  | "Nuclear"
  | "Geothermal";

// Machine types
export type MachineType =
  | "Constructor"
  | "Smelter"
  | "Assembler"
  | "Manufacturer"
  | "Refinery"
  | "Blender"
  | "Foundry"
  | "Packager"
  | "Converter"
  | "OilRefinery"
  | "WaterPump"
  | "PowerShard"
  | "ResourceWellPressurizer"
  | "OilPump"
  | "FrackingExtractor"
  | "FrackingActivator"
  | "QuantumEncoder"
  | "HadronCollider"
  | "AccomplishmentShredder"
  | "AWESOME Sink";

// Transport types
export type TransportType = "Bus" | "Train" | "Truck" | "Drone";

// Logistics transport tiers
export type ConveyorTier = "Mk1" | "Mk2" | "Mk3" | "Mk4" | "Mk5" | "Mk6";
export type PipelineTier = "Mk1" | "Mk2";
export type WagonCarType = "Cargo" | "Fluid";

// Logistics transport payloads
export interface BusConveyorPayload {
  line_id?: string;
  conveyor_type: ConveyorTier;
  item: Item;
  quantity_per_min: number;
}

export interface BusPipelinePayload {
  pipeline_id?: string;
  pipeline_type: PipelineTier;
  item: Item;
  quantity_per_min: number;
}

export interface TrainWagonPayload {
  wagon_id?: string;
  wagon_type: WagonCarType;
  item: Item;
  quantity_per_min: number;
}

// Item balance states
export type ItemBalanceState = "overflow" | "underflow" | "balanced";

// ============================================================================
// Core Data Structures
// ============================================================================

// Item quantity pair
export interface ItemQuantity {
  item: Item;
  quantity: number;
}

// Item flow for logistics
export interface ItemFlow {
  item: Item;
  quantity_per_min: number;
}

// Machine group configuration
export interface MachineGroup {
  number_of_machine: number;
  oc_value: number;
  somersloop: number;
}

// Generator group configuration
export interface GeneratorGroup {
  number_of_generators: number;
  clock_speed: number;
}

// ============================================================================
// Response Types
// ============================================================================

// Factory response
export interface FactoryResponse {
  id: number;
  name: string;
  description: string | null;
  notes: string | null;
  production_lines: ProductionLineResponse[];
  raw_inputs: RawInputResponse[];
  power_generators: PowerGeneratorResponse[];
  items: ItemQuantity[];
  total_power_consumption: number;
  total_power_generation: number;
  power_balance: number;
}

// Production line response (tagged union)
export type ProductionLineResponse =
  | {
      ProductionLineRecipe: {
        id: number;
        name: string;
        description: string;
        recipe: string;
        machine_groups: MachineGroup[];
      };
    }
  | {
      ProductionLineBlueprint: {
        id: number;
        name: string;
        description: string;
        production_lines: ProductionLineResponse[];
      };
    };

// Raw input response
export interface RawInputResponse {
  id: number;
  extractor_type: ExtractorType;
  item: Item;
  purity: Purity | null;
  quantity_per_min: number;
  pressurizer: ResourceWellPressurizer | null;
  extractors: ResourceWellExtractor[];
}

// Resource well pressurizer
export interface ResourceWellPressurizer {
  id: number;
  clock_speed: number;
  power_consumption: number;
}

// Resource well extractor
export interface ResourceWellExtractor {
  id: number;
  item: Item;
  purity: Purity;
  quantity_per_min: number;
}

// Power generator response
export interface PowerGeneratorResponse {
  id: number;
  generator_type: GeneratorType;
  fuel_type: Item | null;
  groups: GeneratorGroup[];
}

// Logistics response
export interface LogisticsResponse {
  id: number;
  from_factory: number;
  to_factory: number;
  transport_type: TransportType;
  transport_id: string;
  transport_name: string | null;
  transport_details: string;
  items: ItemFlow[];
  total_quantity_per_min: number;
}

// Dashboard summary response
export interface DashboardSummary {
  total_factories: number;
  total_production_lines: number;
  total_logistics_lines: number;
  total_power_consumption: number;
  total_power_generation: number;
  net_power: number;
}

// Item balance response
export interface ItemBalance {
  item: Item;
  balance: number;
  state: ItemBalanceState;
}

// Power statistics response
export interface PowerStats {
  total_generation: number;
  total_consumption: number;
  power_balance: number;
  has_surplus: boolean;
  has_deficit: boolean;
  is_balanced: boolean;
  factory_stats: FactoryPowerStats[];
}

// Factory power statistics
export interface FactoryPowerStats {
  factory_id: number;
  factory_name: string;
  generation: number;
  consumption: number;
  balance: number;
  generator_count: number;
  generator_types: string[];
}

// Recipe info response
export interface RecipeInfo {
  name: string;
  machine: string;
  inputs: ItemQuantity[];
  outputs: ItemQuantity[];
}

// Machine info response
export interface MachineInfo {
  name: string;
  base_power: number;
  max_sommersloop: number;
}

// Item info response
export type ItemInfo = Item;

// Health check response
export interface HealthCheckResponse {
  status: string;
  timestamp: string;
  service: string;
}

// ============================================================================
// Request Types
// ============================================================================

// Create factory request
export interface CreateFactoryRequest {
  name: string;
  description?: string;
  notes?: string;
}

// Update factory request
export interface UpdateFactoryRequest {
  name?: string;
  description?: string;
  notes?: string;
}

// Create logistics request
export type CreateLogisticsRequest =
  | {
      from_factory: number;
      to_factory: number;
      transport_type: "Truck";
      item: Item;
      quantity_per_min: number;
      truck_id?: string;
    }
  | {
      from_factory: number;
      to_factory: number;
      transport_type: "Drone";
      item: Item;
      quantity_per_min: number;
      drone_id?: string;
    }
  | {
      from_factory: number;
      to_factory: number;
      transport_type: "Bus";
      bus_name?: string;
      conveyors: BusConveyorPayload[];
      pipelines: BusPipelinePayload[];
    }
  | {
      from_factory: number;
      to_factory: number;
      transport_type: "Train";
      train_name?: string;
      wagons: TrainWagonPayload[];
    };

// Create production line request
export interface CreateProductionLineRequest {
  factory_id: number;
  name: string;
  description?: string;
  type: "recipe" | "blueprint";
  recipe?: string;
  machine_groups?: MachineGroup[];
  production_lines?: ProductionLineResponse[];
}

// Create raw input request
export interface CreateRawInputRequest {
  factory_id: number;
  extractor_type: ExtractorType;
  item: Item;
  purity?: Purity;
  quantity_per_min: number;
  pressurizer?: {
    clock_speed: number;
  };
  extractors?: Array<{
    item: Item;
    purity: Purity;
    quantity_per_min: number;
  }>;
}

// Create power generator request
export interface CreatePowerGeneratorRequest {
  factory_id: number;
  generator_type: GeneratorType;
  fuel_type?: Item;
  groups: GeneratorGroup[];
}

// ============================================================================
// Error Response Type
// ============================================================================

export interface ErrorResponse {
  error: string;
  status: number;
}

// ============================================================================
// API Response Wrapper Types
// ============================================================================

// Common API response wrapper
export interface ApiResponse<T> {
  data?: T;
  error?: ErrorResponse;
}

// Paginated response (for future use)
export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  per_page: number;
}

// ============================================================================
// Utility Types
// ============================================================================

// ID type for better type safety
export type ID = number;

// Timestamp type
export type Timestamp = string;

// Optional fields helper
export type Optional<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;

// Required fields helper
export type WithRequired<T, K extends keyof T> = T & Required<Pick<T, K>>;
