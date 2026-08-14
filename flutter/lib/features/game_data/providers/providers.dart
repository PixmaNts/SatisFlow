import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../core/errors/failures.dart';

/// Cache TTL in milliseconds (5 minutes)
const int _cacheTtlMs = 5 * 60 * 1000;

/// Data class representing an Item in the game.
class ItemInfo {
  /// The item identifier (matching Rust Item enum variant name)
  final String id;

  /// The display name of the item
  final String name;

  const ItemInfo({required this.id, required this.name});

  @override
  String toString() => 'ItemInfo(id: \$id, name: \$name)';
}

/// Data class representing a Machine type in the game.
class MachineInfo {
  /// The machine type identifier
  final String name;

  /// Base power consumption in MW at 100% clock speed
  final double basePowerMw;

  /// Maximum number of Somersloops this machine can use
  final int maxSomersloop;

  const MachineInfo({
    required this.name,
    required this.basePowerMw,
    required this.maxSomersloop,
  });

  @override
  String toString() =>
      'MachineInfo(name: \$name, basePowerMw: \$basePowerMw, maxSomersloop: \$maxSomersloop)';
}

/// Data class representing a Recipe input/output pair.
class RecipeItem {
  /// The item name
  final String item;

  /// The quantity per cycle
  final double quantity;

  const RecipeItem({required this.item, required this.quantity});

  @override
  String toString() => 'RecipeItem(item: \$item, quantity: \$quantity)';
}

/// Data class representing a Recipe in the game.
class RecipeInfo {
  /// The recipe identifier (matching Rust Recipe enum variant name)
  final String id;

  /// The display name of the recipe
  final String name;

  /// The machine type used for this recipe
  final String machine;

  /// Input items with quantities
  final List<RecipeItem> inputs;

  /// Output items with quantities
  final List<RecipeItem> outputs;

  const RecipeInfo({
    required this.id,
    required this.name,
    required this.machine,
    required this.inputs,
    required this.outputs,
  });

  @override
  String toString() => 'RecipeInfo(id: \$id, name: \$name, machine: \$machine)';
}

