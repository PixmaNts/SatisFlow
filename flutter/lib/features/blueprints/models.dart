/// Machine types for production lines.
///
/// Corresponds to Rust MachineType enum from the engine.
enum MachineType {
  /// Constructor: Basic crafting machine (4 MW, max 1 somersloop)
  constructor,

  /// Assembler: Combines two items (16 MW, max 2 somersloop)
  assembler,

  /// Manufacturer: Complex crafting (32 MW, max 4 somersloop)
  manufacturer,

  /// Smelter: Basic ore processing (4 MW, max 1 somersloop)
  smelter,

  /// Foundry: Advanced ore processing (16 MW, max 2 somersloop)
  foundry,

  /// Refinery: Fluid processing (16 MW, max 2 somersloop)
  refinery,

  /// Blender: Advanced fluid processing (32 MW, max 4 somersloop)
  blender,

  /// Packager: Fluid packaging (4 MW, max 1 somersloop)
  packager,

  /// ParticleAccelerator: Advanced production (64 MW, max 4 somersloop)
  particleAccelerator,

  /// QuantumEncoder: Highest tier production (1000 MW avg, max 4 somersloop)
  quantumEncoder,

  /// Converter: Special processing (250 MW avg, max 2 somersloop)
  converter,

  /// Manual: Built manually (0 MW, 0 somersloop)
  manual,

  /// MinerMk1: Basic miner (5 MW)
  minerMk1,

  /// MinerMk2: Advanced miner (15 MW)
  minerMk2,

  /// MinerMk3: Highest tier miner (45 MW)
  minerMk3,

  /// WaterExtractor: Extracts water (20 MW)
  waterExtractor,

  /// OilExtractor: Extracts oil (40 MW)
  oilExtractor,

  /// ResourceWellExtractor: Extracts from resource wells (varies)
  resourceWellExtractor,
}

/// Extension on MachineType to provide utility methods.
extension MachineTypeExtension on MachineType {
  /// Returns the base power consumption in MW at 100% clock speed.
  double get basePowerMw {
    return switch (this) {
      MachineType.constructor => 4.0,
      MachineType.assembler => 16.0,
      MachineType.manufacturer => 32.0,
      MachineType.smelter => 4.0,
      MachineType.foundry => 16.0,
      MachineType.refinery => 16.0,
      MachineType.blender => 32.0,
      MachineType.packager => 4.0,
      MachineType.particleAccelerator => 64.0,
      MachineType.quantumEncoder => 1000.0,
      MachineType.converter => 250.0,
      MachineType.manual => 0.0,
      MachineType.minerMk1 => 5.0,
      MachineType.minerMk2 => 15.0,
      MachineType.minerMk3 => 45.0,
      MachineType.waterExtractor => 20.0,
      MachineType.oilExtractor => 40.0,
      MachineType.resourceWellExtractor => 0.0,
    };
  }

  /// Returns the maximum number of somersloops this machine type supports.
  int get maxSomersloop {
    return switch (this) {
      MachineType.constructor => 1,
      MachineType.assembler => 2,
      MachineType.manufacturer => 4,
      MachineType.smelter => 1,
      MachineType.foundry => 2,
      MachineType.refinery => 2,
      MachineType.blender => 4,
      MachineType.packager => 1,
      MachineType.particleAccelerator => 4,
      MachineType.quantumEncoder => 4,
      MachineType.converter => 2,
      MachineType.manual => 0,
      MachineType.minerMk1 => 0,
      MachineType.minerMk2 => 0,
      MachineType.minerMk3 => 0,
      MachineType.waterExtractor => 0,
      MachineType.oilExtractor => 0,
      MachineType.resourceWellExtractor => 0,
    };
  }

  /// Returns true if this machine type is an extractor (miner, pump, etc.).
  bool get isExtractor {
    return switch (this) {
      MachineType.minerMk1 ||
      MachineType.minerMk2 ||
      MachineType.minerMk3 ||
      MachineType.waterExtractor ||
      MachineType.oilExtractor ||
      MachineType.resourceWellExtractor => true,
      _ => false,
    };
  }

  /// Returns true if this machine type can use somersloops.
  bool get supportsSomersloop => maxSomersloop > 0;

