// Widget tests for LogisticsPage

import 'dart:ffi';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:satisflow/features/logistics/logistics_page.dart';
import 'package:satisflow/features/engine/providers/providers.dart';
import 'package:satisflow/src/rust/api.dart';
import 'package:satisflow/core/providers/preferences_provider.dart';

import '../helpers/test_helpers.dart';

bool _isRustLibAvailable() {
  try {
    DynamicLibrary.open('librust_lib_satisflow.so');
    return true;
  } catch (_) {
    return false;
  }
}

void main() {
  group('LogisticsPage', () {
    testWidgets('renders without error', (WidgetTester tester) async {
      await tester.pumpWithProviders(
        const LogisticsPage(),
        overrides: [
          logisticsLinesProvider.overrideWith((ref) async => {}),
          factoriesProvider.overrideWith((ref) async => {}),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
        ],
      );

      await tester.pump();

      expect(find.byType(LogisticsPage), findsOneWidget);
    });

    testWidgets('shows app bar with Logistics title', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const LogisticsPage(),
        overrides: [
          logisticsLinesProvider.overrideWith((ref) async => {}),
          factoriesProvider.overrideWith((ref) async => {}),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
        ],
      );

      await tester.pump();

      expect(find.text('Logistics'), findsOneWidget);
    });

    testWidgets('shows empty state when no logistics lines', (
      WidgetTester tester,
    ) async {
      if (!_isRustLibAvailable()) {
        markTestSkipped('Rust library not available');
        return;
      }

      await tester.pumpWithProviders(
        const LogisticsPage(),
        overrides: [
          logisticsLinesProvider.overrideWith((ref) async => {}),
          factoriesProvider.overrideWith((ref) async => {}),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          logisticsTransportTypesProvider.overrideWith((ref) async => {}),
        ],
      );

      await tester.pumpAndSettle(const Duration(milliseconds: 100));

      expect(find.text('No logistics lines'), findsOneWidget);
      expect(
        find.text('Create your first logistics line to connect factories.'),
        findsOneWidget,
      );
      expect(find.text('Add Logistics Line'), findsOneWidget);
    });

    testWidgets('shows loading indicator while loading', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const LogisticsPage(),
        overrides: [
          logisticsLinesProvider.overrideWith((ref) async {
            await Future.delayed(const Duration(milliseconds: 100));
            return {};
          }),
          factoriesProvider.overrideWith((ref) async => {}),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
        ],
      );

      await tester.pump();

      expect(find.byType(CircularProgressIndicator), findsOneWidget);

      // Wait for the delayed future to complete
      await tester.pump(const Duration(milliseconds: 100));
    });

    testWidgets('shows error display on logistics error', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const LogisticsPage(),
        overrides: [
          logisticsLinesProvider.overrideWith((ref) async {
            throw Exception('Failed to load logistics');
          }),
          factoriesProvider.overrideWith((ref) async => {}),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
        ],
      );

      await tester.pump();

      expect(find.textContaining('Failed to load logistics'), findsOneWidget);
    });

    testWidgets('shows error display on factories error', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const LogisticsPage(),
        overrides: [
          logisticsLinesProvider.overrideWith((ref) async => {}),
          factoriesProvider.overrideWith((ref) async {
            throw Exception('Failed to load factories');
          }),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
        ],
      );

      await tester.pump();

      expect(find.textContaining('Failed to load factories'), findsOneWidget);
    });

    testWidgets('shows search bar when logistics lines exist', (
      WidgetTester tester,
    ) async {
      if (!_isRustLibAvailable()) {
        markTestSkipped('Rust library not available');
        return;
      }

      await tester.pumpWithProviders(
        const LogisticsPage(),
        overrides: [
          logisticsLinesProvider.overrideWith(
            (ref) async => <String, LogisticsFlux>{
              'logistics-1': null as LogisticsFlux,
            },
          ),
          factoriesProvider.overrideWith((ref) async => {}),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          logisticsTransportTypesProvider.overrideWith(
            (ref) async => <String, String>{'logistics-1': 'Truck'},
          ),
        ],
      );

      await tester.pumpAndSettle(const Duration(milliseconds: 100));

      expect(find.byType(TextField), findsOneWidget);
    });

    testWidgets('shows filter chips when logistics lines exist', (
      WidgetTester tester,
    ) async {
      if (!_isRustLibAvailable()) {
        markTestSkipped('Rust library not available');
        return;
      }

      await tester.pumpWithProviders(
        const LogisticsPage(),
        overrides: [
          logisticsLinesProvider.overrideWith(
            (ref) async => <String, LogisticsFlux>{
              'logistics-1': null as LogisticsFlux,
            },
          ),
          factoriesProvider.overrideWith((ref) async => {}),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          logisticsTransportTypesProvider.overrideWith(
            (ref) async => <String, String>{'logistics-1': 'Truck'},
          ),
        ],
      );

      await tester.pumpAndSettle(const Duration(milliseconds: 100));

      expect(find.text('Bus'), findsOneWidget);
      expect(find.text('Train'), findsOneWidget);
      expect(find.text('Truck'), findsOneWidget);
      expect(find.text('Drone'), findsOneWidget);
    });

    testWidgets('shows floating action button', (WidgetTester tester) async {
      await tester.pumpWithProviders(
        const LogisticsPage(),
        overrides: [
          logisticsLinesProvider.overrideWith((ref) async => {}),
          factoriesProvider.overrideWith((ref) async => {}),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
        ],
      );

      await tester.pump();

      expect(find.byType(FloatingActionButton), findsOneWidget);
      expect(find.byIcon(Icons.add), findsOneWidget);
    });

    testWidgets('shows clear filters button in app bar', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const LogisticsPage(),
        overrides: [
          logisticsLinesProvider.overrideWith((ref) async => {}),
          factoriesProvider.overrideWith((ref) async => {}),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
        ],
      );

      await tester.pump();

      expect(find.byIcon(Icons.filter_list_off), findsOneWidget);
    });

    testWidgets('groups logistics by transport type', (
      WidgetTester tester,
    ) async {
      if (!_isRustLibAvailable()) {
        markTestSkipped('Rust library not available');
        return;
      }

      // Create mock logistics flux entries
      final mockFlux1 = _MockLogisticsFlux(
        fromFactory: 'factory-1',
        toFactory: 'factory-2',
        transportDetails: 'Route A',
      );

      await tester.pumpWithProviders(
        const LogisticsPage(),
        overrides: [
          logisticsLinesProvider.overrideWith(
            (ref) async => <String, LogisticsFlux>{
              'logistics-1': mockFlux1 as LogisticsFlux,
            },
          ),
          factoriesProvider.overrideWith(
            (ref) async => <String, Factory>{
              'factory-1': null as Factory,
              'factory-2': null as Factory,
            },
          ),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          logisticsTransportTypesProvider.overrideWith(
            (ref) async => <String, String>{'logistics-1': 'Bus'},
          ),
        ],
      );

      await tester.pumpAndSettle(const Duration(milliseconds: 100));

      // Should show Bus as transport type group
      expect(find.text('Bus'), findsWidgets);
    });
  });
}

/// Mock preferences notifier for testing
class MockPreferencesNotifier extends PreferencesNotifier {
  final UserPreferences? _prefs;

  MockPreferencesNotifier({UserPreferences? prefs}) : _prefs = prefs;

  @override
  Future<UserPreferences> build() async {
    return _prefs ?? UserPreferences.defaults;
  }
}

/// Mock LogisticsFlux for testing
class _MockLogisticsFlux {
  final String fromFactory;
  final String toFactory;
  final String transportDetails;

  _MockLogisticsFlux({
    required this.fromFactory,
    required this.toFactory,
    required this.transportDetails,
  });
}