/// Static game data from the Rust engine.
class GameData {
  // All items from the Rust Item enum
  static final List<ItemInfo> _items = [
    const ItemInfo(id: 'AILimiter', name: 'AI Limiter'),
    const ItemInfo(id: 'AdaptiveControlUnit', name: 'Adaptive Control Unit'),
    const ItemInfo(id: 'AIExpansionServer', name: 'AI Expansion Server'),
    const ItemInfo(id: 'AlcladAluminumSheet', name: 'Alclad Aluminum Sheet'),
    const ItemInfo(id: 'AlienDNACapsule', name: 'Alien DNA Capsule'),
    const ItemInfo(id: 'AlienPowerMatrix', name: 'Alien Power Matrix'),
    const ItemInfo(id: 'AlienProtein', name: 'Alien Protein'),
    const ItemInfo(id: 'AluminaSolution', name: 'Alumina Solution'),
    const ItemInfo(id: 'AluminumCasing', name: 'Aluminum Casing'),
    const ItemInfo(id: 'AluminumIngot', name: 'Aluminum Ingot'),
    const ItemInfo(id: 'AluminumScrap', name: 'Aluminum Scrap'),
    const ItemInfo(
      id: 'AssemblyDirectorSystem',
      name: 'Assembly Director System',
    ),
    const ItemInfo(id: 'AutomatedWiring', name: 'Automated Wiring'),
    const ItemInfo(id: 'BaconAgaric', name: 'Bacon Agaric'),
    const ItemInfo(id: 'BallisticWarpDrive', name: 'Ballistic Warp Drive'),
    const ItemInfo(id: 'Battery', name: 'Battery'),
    const ItemInfo(id: 'Bauxite', name: 'Bauxite'),
    const ItemInfo(id: 'Beacon', name: 'Beacon'),
    const ItemInfo(id: 'BerylNut', name: 'Beryl Nut'),
    const ItemInfo(id: 'BiochemicalSculptor', name: 'Biochemical Sculptor'),
    const ItemInfo(id: 'Biomass', name: 'Biomass'),
    const ItemInfo(id: 'BlackPowder', name: 'Black Powder'),
    const ItemInfo(id: 'BladeRunners', name: 'Blade Runners'),
    const ItemInfo(id: 'BluePowerSlug', name: 'Blue Power Slug'),
    const ItemInfo(id: 'BoomBox', name: 'Boom Box'),
    const ItemInfo(id: 'Cable', name: 'Cable'),
    const ItemInfo(id: 'CateriumIngot', name: 'Caterium Ingot'),
    const ItemInfo(id: 'CateriumOre', name: 'Caterium Ore'),
    const ItemInfo(id: 'Chainsaw', name: 'Chainsaw'),
    const ItemInfo(id: 'CircuitBoard', name: 'Circuit Board'),
    const ItemInfo(id: 'ClusterNobelisk', name: 'Cluster Nobelisk'),
    const ItemInfo(id: 'Coal', name: 'Coal'),
    const ItemInfo(id: 'ColorCartridge', name: 'Color Cartridge'),
    const ItemInfo(id: 'CompactedCoal', name: 'Compacted Coal'),
    const ItemInfo(id: 'Computer', name: 'Computer'),
    const ItemInfo(id: 'Concrete', name: 'Concrete'),
    const ItemInfo(id: 'CoolingSystem', name: 'Cooling System'),
    const ItemInfo(id: 'CopperIngot', name: 'Copper Ingot'),
    const ItemInfo(id: 'CopperOre', name: 'Copper Ore'),
    const ItemInfo(id: 'CopperPowder', name: 'Copper Powder'),
    const ItemInfo(id: 'CopperSheet', name: 'Copper Sheet'),
    const ItemInfo(id: 'CrudeOil', name: 'Crude Oil'),
    const ItemInfo(id: 'CrystalOscillator', name: 'Crystal Oscillator'),
    const ItemInfo(id: 'DarkMatterCrystal', name: 'Dark Matter Crystal'),
    const ItemInfo(id: 'Diamonds', name: 'Diamonds'),
    const ItemInfo(
      id: 'ElectromagneticControlRod',
      name: 'Electromagnetic Control Rod',
    ),
    const ItemInfo(id: 'EmptyCanister', name: 'Empty Canister'),
    const ItemInfo(id: 'EmptyFluidTank', name: 'Empty Fluid Tank'),
    const ItemInfo(
      id: 'EncasedIndustrialBeam',
      name: 'Encased Industrial Beam',
    ),
    const ItemInfo(id: 'EncasedPlutoniumCell', name: 'Encased Plutonium Cell'),
    const ItemInfo(id: 'EncasedUraniumCell', name: 'Encased Uranium Cell'),
    const ItemInfo(id: 'ExplosiveRebar', name: 'Explosive Rebar'),
    const ItemInfo(id: 'Fabric', name: 'Fabric'),
    const ItemInfo(id: 'FicsiteIngot', name: 'Ficsite Ingot'),
    const ItemInfo(id: 'FicsiteTrigon', name: 'Ficsite Trigon'),
    const ItemInfo(id: 'Ficsonium', name: 'Ficsonium'),
    const ItemInfo(id: 'FicsoniumFuelRod', name: 'Ficsonium Fuel Rod'),
    const ItemInfo(id: 'FactoryCart', name: 'Factory Cart'),
    const ItemInfo(id: 'FlowerPetals', name: 'Flower Petals'),
    const ItemInfo(id: 'Fuel', name: 'Fuel'),
    const ItemInfo(id: 'FusedModularFrame', name: 'Fused Modular Frame'),
    const ItemInfo(id: 'GasFilter', name: 'Gas Filter'),
    const ItemInfo(id: 'GasMask', name: 'Gas Mask'),
    const ItemInfo(id: 'GasNobelisk', name: 'Gas Nobelisk'),
    const ItemInfo(id: 'GoldenFactoryCart', name: 'Golden Factory Cart'),
    const ItemInfo(id: 'HUBParts', name: 'HUB Parts'),
    const ItemInfo(id: 'HatcherRemains', name: 'Hatcher Remains'),
    const ItemInfo(id: 'HazmatSuit', name: 'Hazmat Suit'),
    const ItemInfo(id: 'HeatSink', name: 'Heat Sink'),
    const ItemInfo(id: 'HeavyModularFrame', name: 'Heavy Modular Frame'),
    const ItemInfo(id: 'HeavyOilResidue', name: 'Heavy Oil Residue'),
    const ItemInfo(id: 'HighSpeedConnector', name: 'High-Speed Connector'),
    const ItemInfo(id: 'HogRemains', name: 'Hog Remains'),
    const ItemInfo(id: 'HomingRifleAmmo', name: 'Homing Rifle Ammo'),
    const ItemInfo(id: 'HoverPack', name: 'Hover Pack'),
    const ItemInfo(id: 'IodineInfusedFilter', name: 'Iodine-Infused Filter'),
    const ItemInfo(id: 'IronIngot', name: 'Iron Ingot'),
    const ItemInfo(id: 'IronOre', name: 'Iron Ore'),
    const ItemInfo(id: 'IronPlate', name: 'Iron Plate'),
    const ItemInfo(id: 'IronRebar', name: 'Iron Rebar'),
    const ItemInfo(id: 'IronRod', name: 'Iron Rod'),
    const ItemInfo(id: 'Jetpack', name: 'Jetpack'),
    const ItemInfo(id: 'Leaves', name: 'Leaves'),
    const ItemInfo(id: 'Limestone', name: 'Limestone'),
    const ItemInfo(id: 'LiquidBiofuel', name: 'Liquid Biofuel'),
    const ItemInfo(
      id: 'MagneticFieldGenerator',
      name: 'Magnetic Field Generator',
    ),
    const ItemInfo(id: 'MedicinalInhaler', name: 'Medicinal Inhaler'),
    const ItemInfo(id: 'MercerSphere', name: 'Mercer Sphere'),
    const ItemInfo(id: 'ModularEngine', name: 'Modular Engine'),
    const ItemInfo(id: 'ModularFrame', name: 'Modular Frame'),
    const ItemInfo(id: 'Motor', name: 'Motor'),
    const ItemInfo(id: 'Mycelia', name: 'Mycelia'),
    const ItemInfo(id: 'NitricAcid', name: 'Nitric Acid'),
    const ItemInfo(id: 'NitrogenGas', name: 'Nitrogen Gas'),
    const ItemInfo(id: 'Nobelisk', name: 'Nobelisk'),
    const ItemInfo(id: 'NobeliskDetonator', name: 'Nobelisk Detonator'),
    const ItemInfo(id: 'NonFissileUranium', name: 'Non-Fissile Uranium'),
    const ItemInfo(id: 'NuclearPasta', name: 'Nuclear Pasta'),
    const ItemInfo(id: 'NukeNobelisk', name: 'Nuke Nobelisk'),
    const ItemInfo(id: 'ObjectScanner', name: 'Object Scanner'),
    const ItemInfo(
      id: 'PackagedAluminaSolution',
      name: 'Packaged Alumina Solution',
    ),
    const ItemInfo(id: 'PackagedFuel', name: 'Packaged Fuel'),
    const ItemInfo(
      id: 'PackagedHeavyOilResidue',
      name: 'Packaged Heavy Oil Residue',
    ),
    const ItemInfo(
      id: 'PackagedLiquidBiofuel',
      name: 'Packaged Liquid Biofuel',
    ),
    const ItemInfo(id: 'PackagedNitricAcid', name: 'Packaged Nitric Acid'),
    const ItemInfo(id: 'PackagedIonizedFuel', name: 'Packaged Ionized Fuel'),
    const ItemInfo(id: 'PackagedNitrogenGas', name: 'Packaged Nitrogen Gas'),
    const ItemInfo(id: 'PackagedOil', name: 'Packaged Oil'),
    const ItemInfo(id: 'PackagedRocketFuel', name: 'Packaged Rocket Fuel'),
    const ItemInfo(id: 'PackagedSulfuricAcid', name: 'Packaged Sulfuric Acid'),
    const ItemInfo(id: 'PackagedTurbofuel', name: 'Packaged Turbofuel'),
    const ItemInfo(id: 'PackagedWater', name: 'Packaged Water'),
    const ItemInfo(id: 'Paleberry', name: 'Paleberry'),
    const ItemInfo(id: 'Parachute', name: 'Parachute'),
    const ItemInfo(id: 'PetroleumCoke', name: 'Petroleum Coke'),
    const ItemInfo(id: 'PlasmaSpitterRemains', name: 'Plasma Spitter Remains'),
    const ItemInfo(id: 'Plastic', name: 'Plastic'),
    const ItemInfo(id: 'PlutoniumFuelRod', name: 'Plutonium Fuel Rod'),
    const ItemInfo(id: 'PlutoniumPellet', name: 'Plutonium Pellet'),
    const ItemInfo(id: 'PlutoniumWaste', name: 'Plutonium Waste'),
    const ItemInfo(id: 'PolymerResin', name: 'Polymer Resin'),
    const ItemInfo(id: 'PortableMiner', name: 'Portable Miner'),
    const ItemInfo(id: 'PowerShard', name: 'Power Shard'),
    const ItemInfo(
      id: 'PressureConversionCube',
      name: 'Pressure Conversion Cube',
    ),
    const ItemInfo(id: 'PulseNobelisk', name: 'Pulse Nobelisk'),
    const ItemInfo(id: 'PurplePowerSlug', name: 'Purple Power Slug'),
    const ItemInfo(id: 'QuartzCrystal', name: 'Quartz Crystal'),
    const ItemInfo(id: 'Quickwire', name: 'Quickwire'),
    const ItemInfo(id: 'RadioControlUnit', name: 'Radio Control Unit'),
    const ItemInfo(id: 'RawQuartz', name: 'Raw Quartz'),
    const ItemInfo(id: 'RebarGun', name: 'Rebar Gun'),
    const ItemInfo(id: 'ReinforcedIronPlate', name: 'Reinforced Iron Plate'),
    const ItemInfo(id: 'Rifle', name: 'Rifle'),
    const ItemInfo(id: 'RifleAmmo', name: 'Rifle Ammo'),
    const ItemInfo(id: 'Rotor', name: 'Rotor'),
    const ItemInfo(id: 'Rubber', name: 'Rubber'),
    const ItemInfo(id: 'Sam', name: 'SAM'),
    const ItemInfo(id: 'Screw', name: 'Screw'),
    const ItemInfo(id: 'ShatterRebar', name: 'Shatter Rebar'),
    const ItemInfo(id: 'Silica', name: 'Silica'),
    const ItemInfo(id: 'SingularityCell', name: 'Singularity Cell'),
    const ItemInfo(id: 'SmartPlating', name: 'Smart Plating'),
    const ItemInfo(id: 'SmokelessPowder', name: 'Smokeless Powder'),
    const ItemInfo(id: 'Somersloop', name: 'Somersloop'),
    const ItemInfo(id: 'SolidBiofuel', name: 'Solid Biofuel'),
    const ItemInfo(id: 'Stator', name: 'Stator'),
    const ItemInfo(id: 'SteelBeam', name: 'Steel Beam'),
    const ItemInfo(id: 'SteelIngot', name: 'Steel Ingot'),
    const ItemInfo(id: 'SteelPipe', name: 'Steel Pipe'),
    const ItemInfo(id: 'StingerRemains', name: 'Stinger Remains'),
    const ItemInfo(id: 'StunRebar', name: 'Stun Rebar'),
    const ItemInfo(id: 'Sulfur', name: 'Sulfur'),
    const ItemInfo(id: 'SulfuricAcid', name: 'Sulfuric Acid'),
    const ItemInfo(id: 'Supercomputer', name: 'Supercomputer'),
    const ItemInfo(
      id: 'ThermalPropulsionRocket',
      name: 'Thermal Propulsion Rocket',
    ),
    const ItemInfo(id: 'TimeCrystal', name: 'Time Crystal'),
    const ItemInfo(id: 'TurboMotor', name: 'Turbo Motor'),
    const ItemInfo(id: 'TurboRifleAmmo', name: 'Turbo Rifle Ammo'),
    const ItemInfo(id: 'Turbofuel', name: 'Turbofuel'),
    const ItemInfo(id: 'Uranium', name: 'Uranium'),
    const ItemInfo(id: 'UraniumFuelRod', name: 'Uranium Fuel Rod'),
    const ItemInfo(id: 'UraniumWaste', name: 'Uranium Waste'),
    const ItemInfo(id: 'VersatileFramework', name: 'Versatile Framework'),
    const ItemInfo(id: 'Vines', name: 'Vines'),
    const ItemInfo(id: 'Water', name: 'Water'),
    const ItemInfo(id: 'Wire', name: 'Wire'),
    const ItemInfo(id: 'Wood', name: 'Wood'),
    const ItemInfo(id: 'XenoBasher', name: 'Xeno-Basher'),
    const ItemInfo(id: 'XenoZapper', name: 'Xeno-Zapper'),
    const ItemInfo(id: 'YellowPowerSlug', name: 'Yellow Power Slug'),
    const ItemInfo(id: 'Zipline', name: 'Zipline'),
  ];