  /// Returns the display name for this machine type.
  String get displayName {
    return switch (this) {
      MachineType.constructor => 'Constructor',
      MachineType.assembler => 'Assembler',
      MachineType.manufacturer => 'Manufacturer',
      MachineType.smelter => 'Smelter',
      MachineType.foundry => 'Foundry',
      MachineType.refinery => 'Refinery',
      MachineType.blender => 'Blender',
      MachineType.packager => 'Packager',
      MachineType.particleAccelerator => 'Particle Accelerator',
      MachineType.quantumEncoder => 'Quantum Encoder',
      MachineType.converter => 'Converter',
      MachineType.manual => 'Manual',
      MachineType.minerMk1 => 'Miner Mk.1',
      MachineType.minerMk2 => 'Miner Mk.2',
      MachineType.minerMk3 => 'Miner Mk.3',
      MachineType.waterExtractor => 'Water Extractor',
      MachineType.oilExtractor => 'Oil Extractor',
      MachineType.resourceWellExtractor => 'Resource Well Extractor',
    };
  }
}

/// Type alias for recipe identifiers.
///
/// Recipe IDs correspond to Rust Recipe enum variant names.
typedef RecipeId = String;

/// Custom parameter override for blueprint lines.
///
/// Used to store custom configuration values that deviate from defaults.
/// Common overrides include:
/// - overclock: Overclock percentage (0.0 - 250.0)
/// - somersloop: Number of somersloops per machine (0 - maxSomersloop)
/// - purity: Resource purity for extractors (impure, normal, pure)
/// - clockSpeed: Alternative overclock representation
/// - customName: User-defined name for this line
@pragma('vm:entry-point')
typedef OverrideKey = String;

@pragma('vm:entry-point')
typedef OverrideValue = dynamic;

/// Data class representing a production line within a blueprint.
///
/// This corresponds to the configuration data needed to instantiate
/// a ProductionLineRecipe in the Rust engine.
class BlueprintLineData {
  /// The type of machine for this production line.
  final MachineType machineType;

  /// The recipe identifier (matches Rust Recipe enum variant name).
  final RecipeId recipeId;

  /// Number of machines in this production line.
  final int count;

  /// Custom parameter overrides for this line.
  /// Keys: 'overclock', 'somersloop', 'purity', etc.
  final Map<OverrideKey, OverrideValue> overrides;

  /// Optional display name for this line.
  final String? name;

  /// Optional description for this line.
  final String? description;

  const BlueprintLineData({
    required this.machineType,
    required this.recipeId,
    required this.count,
    this.overrides = const {},
    this.name,
    this.description,
  });

  /// Creates a copy of this [BlueprintLineData] with the given fields replaced.
  BlueprintLineData copyWith({
    MachineType? machineType,
    RecipeId? recipeId,
    int? count,
    Map<OverrideKey, OverrideValue>? overrides,
    String? name,
    String? description,
  }) {
    return BlueprintLineData(
      machineType: machineType ?? this.machineType,
      recipeId: recipeId ?? this.recipeId,
      count: count ?? this.count,
      overrides: overrides ?? this.overrides,
      name: name ?? this.name,
      description: description ?? this.description,
    );
  }

  /// Returns the overclock value (defaults to 100.0 if not overridden).
  double get overclock {
    final value = overrides['overclock'] ?? overrides['clockSpeed'];
    if (value == null) return 100.0;
    if (value is double) return value;
    if (value is int) return value.toDouble();
    return double.tryParse(value.toString()) ?? 100.0;
  }

  /// Returns the somersloop count (defaults to 0 if not overridden).
  int get somersloop {
    final value = overrides['somersloop'];
    if (value == null) return 0;
    if (value is int) return value;
    if (value is double) return value.toInt();
    return int.tryParse(value.toString()) ?? 0;
  }

  /// Returns true if this line has valid configuration.
  bool get isValid {
    return count > 0 &&
        recipeId.isNotEmpty &&
        overclock >= 0.0 &&
        overclock <= 250.0 &&
        somersloop >= 0 &&
        somersloop <= machineType.maxSomersloop;
  }

