import 'dart:ffi';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:satisflow/src/rust/api.dart'; // Contains newEngine and wrapper functions
import 'package:satisflow/src/rust/frb_generated.dart';

bool _isRustLibAvailable() {
  try {
    DynamicLibrary.open('librust_lib_satisflow.so');
    return true;
  } catch (_) {
    return false;
  }
}

void main() {
  final libAvailable = _isRustLibAvailable();

  group('FRB Engine Round-Trip Tests', () {
    setUpAll(() async {
      if (!libAvailable) {
        return;
      }
      await RustLib.init();
    });

    test('newEngine creates a valid engine instance', () async {
      if (!libAvailable) {
        markTestSkipped('Rust library not available - skipping FRB tests');
        return;
      }
      final engine = await newEngine();
      expect(engine, isNotNull);
    });

    test(
      'ffiCreateFactory creates factory - opaque type verification',
      () async {
        if (!libAvailable) {
          markTestSkipped('Rust library not available - skipping FRB tests');
          return;
        }
        final engine = await newEngine();

        // ffiCreateFactory returns FfiResultString (opaque)
        // Success means no exception was thrown
        final result = await ffiCreateFactory(
          engine: engine,
          name: 'Test Factory',
          description: 'A test factory',
        );

        // The result is an opaque FfiResultString - we verify it was created
        // by checking that subsequent operations work
        expect(result, isNotNull);
      },
    );

    test('ffiGetAllFactories returns factories after creation', () async {
      if (!libAvailable) {
        markTestSkipped('Rust library not available - skipping FRB tests');
        return;
      }
      final engine = await newEngine();

      // Initially empty
      var factories = await ffiGetAllFactories(engine: engine);
      expect(factories.length, equals(0));

      // Create a factory
      await ffiCreateFactory(
        engine: engine,
        name: 'Test Factory',
        description: null,
      );

      // Now should have 1 factory
      factories = await ffiGetAllFactories(engine: engine);
      expect(factories.length, equals(1));
    });

    test('ffiDeleteFactory removes factory', () async {
      if (!libAvailable) {
        markTestSkipped('Rust library not available - skipping FRB tests');
        return;
      }
      final engine = await newEngine();

      // Create a factory - result is opaque but call succeeds
      await ffiCreateFactory(
        engine: engine,
        name: 'To Be Deleted',
        description: null,
      );

      // Get factory ID from the map
      var factories = await ffiGetAllFactories(engine: engine);
      expect(factories.length, equals(1));

      // Get the first factory's ID
      final factoryId = factories.keys.first;

      // Delete it
      await ffiDeleteFactory(engine: engine, id: factoryId);

      // Verify it's gone
      factories = await ffiGetAllFactories(engine: engine);
      expect(factories.length, equals(0));
    });

    test('ffiReset clears all factories', () async {
      if (!libAvailable) {
        markTestSkipped('Rust library not available - skipping FRB tests');
        return;
      }
      final engine = await newEngine();

      // Create some factories
      await ffiCreateFactory(
        engine: engine,
        name: 'Factory 1',
        description: null,
      );
      await ffiCreateFactory(
        engine: engine,
        name: 'Factory 2',
        description: null,
      );

      // Verify they exist
      var factories = await ffiGetAllFactories(engine: engine);
      expect(factories.length, equals(2));

      // Reset
      await ffiReset(engine: engine);

      // Verify all gone
      factories = await ffiGetAllFactories(engine: engine);
      expect(factories.length, equals(0));
    });

    test('ffiSaveToJson and ffiLoadFromJson round-trip', () async {
      if (!libAvailable) {
        markTestSkipped('Rust library not available - skipping FRB tests');
        return;
      }
      final engine = await newEngine();

      // Create a factory
      await ffiCreateFactory(
        engine: engine,
        name: 'Saved Factory',
        description: 'Will be saved',
      );

      // Save to JSON - returns opaque type but contains the JSON
      final saveResult = await ffiSaveToJson(engine: engine);
      expect(saveResult, isNotNull);

      // Load into new engine
      final engine2 = await newEngine();
      await ffiLoadFromJson(engine: engine2, json: saveResult);

      // Verify factory loaded
      final factories = await ffiGetAllFactories(engine: engine2);
      expect(factories.length, equals(1));
    });
  });
}