  // All machine types from the Rust MachineType enum
  static final List<MachineInfo> _machines = [
    const MachineInfo(name: 'Constructor', basePowerMw: 4.0, maxSomersloop: 1),
    const MachineInfo(name: 'Assembler', basePowerMw: 16.0, maxSomersloop: 2),
    const MachineInfo(
      name: 'Manufacturer',
      basePowerMw: 32.0,
      maxSomersloop: 4,
    ),
    const MachineInfo(name: 'Smelter', basePowerMw: 4.0, maxSomersloop: 1),
    const MachineInfo(name: 'Foundry', basePowerMw: 16.0, maxSomersloop: 2),
    const MachineInfo(name: 'Refinery', basePowerMw: 16.0, maxSomersloop: 2),
    const MachineInfo(name: 'Blender', basePowerMw: 32.0, maxSomersloop: 4),
    const MachineInfo(name: 'Packager', basePowerMw: 4.0, maxSomersloop: 1),
    const MachineInfo(
      name: 'ParticleAccelerator',
      basePowerMw: 64.0,
      maxSomersloop: 4,
    ),
    const MachineInfo(
      name: 'QuantumEncoder',
      basePowerMw: 1000.0,
      maxSomersloop: 4,
    ),
    const MachineInfo(name: 'Converter', basePowerMw: 250.0, maxSomersloop: 2),
  ];