  /// Returns validation errors if any, or null if valid.
  String? get validationError {
    if (count <= 0) {
      return 'Machine count must be greater than 0';
    }
    if (recipeId.isEmpty) {
      return 'Recipe ID cannot be empty';
    }
    if (overclock < 0.0 || overclock > 250.0) {
      return 'Overclock must be between 0 and 250';
    }
    if (somersloop < 0 || somersloop > machineType.maxSomersloop) {
      return 'Somersloop count must be between 0 and ${machineType.maxSomersloop}';
    }
    return null;
  }

  /// Converts this blueprint line data to a JSON-serializable map.
  Map<String, dynamic> toJson() {
    return {
      'machineType': machineType.name,
      'recipeId': recipeId,
      'count': count,
      'overrides': overrides,
      if (name != null) 'name': name,
      if (description != null) 'description': description,
    };
  }

  /// Creates a [BlueprintLineData] from a JSON map.
  factory BlueprintLineData.fromJson(Map<String, dynamic> json) {
    return BlueprintLineData(
      machineType: MachineType.values.byName(json['machineType'] as String),
      recipeId: json['recipeId'] as RecipeId,
      count: json['count'] as int,
      overrides:
          (json['overrides'] as Map<String, dynamic>?)?.map(
            (key, value) => MapEntry(key, value as OverrideValue),
          ) ??
          const {},
      name: json['name'] as String?,
      description: json['description'] as String?,
    );
  }

  @override
  String toString() {
    return 'BlueprintLineData('
        'machineType: ${machineType.displayName}, '
        'recipeId: $recipeId, '
        'count: $count, '
        'overclock: $overclock%, '
        'somersloop: $somersloop)';
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is BlueprintLineData &&
        other.machineType == machineType &&
        other.recipeId == recipeId &&
        other.count == count &&
        _mapEquals(other.overrides, overrides);
  }

  @override
  int get hashCode => Object.hash(machineType, recipeId, count, overrides);
}

/// Helper function to compare maps for equality.
bool _mapEquals<K, V>(Map<K, V> a, Map<K, V> b) {
  if (a.length != b.length) return false;
  for (final key in a.keys) {
    if (!b.containsKey(key) || b[key] != a[key]) return false;
  }
  return true;
}

/// Data class representing a machine group configuration.
///
/// A machine group represents a set of identical machines
/// with the same overclock and somersloop settings.
class MachineGroupData {
  /// Number of machines in this group.
  final int numberOfMachines;

  /// Overclock value (0.0 - 250.0).
  final double ocValue;

  /// Number of somersloops per machine.
  final int somersloop;

  const MachineGroupData({
    required this.numberOfMachines,
    required this.ocValue,
    required this.somersloop,
  });

  /// Creates a [MachineGroupData] with default values.
  factory MachineGroupData.defaults() {
    return const MachineGroupData(
      numberOfMachines: 1,
      ocValue: 100.0,
      somersloop: 0,
    );
  }

  /// Creates a copy with the given fields replaced.
  MachineGroupData copyWith({
    int? numberOfMachines,
    double? ocValue,
    int? somersloop,
  }) {
    return MachineGroupData(
      numberOfMachines: numberOfMachines ?? this.numberOfMachines,
      ocValue: ocValue ?? this.ocValue,
      somersloop: somersloop ?? this.somersloop,
    );
  }

  /// Returns true if this group has valid configuration.
  bool isValid(int maxSomersloop) {
    return numberOfMachines > 0 &&
        ocValue >= 0.0 &&
        ocValue <= 250.0 &&
        somersloop >= 0 &&
        somersloop <= maxSomersloop;
  }

  /// Converts to a JSON-serializable map.
  Map<String, dynamic> toJson() {
    return {
      'numberOfMachines': numberOfMachines,
      'ocValue': ocValue,
      'somersloop': somersloop,
    };
  }

  /// Creates from a JSON map.
  factory MachineGroupData.fromJson(Map<String, dynamic> json) {
    return MachineGroupData(
      numberOfMachines: json['numberOfMachines'] as int,
      ocValue: (json['ocValue'] as num).toDouble(),
      somersloop: json['somersloop'] as int,
    );
  }

