import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../src/rust/api.dart';
import '../../../core/errors/failures.dart';

/// Provider for the currently selected factory ID.
final currentFactoryIdProvider = StateProvider<String?>((ref) => null);

/// AsyncNotifier that manages the SatisflowEngine lifecycle.
class EngineNotifier extends AsyncNotifier<SatisflowEngine> {
  @override
  Future<SatisflowEngine> build() async {
    return newEngine();
  }

  /// Refreshes the engine by creating a new instance.
  Future<void> refresh() async {
    state = const AsyncValue.loading();
    state = await AsyncValue.guard(() => newEngine());
  }
}

/// Provider for the SatisflowEngine instance.
/// Initialize engine via `newEngine()` from api.dart.
final engineProvider = AsyncNotifierProvider<EngineNotifier, SatisflowEngine>(
  EngineNotifier.new,
);

/// Provider that watches all factories from the engine.
final factoriesProvider = FutureProvider<Map<String, Factory>>((ref) async {
  final engine = await ref.watch(engineProvider.future);
  return ffiGetAllFactories(engine: engine);
});

/// Family provider to get a single factory by ID.
final factoryByIdProvider = FutureProvider.family<Factory?, String>((
  ref,
  id,
) async {
  final engine = await ref.watch(engineProvider.future);
  return ffiGetFactory(engine: engine, id: id);
});

/// Controller for creating a new factory.
class CreateFactoryController extends AsyncNotifier<String> {
  Future<AsyncValue<String>> create({
    required String name,
    String? description,
  }) async {
    final engine = await ref.read(engineProvider.future);
    try {
      final result = await ffiCreateFactory(
        engine: engine,
        name: name,
        description: description,
      );
      // FfiResultString is opaque, we assume success if no exception
      // Invalidate factories to refresh the list
      ref.invalidate(factoriesProvider);
      return AsyncValue.data(result.hashCode.toString());
    } on Exception catch (e) {
      return AsyncValue.error(
        FactoryFailure('Failed to create factory: ${e.toString()}'),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(
        FactoryFailure('Failed to create factory: $e'),
        st,
      );
    }
  }

  @override
  Future<String> build() async {
    return '';
  }
}

/// Provider for factory creation.
final createFactoryController =
    AsyncNotifierProvider<CreateFactoryController, String>(
      CreateFactoryController.new,
    );

/// Controller for updating a factory.
///
/// The current Flutter FFI surface does not expose a factory update endpoint,
/// so this controller returns an explicit failure instead of pretending the
/// operation succeeded.
class UpdateFactoryController extends AsyncNotifier<void> {
  Future<AsyncValue<void>> updateFactory({
    required String id,
    required String name,
    String? description,
  }) async {
    final displayName = name.trim().isNotEmpty ? name.trim() : id;
    final requestedFields = description?.trim().isNotEmpty == true
        ? 'name and description'
        : 'name';

    return AsyncValue.error(
      FactoryFailure(
        'Factory $requestedFields updates for "$displayName" are unavailable '
        'because the Rust FFI bridge does not expose a factory update '
        'operation yet.',
        factoryId: id,
      ),
      StackTrace.current,
    );
  }