  // Production recipes
  static final List<RecipeInfo> _recipes = [
    RecipeInfo(
      id: 'IronPlate',
      name: 'Iron Plate',
      machine: 'Constructor',
      inputs: const [RecipeItem(item: 'Iron Ingot', quantity: 3.0)],
      outputs: const [RecipeItem(item: 'Iron Plate', quantity: 2.0)],
    ),
    RecipeInfo(
      id: 'IronRod',
      name: 'Iron Rod',
      machine: 'Constructor',
      inputs: const [RecipeItem(item: 'Iron Ingot', quantity: 1.0)],
      outputs: const [RecipeItem(item: 'Iron Rod', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'Screw',
      name: 'Screw',
      machine: 'Constructor',
      inputs: const [RecipeItem(item: 'Iron Rod', quantity: 1.0)],
      outputs: const [RecipeItem(item: 'Screw', quantity: 4.0)],
    ),
    RecipeInfo(
      id: 'CopperSheet',
      name: 'Copper Sheet',
      machine: 'Constructor',
      inputs: const [RecipeItem(item: 'Copper Ingot', quantity: 2.0)],
      outputs: const [RecipeItem(item: 'Copper Sheet', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'Wire',
      name: 'Wire',
      machine: 'Constructor',
      inputs: const [RecipeItem(item: 'Copper Ingot', quantity: 1.0)],
      outputs: const [RecipeItem(item: 'Wire', quantity: 2.0)],
    ),
    RecipeInfo(
      id: 'Cable',
      name: 'Cable',
      machine: 'Constructor',
      inputs: const [RecipeItem(item: 'Wire', quantity: 2.0)],
      outputs: const [RecipeItem(item: 'Cable', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'Concrete',
      name: 'Concrete',
      machine: 'Constructor',
      inputs: const [RecipeItem(item: 'Limestone', quantity: 3.0)],
      outputs: const [RecipeItem(item: 'Concrete', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'SteelBeam',
      name: 'Steel Beam',
      machine: 'Constructor',
      inputs: const [RecipeItem(item: 'Steel Ingot', quantity: 4.0)],
      outputs: const [RecipeItem(item: 'Steel Beam', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'SteelPipe',
      name: 'Steel Pipe',
      machine: 'Constructor',
      inputs: const [RecipeItem(item: 'Steel Ingot', quantity: 3.0)],
      outputs: const [RecipeItem(item: 'Steel Pipe', quantity: 2.0)],
    ),
    RecipeInfo(
      id: 'Quickwire',
      name: 'Quickwire',
      machine: 'Constructor',
      inputs: const [RecipeItem(item: 'Caterium Ingot', quantity: 1.0)],
      outputs: const [RecipeItem(item: 'Quickwire', quantity: 5.0)],
    ),
    RecipeInfo(
      id: 'ReinforcedIronPlate',
      name: 'Reinforced Iron Plate',
      machine: 'Assembler',
      inputs: const [
        RecipeItem(item: 'Iron Plate', quantity: 6.0),
        RecipeItem(item: 'Screw', quantity: 12.0),
      ],
      outputs: const [RecipeItem(item: 'Reinforced Iron Plate', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'Rotor',
      name: 'Rotor',
      machine: 'Assembler',
      inputs: const [
        RecipeItem(item: 'Iron Rod', quantity: 5.0),
        RecipeItem(item: 'Screw', quantity: 25.0),
      ],
      outputs: const [RecipeItem(item: 'Rotor', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'ModularFrame',
      name: 'Modular Frame',
      machine: 'Assembler',
      inputs: const [
        RecipeItem(item: 'Reinforced Iron Plate', quantity: 3.0),
        RecipeItem(item: 'Iron Rod', quantity: 12.0),
      ],
      outputs: const [RecipeItem(item: 'Modular Frame', quantity: 2.0)],
    ),
    RecipeInfo(
      id: 'SmartPlating',
      name: 'Smart Plating',
      machine: 'Assembler',
      inputs: const [
        RecipeItem(item: 'Reinforced Iron Plate', quantity: 1.0),
        RecipeItem(item: 'Rotor', quantity: 1.0),
      ],
      outputs: const [RecipeItem(item: 'Smart Plating', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'Stator',
      name: 'Stator',
      machine: 'Assembler',
      inputs: const [
        RecipeItem(item: 'Steel Pipe', quantity: 3.0),
        RecipeItem(item: 'Wire', quantity: 8.0),
      ],
      outputs: const [RecipeItem(item: 'Stator', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'Motor',
      name: 'Motor',
      machine: 'Assembler',
      inputs: const [
        RecipeItem(item: 'Rotor', quantity: 2.0),
        RecipeItem(item: 'Stator', quantity: 2.0),
      ],
      outputs: const [RecipeItem(item: 'Motor', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'CircuitBoard',
      name: 'Circuit Board',
      machine: 'Assembler',
      inputs: const [
        RecipeItem(item: 'Copper Sheet', quantity: 2.0),
        RecipeItem(item: 'Plastic', quantity: 4.0),
      ],
      outputs: const [RecipeItem(item: 'Circuit Board', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'HeavyModularFrame',
      name: 'Heavy Modular Frame',
      machine: 'Manufacturer',
      inputs: const [
        RecipeItem(item: 'Modular Frame', quantity: 5.0),
        RecipeItem(item: 'Steel Pipe', quantity: 15.0),
        RecipeItem(item: 'Encased Industrial Beam', quantity: 5.0),
        RecipeItem(item: 'Screw', quantity: 100.0),
      ],
      outputs: const [RecipeItem(item: 'Heavy Modular Frame', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'Computer',
      name: 'Computer',
      machine: 'Manufacturer',
      inputs: const [
        RecipeItem(item: 'Circuit Board', quantity: 10.0),
        RecipeItem(item: 'Cable', quantity: 9.0),
        RecipeItem(item: 'Plastic', quantity: 18.0),
        RecipeItem(item: 'Screw', quantity: 52.0),
      ],
      outputs: const [RecipeItem(item: 'Computer', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'ModularEngine',
      name: 'Modular Engine',
      machine: 'Manufacturer',
      inputs: const [
        RecipeItem(item: 'Motor', quantity: 2.0),
        RecipeItem(item: 'Rubber', quantity: 15.0),
        RecipeItem(item: 'Smart Plating', quantity: 2.0),
      ],
      outputs: const [RecipeItem(item: 'Modular Engine', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'IronIngot',
      name: 'Iron Ingot',
      machine: 'Smelter',
      inputs: const [RecipeItem(item: 'Iron Ore', quantity: 1.0)],
      outputs: const [RecipeItem(item: 'Iron Ingot', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'CopperIngot',
      name: 'Copper Ingot',
      machine: 'Smelter',
      inputs: const [RecipeItem(item: 'Copper Ore', quantity: 1.0)],
      outputs: const [RecipeItem(item: 'Copper Ingot', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'CateriumIngot',
      name: 'Caterium Ingot',
      machine: 'Smelter',
      inputs: const [RecipeItem(item: 'Caterium Ore', quantity: 3.0)],
      outputs: const [RecipeItem(item: 'Caterium Ingot', quantity: 1.0)],
    ),
    RecipeInfo(
      id: 'SteelIngot',
      name: 'Steel Ingot',
      machine: 'Foundry',
      inputs: const [
        RecipeItem(item: 'Iron Ore', quantity: 3.0),
        RecipeItem(item: 'Coal', quantity: 3.0),
      ],
      outputs: const [RecipeItem(item: 'Steel Ingot', quantity: 3.0)],
    ),
    RecipeInfo(
      id: 'EncasedIndustrialBeam',
      name: 'Encased Industrial Beam',
      machine: 'Assembler',
      inputs: const [
        RecipeItem(item: 'Steel Beam', quantity: 4.0),
        RecipeItem(item: 'Concrete', quantity: 5.0),
      ],
      outputs: const [
        RecipeItem(item: 'Encased Industrial Beam', quantity: 1.0),
      ],
    ),
    RecipeInfo(
      id: 'Plastic',
      name: 'Plastic',
      machine: 'Refinery',
      inputs: const [RecipeItem(item: 'Crude Oil', quantity: 3.0)],
      outputs: const [
        RecipeItem(item: 'Plastic', quantity: 2.0),
        RecipeItem(item: 'Heavy Oil Residue', quantity: 1.0),
      ],
    ),
    RecipeInfo(
      id: 'Rubber',
      name: 'Rubber',
      machine: 'Refinery',
      inputs: const [RecipeItem(item: 'Crude Oil', quantity: 3.0)],
      outputs: const [
        RecipeItem(item: 'Rubber', quantity: 2.0),
        RecipeItem(item: 'Heavy Oil Residue', quantity: 2.0),
      ],
    ),
    RecipeInfo(
      id: 'Fuel',
      name: 'Fuel',
      machine: 'Refinery',
      inputs: const [RecipeItem(item: 'Crude Oil', quantity: 6.0)],
      outputs: const [
        RecipeItem(item: 'Fuel', quantity: 4.0),
        RecipeItem(item: 'Polymer Resin', quantity: 3.0),
      ],
    ),
  ];

  static List<ItemInfo> get items => List.unmodifiable(_items);
  static List<MachineInfo> get machines => List.unmodifiable(_machines);
  static List<RecipeInfo> get recipes => List.unmodifiable(_recipes);

  static ItemInfo? getItemByName(String name) {
    try {
      return _items.firstWhere((item) => item.name == name);
    } catch (_) {
      return null;
    }
  }

  static ItemInfo? getItemById(String id) {
    try {
      return _items.firstWhere((item) => item.id == id);
    } catch (_) {
      return null;
    }
  }

  static MachineInfo? getMachineByName(String name) {
    try {
      return _machines.firstWhere((machine) => machine.name == name);
    } catch (_) {
      return null;
    }
  }

  static RecipeInfo? getRecipeByName(String name) {
    try {
      return _recipes.firstWhere((recipe) => recipe.name == name);
    } catch (_) {
      return null;
    }
  }

  static RecipeInfo? getRecipeById(String id) {
    try {
      return _recipes.firstWhere((recipe) => recipe.id == id);
    } catch (_) {
      return null;
    }
  }

  static List<RecipeInfo> getRecipesByMachine(String machineName) {
    return _recipes.where((recipe) => recipe.machine == machineName).toList();
  }

  static List<RecipeInfo> searchRecipesByItem(String itemName) {
    final searchLower = itemName.toLowerCase();
    return _recipes.where((recipe) {
      return recipe.inputs.any(
            (input) => input.item.toLowerCase() == searchLower,
          ) ||
          recipe.outputs.any(
            (output) => output.item.toLowerCase() == searchLower,
          );
    }).toList();
  }

  static List<RecipeInfo> searchRecipesByName(String searchText) {
    final searchLower = searchText.toLowerCase();
    return _recipes
        .where((recipe) => recipe.name.toLowerCase().contains(searchLower))
        .toList();
  }
}

class _GameDataCacheState {
  DateTime? lastFetchTime;
  dynamic data;

  bool isValid() {
    if (lastFetchTime == null) return false;
    final now = DateTime.now();
    final diff = now.difference(lastFetchTime!);
    return diff.inMilliseconds < _cacheTtlMs;
  }
}

class _ItemsNotifier extends AsyncNotifier<List<ItemInfo>> {
  final _GameDataCacheState _cache = _GameDataCacheState();

  @override
  Future<List<ItemInfo>> build() async {
    return _fetchData();
  }

  Future<List<ItemInfo>> _fetchData() async {
    if (_cache.isValid() && _cache.data != null) {
      return _cache.data as List<ItemInfo>;
    }

    try {
      final data = GameData.items;
      _cache.data = data;
      _cache.lastFetchTime = DateTime.now();
      return data;
    } on Exception catch (e) {
      throw GameDataFailure('Failed to load items: ${e.toString()}');
    }
  }

  Future<void> refresh() async {
    state = const AsyncValue.loading();
    _cache.lastFetchTime = null;
    state = await AsyncValue.guard(_fetchData);
  }

  ItemInfo? getItemByName(String name) => GameData.getItemByName(name);
  ItemInfo? getItemById(String id) => GameData.getItemById(id);
}

class _MachinesNotifier extends AsyncNotifier<List<MachineInfo>> {
  final _GameDataCacheState _cache = _GameDataCacheState();

  @override
  Future<List<MachineInfo>> build() async {
    return _fetchData();
  }

  Future<List<MachineInfo>> _fetchData() async {
    if (_cache.isValid() && _cache.data != null) {
      return _cache.data as List<MachineInfo>;
    }

    try {
      final data = GameData.machines;
      _cache.data = data;
      _cache.lastFetchTime = DateTime.now();
      return data;
    } on Exception catch (e) {
      throw GameDataFailure('Failed to load machines: ${e.toString()}');
    }
  }

  Future<void> refresh() async {
    state = const AsyncValue.loading();
    _cache.lastFetchTime = null;
    state = await AsyncValue.guard(_fetchData);
  }

  MachineInfo? getMachineByName(String name) => GameData.getMachineByName(name);

  List<MachineInfo> getSomersloopMachines() {
    return GameData.machines
        .where((machine) => machine.maxSomersloop > 0)
        .toList();
  }
}

class _RecipesNotifier extends AsyncNotifier<List<RecipeInfo>> {
  final _GameDataCacheState _cache = _GameDataCacheState();

  @override
  Future<List<RecipeInfo>> build() async {
    return _fetchData();
  }

  Future<List<RecipeInfo>> _fetchData() async {
    if (_cache.isValid() && _cache.data != null) {
      return _cache.data as List<RecipeInfo>;
    }

    try {
      final data = GameData.recipes;
      _cache.data = data;
      _cache.lastFetchTime = DateTime.now();
      return data;
    } on Exception catch (e) {
      throw GameDataFailure('Failed to load recipes: ${e.toString()}');
    }
  }

  Future<void> refresh() async {
    state = const AsyncValue.loading();
    _cache.lastFetchTime = null;
    state = await AsyncValue.guard(_fetchData);
  }

  RecipeInfo? getRecipeByName(String name) => GameData.getRecipeByName(name);
  RecipeInfo? getRecipeById(String id) => GameData.getRecipeById(id);
}

final itemsProvider = AsyncNotifierProvider<_ItemsNotifier, List<ItemInfo>>(
  _ItemsNotifier.new,
);

final machinesProvider =
    AsyncNotifierProvider<_MachinesNotifier, List<MachineInfo>>(
      _MachinesNotifier.new,
    );

final recipesProvider =
    AsyncNotifierProvider<_RecipesNotifier, List<RecipeInfo>>(
      _RecipesNotifier.new,
    );

final recipesByMachineProvider =
    Provider<AsyncValue<Map<String, List<RecipeInfo>>>>((ref) {
      final recipesAsync = ref.watch(recipesProvider);

      return recipesAsync.when(
        data: (recipes) {
          final grouped = <String, List<RecipeInfo>>{};
          for (final recipe in recipes) {
            grouped.putIfAbsent(recipe.machine, () => []).add(recipe);
          }
          return AsyncValue.data(grouped);
        },
        loading: () => const AsyncValue.loading(),
        error: (err, stack) => AsyncValue.error(err, stack),
      );
    });

final recipeSearchProvider =
    Provider.family<AsyncValue<List<RecipeInfo>>, String>((ref, query) {
      final recipesAsync = ref.watch(recipesProvider);
      final searchLower = query.toLowerCase().trim();

      if (searchLower.isEmpty) {
        return recipesAsync;
      }

      return recipesAsync.when(
        data: (recipes) {
          final filtered = recipes.where((recipe) {
            if (recipe.name.toLowerCase().contains(searchLower)) return true;
            if (recipe.inputs.any(
              (input) => input.item.toLowerCase().contains(searchLower),
            )) {
              return true;
            }
            if (recipe.outputs.any(
              (output) => output.item.toLowerCase().contains(searchLower),
            )) {
              return true;
            }
            return false;
          }).toList();
          return AsyncValue.data(filtered);
        },
        loading: () => const AsyncValue.loading(),
        error: (err, stack) => AsyncValue.error(err, stack),
      );
    });

class GameDataFailure extends Failure {
  GameDataFailure(super.message);
}
