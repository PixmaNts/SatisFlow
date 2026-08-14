import 'dart:io';

import 'package:file_picker/file_picker.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../src/rust/api.dart';
import '../../../core/errors/failures.dart';
import '../../features/engine/providers/providers.dart';

/// Controller for saving engine state to a file.
/// Uses file_picker to let the user choose where to save.
class SaveToFileController extends AsyncNotifier<void> {
  Future<AsyncValue<void>> save() async {
    try {
      // Get JSON from engine
      final engine = await ref.read(engineProvider.future);
      final jsonString = await ffiSaveToJson(engine: engine);

      // Let user pick save location
      final result = await FilePicker.platform.saveFile(
        dialogTitle: 'Save Satisflow State',
        fileName: 'satisflow_save.json',
        type: FileType.custom,
        allowedExtensions: ['json'],
      );

      if (result == null) {
        return const AsyncValue.data(null); // User cancelled
      }

      // Write file
      final file = File(result);
      await file.writeAsString(jsonString);

      return const AsyncValue.data(null);
    } on Exception catch (e) {
      return AsyncValue.error(
        StorageFailure('Failed to save file: ${e.toString()}'),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(StorageFailure('Failed to save file: $e'), st);
    }
  }

  @override
  Future<void> build() async {}
}

/// Provider for saving engine state to a file.
final saveToFileController = AsyncNotifierProvider<SaveToFileController, void>(
  SaveToFileController.new,
);

/// Controller for loading engine state from a file.
/// Uses file_picker to let the user choose a file to load.
class LoadFromFileController extends AsyncNotifier<void> {
  Future<AsyncValue<void>> load() async {
    try {
      // Let user pick a file
      final result = await FilePicker.platform.pickFiles(
        type: FileType.custom,
        allowedExtensions: ['json'],
        allowMultiple: false,
      );

      if (result == null || result.files.isEmpty) {
        return const AsyncValue.data(null); // User cancelled
      }

      final filePath = result.files.single.path;
      if (filePath == null) {
        return AsyncValue.error(
          const StorageFailure('Could not get file path'),
          StackTrace.current,
        );
      }

      // Read file and load into engine
      final file = File(filePath);
      final jsonString = await file.readAsString();

      final engine = await ref.read(engineProvider.future);
      await ffiLoadFromJson(engine: engine, json: jsonString);

      // Invalidate all data providers to refresh
      ref.invalidate(factoriesProvider);
      ref.invalidate(logisticsLinesProvider);
      ref.invalidate(logisticsTransportTypesProvider);
      ref.invalidate(blueprintTemplatesProvider);

      return const AsyncValue.data(null);
    } on Exception catch (e) {
      return AsyncValue.error(
        StorageFailure('Failed to load file: ${e.toString()}'),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(StorageFailure('Failed to load file: $e'), st);
    }
  }

  @override
  Future<void> build() async {}
}

/// Provider for loading engine state from a file.
final loadFromFileController =
    AsyncNotifierProvider<LoadFromFileController, void>(
      LoadFromFileController.new,
    );

/// Controller for exporting a blueprint template to a JSON file.
class ExportBlueprintController extends AsyncNotifier<void> {
  Future<AsyncValue<void>> export({required String blueprintId}) async {
    try {
      // Get blueprint from engine
      final engine = await ref.read(engineProvider.future);
      final blueprint = await ffiGetBlueprintTemplate(
        engine: engine,
        id: blueprintId,
      );

      if (blueprint == null) {
        return AsyncValue.error(
          BlueprintFailure('Blueprint not found', blueprintId: blueprintId),
          StackTrace.current,
        );
      }

      // Serialize blueprint to JSON
      final jsonString = await ffiBlueprintToJson(blueprint: blueprint);

      // Let user pick save location
      final result = await FilePicker.platform.saveFile(
        dialogTitle: 'Export Blueprint',
        fileName: 'blueprint.json',
        type: FileType.custom,
        allowedExtensions: ['json'],
      );

      if (result == null) {
        return const AsyncValue.data(null); // User cancelled
      }

      // Write file
      final file = File(result);
      await file.writeAsString(jsonString);

      return const AsyncValue.data(null);
    } on Exception catch (e) {
      return AsyncValue.error(
        BlueprintFailure(
          'Failed to export blueprint: ${e.toString()}',
          blueprintId: blueprintId,
        ),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(
        BlueprintFailure(
          'Failed to export blueprint: $e',
          blueprintId: blueprintId,
        ),
        st,
      );
    }
  }

  @override
  Future<void> build() async {}
}

/// Provider for exporting blueprint templates to files.
final exportBlueprintController =
    AsyncNotifierProvider<ExportBlueprintController, void>(
      ExportBlueprintController.new,
    );

/// Controller for importing a blueprint template from a JSON file.
class ImportBlueprintController extends AsyncNotifier<void> {
  Future<AsyncValue<void>> import() async {
    try {
      // Let user pick a file
      final result = await FilePicker.platform.pickFiles(
        type: FileType.custom,
        allowedExtensions: ['json'],
        allowMultiple: false,
      );

      if (result == null || result.files.isEmpty) {
        return const AsyncValue.data(null); // User cancelled
      }

      final filePath = result.files.single.path;
      if (filePath == null) {
        return AsyncValue.error(
          const StorageFailure('Could not get file path'),
          StackTrace.current,
        );
      }

      // Read file
      final file = File(filePath);
      final jsonString = await file.readAsString();

      // Deserialize blueprint from JSON
      final blueprint = await ffiBlueprintFromJson(json: jsonString);

      // Add blueprint to engine
      final engine = await ref.read(engineProvider.future);
      await ffiAddBlueprintTemplate(engine: engine, blueprint: blueprint);

      // Invalidate blueprints to refresh the list
      ref.invalidate(blueprintTemplatesProvider);

      return const AsyncValue.data(null);
    } on Exception catch (e) {
      return AsyncValue.error(
        BlueprintFailure('Failed to import blueprint: ${e.toString()}'),
        StackTrace.current,
      );
    } catch (e, st) {
      return AsyncValue.error(
        BlueprintFailure('Failed to import blueprint: $e'),
        st,
      );
    }
  }

  @override
  Future<void> build() async {}
}

/// Provider for importing blueprint templates from files.
final importBlueprintController =
    AsyncNotifierProvider<ImportBlueprintController, void>(
      ImportBlueprintController.new,
    );