  @override
  Future<void> build() async {}
}

/// Provider for factory update - returns FactoryFailure when FFI not available.
final updateFactoryController =
    AsyncNotifierProvider<UpdateFactoryController, void>(
      UpdateFactoryController.new,
    );

/// Controller for deleting a factory.
class DeleteFactoryController extends AsyncNotifier<void> {
  Future<AsyncValue<void>> delete({required String id}) async {
    final engine = await ref.read(engineProvider.future);
    try {
      await ffiDeleteFactory(engine: engine, id: id);
      // Invalidate factories to refresh the list
      ref.invalidate(factoriesProvider);
      // Clear current factory if it was deleted
      final currentId = ref.read(currentFactoryIdProvider);
      if (currentId == id) {
        ref.read(currentFactoryIdProvider.notifier).state = null;
      }
      return const AsyncValue.data(null);
    } on Exception catch (e) {
      return AsyncValue.error(
        FactoryFailure(
          'Failed to delete factory: ${e.toString()}',
          factoryId: id,
        ),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(
        FactoryFailure('Failed to delete factory: $e', factoryId: id),
        st,
      );
    }
  }

  @override
  Future<void> build() async {}
}

/// Provider for factory deletion.
final deleteFactoryController =
    AsyncNotifierProvider<DeleteFactoryController, void>(
      DeleteFactoryController.new,
    );

/// Provider that watches all logistics lines from the engine.
final logisticsLinesProvider = FutureProvider<Map<String, LogisticsFlux>>((
  ref,
) async {
  final engine = await ref.watch(engineProvider.future);
  return ffiGetAllLogistics(engine: engine);
});

/// Provider that exposes the transport type label for each logistics line.
final logisticsTransportTypesProvider = FutureProvider<Map<String, String>>((
  ref,
) async {
  final engine = await ref.watch(engineProvider.future);
  return ffiGetAllLogisticsTransportTypes(engine: engine);
});

/// Family provider to get a single logistics line by ID.
final logisticsLineByIdProvider = FutureProvider.family<LogisticsFlux?, String>(
  (ref, id) async {
    final engine = await ref.watch(engineProvider.future);
    return ffiGetLogisticsLine(engine: engine, id: id);
  },
);

/// Controller for creating a new logistics line.
class CreateLogisticsLineController extends AsyncNotifier<String> {
  Future<AsyncValue<String>> create({
    required String fromId,
    required String toId,
    required String transportTypeName,
    required String transportDetail,
  }) async {
    final engine = await ref.read(engineProvider.future);
    try {
      // Create TransportType from string name since it's opaque in Flutter
      final transportType = await ffiCreateTransportType(
        typeName: transportTypeName,
      );
      final result = await ffiCreateLogisticsLine(
        engine: engine,
        fromId: fromId,
        toId: toId,
        transportType: transportType,
        transportDetail: transportDetail,
      );
      // Invalidate logistics to refresh the list
      ref.invalidate(logisticsLinesProvider);
      ref.invalidate(logisticsTransportTypesProvider);
      return AsyncValue.data(result.hashCode.toString());
    } on Exception catch (e) {
      return AsyncValue.error(
        LogisticsFailure('Failed to create logistics line: ${e.toString()}'),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(
        LogisticsFailure('Failed to create logistics line: $e'),
        st,
      );
    }
  }

  @override
  Future<String> build() async {
    return '';
  }
}

/// Provider for logistics line creation.
final createLogisticsLineController =
    AsyncNotifierProvider<CreateLogisticsLineController, String>(
      CreateLogisticsLineController.new,
    );

/// Controller for updating a logistics line.
class UpdateLogisticsLineController extends AsyncNotifier<void> {
  Future<AsyncValue<void>> updateLine({
    required String id,
    required String fromId,
    required String toId,
    required String transportTypeName,
    required String transportDetail,
  }) async {
    final engine = await ref.read(engineProvider.future);
    try {
      // Create TransportType from string name since it's opaque in Flutter
      final transportType = await ffiCreateTransportType(
        typeName: transportTypeName,
      );
      await ffiUpdateLogisticsLine(
        engine: engine,
        id: id,
        fromId: fromId,
        toId: toId,
        transportType: transportType,
        transportDetail: transportDetail,
      );
      // Invalidate logistics to refresh the list
      ref.invalidate(logisticsLinesProvider);
      ref.invalidate(logisticsTransportTypesProvider);
      return const AsyncValue.data(null);
    } on Exception catch (e) {
      return AsyncValue.error(
        LogisticsFailure(
          'Failed to update logistics line: ${e.toString()}',
          logisticsId: id,
        ),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(
        LogisticsFailure(
          'Failed to update logistics line: $e',
          logisticsId: id,
        ),
        st,
      );
    }
  }

  @override
  Future<void> build() async {}
}

/// Provider for logistics line update.
final updateLogisticsLineController =
    AsyncNotifierProvider<UpdateLogisticsLineController, void>(
      UpdateLogisticsLineController.new,
    );

/// Controller for deleting a logistics line.
class DeleteLogisticsLineController extends AsyncNotifier<void> {
  Future<AsyncValue<void>> delete({required String id}) async {
    final engine = await ref.read(engineProvider.future);
    try {
      await ffiDeleteLogisticsLine(engine: engine, id: id);
      // Invalidate logistics to refresh the list
      ref.invalidate(logisticsLinesProvider);
      ref.invalidate(logisticsTransportTypesProvider);
      return const AsyncValue.data(null);
    } on Exception catch (e) {
      return AsyncValue.error(
        LogisticsFailure(
          'Failed to delete logistics line: ${e.toString()}',
          logisticsId: id,
        ),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(
        LogisticsFailure(
          'Failed to delete logistics line: $e',
          logisticsId: id,
        ),
        st,
      );
    }
  }

  @override
  Future<void> build() async {}
}

/// Provider for logistics line deletion.
final deleteLogisticsLineController =
    AsyncNotifierProvider<DeleteLogisticsLineController, void>(
      DeleteLogisticsLineController.new,
    );

/// Provider that watches all blueprint templates from the engine.
final blueprintTemplatesProvider =
    FutureProvider<Map<String, ProductionLineBlueprint>>((ref) async {
      final engine = await ref.watch(engineProvider.future);
      return ffiGetAllBlueprintTemplates(engine: engine);
    });

/// Family provider to get a single blueprint template by ID.
final blueprintByIdProvider =
    FutureProvider.family<ProductionLineBlueprint?, String>((ref, id) async {
      final engine = await ref.watch(engineProvider.future);
      return ffiGetBlueprintTemplate(engine: engine, id: id);
    });

/// Controller for adding a blueprint template.
class AddBlueprintTemplateController extends AsyncNotifier<String> {
  Future<AsyncValue<String>> add({
    required ProductionLineBlueprint blueprint,
  }) async {
    final engine = await ref.read(engineProvider.future);
    try {
      final id = await ffiAddBlueprintTemplate(
        engine: engine,
        blueprint: blueprint,
      );
      // Invalidate blueprints to refresh the list
      ref.invalidate(blueprintTemplatesProvider);
      return AsyncValue.data(id);
    } on Exception catch (e) {
      return AsyncValue.error(
        BlueprintFailure('Failed to add blueprint: ${e.toString()}'),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(
        BlueprintFailure('Failed to add blueprint: $e'),
        st,
      );
    }
  }

  @override
  Future<String> build() async {
    return '';
  }
}

/// Provider for adding blueprint templates.
final addBlueprintTemplateController =
    AsyncNotifierProvider<AddBlueprintTemplateController, String>(
      AddBlueprintTemplateController.new,
    );

/// Controller for removing a blueprint template.
class RemoveBlueprintTemplateController extends AsyncNotifier<void> {
  Future<AsyncValue<void>> remove({required String id}) async {
    final engine = await ref.read(engineProvider.future);
    try {
      await ffiRemoveBlueprintTemplate(engine: engine, id: id);
      // Invalidate blueprints to refresh the list
      ref.invalidate(blueprintTemplatesProvider);
      return const AsyncValue.data(null);
    } on Exception catch (e) {
      return AsyncValue.error(
        BlueprintFailure(
          'Failed to remove blueprint: ${e.toString()}',
          blueprintId: id,
        ),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(
        BlueprintFailure('Failed to remove blueprint: $e', blueprintId: id),
        st,
      );
    }
  }

  @override
  Future<void> build() async {}
}

/// Provider for removing blueprint templates.
final removeBlueprintTemplateController =
    AsyncNotifierProvider<RemoveBlueprintTemplateController, void>(
      RemoveBlueprintTemplateController.new,
    );

/// Controller for saving engine state to JSON.
class SaveToJsonController extends AsyncNotifier<String> {
  Future<AsyncValue<String>> save() async {
    final engine = await ref.read(engineProvider.future);
    try {
      final result = await ffiSaveToJson(engine: engine);
      return AsyncValue.data(result);
    } on Exception catch (e) {
      return AsyncValue.error(
        StorageFailure('Failed to save: ${e.toString()}'),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(StorageFailure('Failed to save: $e'), st);
    }
  }

  @override
  Future<String> build() async {
    return '';
  }
}

/// Provider for saving engine state to JSON.
final saveToJsonController =
    AsyncNotifierProvider<SaveToJsonController, String>(
      SaveToJsonController.new,
    );

/// Controller for loading engine state from JSON.
class LoadFromJsonController extends AsyncNotifier<void> {
  Future<AsyncValue<void>> load({required String json}) async {
    final engine = await ref.read(engineProvider.future);
    try {
      await ffiLoadFromJson(engine: engine, json: json);
      // Invalidate all data providers to refresh
      ref.invalidate(factoriesProvider);
      ref.invalidate(logisticsLinesProvider);
      ref.invalidate(logisticsTransportTypesProvider);
      ref.invalidate(blueprintTemplatesProvider);
      return const AsyncValue.data(null);
    } on Exception catch (e) {
      return AsyncValue.error(
        StorageFailure('Failed to load: ${e.toString()}'),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(StorageFailure('Failed to load: $e'), st);
    }
  }

  @override
  Future<void> build() async {}
}

/// Provider for loading engine state from JSON.
final loadFromJsonController =
    AsyncNotifierProvider<LoadFromJsonController, void>(
      LoadFromJsonController.new,
    );

/// Controller for resetting the engine.
class ResetEngineController extends AsyncNotifier<void> {
  Future<AsyncValue<void>> reset() async {
    final engine = await ref.read(engineProvider.future);
    try {
      await ffiReset(engine: engine);
      // Invalidate all data providers to refresh
      ref.invalidate(factoriesProvider);
      ref.invalidate(logisticsLinesProvider);
      ref.invalidate(logisticsTransportTypesProvider);
      ref.invalidate(blueprintTemplatesProvider);
      // Clear current factory selection
      ref.read(currentFactoryIdProvider.notifier).state = null;
      return const AsyncValue.data(null);
    } on Exception catch (e) {
      return AsyncValue.error(
        EngineFailure('Failed to reset engine: ${e.toString()}'),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(EngineFailure('Failed to reset engine: $e'), st);
    }
  }

  @override
  Future<void> build() async {}
}

/// Provider for resetting the engine.
final resetEngineController =
    AsyncNotifierProvider<ResetEngineController, void>(
      ResetEngineController.new,
    );

/// Provider for global power statistics.
final powerStatsProvider = FutureProvider<PowerStats>((ref) async {
  final engine = await ref.watch(engineProvider.future);
  return ffiGlobalPowerStats(engine: engine);
});

/// Provider for running one update cycle and getting item balances.
final updateCycleProvider = FutureProvider<Map<String, double>>((ref) async {
  final engine = await ref.watch(engineProvider.future);
  return ffiUpdate(engine: engine);
});

/// Data class for power statistics that can be accessed from Flutter.
class PowerStatsData {
  final double totalGeneration;
  final double totalConsumption;
  final double powerBalance;
  final List<FactoryPowerStatsData> factoryStats;

  const PowerStatsData({
    required this.totalGeneration,
    required this.totalConsumption,
    required this.powerBalance,
    required this.factoryStats,
  });
}

/// Data class for factory power statistics.
class FactoryPowerStatsData {
  final String factoryId;
  final String factoryName;
  final double generation;
  final double consumption;
  final double balance;
  final int generatorCount;
  final List<String> generatorTypes;

  const FactoryPowerStatsData({
    required this.factoryId,
    required this.factoryName,
    required this.generation,
    required this.consumption,
    required this.balance,
    required this.generatorCount,
    required this.generatorTypes,
  });
}

/// Provider exposing fallback power-stats data for Flutter.
///
/// `PowerStats` is still opaque across the Rust FFI boundary, so this provider
/// returns empty values until the bridge exposes serializable power-stat fields.
final powerStatsDataProvider = FutureProvider<PowerStatsData>((ref) async {
  // Return empty values until the Rust bridge exposes concrete power stats.
  return const PowerStatsData(
    totalGeneration: 0.0,
    totalConsumption: 0.0,
    powerBalance: 0.0,
    factoryStats: [],
  );
});