  @override
  String toString() {
    return 'MachineGroupData('
        'machines: $numberOfMachines, '
        'oc: ${ocValue.toStringAsFixed(1)}%, '
        'somersloop: $somersloop)';
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is MachineGroupData &&
        other.numberOfMachines == numberOfMachines &&
        other.ocValue == ocValue &&
        other.somersloop == somersloop;
  }

  @override
  int get hashCode => Object.hash(numberOfMachines, ocValue, somersloop);
}

/// Complete blueprint data including multiple production lines.
///
/// This represents a reusable blueprint template that can be saved
/// and instantiated into factories.
class BlueprintData {
  /// Unique identifier for this blueprint.
  final String id;

  /// Display name of the blueprint.
  final String name;

  /// Optional description.
  final String? description;

  /// List of production lines in this blueprint.
  final List<BlueprintLineData> productionLines;

  /// Creation timestamp.
  final DateTime? createdAt;

  /// Last modification timestamp.
  final DateTime? updatedAt;

  const BlueprintData({
    required this.id,
    required this.name,
    this.description,
    this.productionLines = const [],
    this.createdAt,
    this.updatedAt,
  });

  /// Total number of machines across all lines.
  int get totalMachines {
    return productionLines.fold(0, (sum, line) => sum + line.count);
  }

  /// Total power consumption in MW (estimated).
  double get totalPowerMw {
    return productionLines.fold(0.0, (sum, line) {
      final basePower = line.machineType.basePowerMw;
      final ocMultiplier = line.overclock / 100.0;
      final somersloopRatio = line.somersloop > 0
          ? line.somersloop / line.machineType.maxSomersloop
          : 0.0;
      final somersloopMultiplier =
          (1.0 + somersloopRatio) * (1.0 + somersloopRatio);
      return sum +
          (basePower * somersloopMultiplier * ocMultiplier * line.count);
    });
  }

  /// Returns true if this blueprint has valid configuration.
  bool get isValid {
    return id.isNotEmpty &&
        name.isNotEmpty &&
        productionLines.isNotEmpty &&
        productionLines.every((line) => line.isValid);
  }

  /// Creates a copy with the given fields replaced.
  BlueprintData copyWith({
    String? id,
    String? name,
    String? description,
    List<BlueprintLineData>? productionLines,
    DateTime? createdAt,
    DateTime? updatedAt,
  }) {
    return BlueprintData(
      id: id ?? this.id,
      name: name ?? this.name,
      description: description ?? this.description,
      productionLines: productionLines ?? this.productionLines,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
    );
  }

  /// Converts to a JSON-serializable map.
  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      if (description != null) 'description': description,
      'productionLines': productionLines.map((l) => l.toJson()).toList(),
      if (createdAt != null) 'createdAt': createdAt!.toIso8601String(),
      if (updatedAt != null) 'updatedAt': updatedAt!.toIso8601String(),
    };
  }

  /// Creates from a JSON map.
  factory BlueprintData.fromJson(Map<String, dynamic> json) {
    return BlueprintData(
      id: json['id'] as String,
      name: json['name'] as String,
      description: json['description'] as String?,
      productionLines: (json['productionLines'] as List<dynamic>)
          .map((e) => BlueprintLineData.fromJson(e as Map<String, dynamic>))
          .toList(),
      createdAt: json['createdAt'] != null
          ? DateTime.parse(json['createdAt'] as String)
          : null,
      updatedAt: json['updatedAt'] != null
          ? DateTime.parse(json['updatedAt'] as String)
          : null,
    );
  }

  @override
  String toString() {
    return 'BlueprintData('
        'id: $id, '
        'name: $name, '
        'lines: ${productionLines.length}, '
        'totalMachines: $totalMachines)';
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is BlueprintData &&
        other.id == id &&
        other.name == name &&
        other.description == description &&
        _listEquals(other.productionLines, productionLines);
  }

  @override
  int get hashCode => Object.hash(id, name, productionLines);
}

/// Helper function to compare lists for equality.
bool _listEquals<T>(List<T> a, List<T> b) {
  if (a.length != b.length) return false;
  for (var i = 0; i < a.length; i++) {
    if (a[i] != b[i]) return false;
  }
  return true;
}
