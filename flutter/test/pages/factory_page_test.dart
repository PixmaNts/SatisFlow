// Widget tests for FactoryPage

import 'dart:ffi';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:satisflow/features/factory/factory_page.dart';
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
  group('FactoryPage', () {
    testWidgets('renders without error', (WidgetTester tester) async {
      await tester.pumpWithProviders(
        const FactoryPage(),
        overrides: [
          factoriesProvider.overrideWith((ref) async => {}),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          currentFactoryIdProvider.overrideWith((ref) => null),
        ],
      );

      await tester.pump();

      expect(find.byType(FactoryPage), findsOneWidget);
    });

    testWidgets('shows app bar with Factory title', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const FactoryPage(),
        overrides: [
          factoriesProvider.overrideWith((ref) async => {}),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          currentFactoryIdProvider.overrideWith((ref) => null),
        ],
      );

      await tester.pump();

      expect(find.text('Factory'), findsOneWidget);
    });

    testWidgets('shows empty state when no factories', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const FactoryPage(),
        overrides: [
          factoriesProvider.overrideWith((ref) async => {}),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          currentFactoryIdProvider.overrideWith((ref) => null),
        ],
      );

      await tester.pump();

      expect(find.text('No factories'), findsOneWidget);
      expect(find.text('Create a factory to get started'), findsOneWidget);
    });

    testWidgets('shows loading indicator while loading', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const FactoryPage(),
        overrides: [
          factoriesProvider.overrideWith((ref) async {
            await Future.delayed(const Duration(milliseconds: 100));
            return {};
          }),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          currentFactoryIdProvider.overrideWith((ref) => null),
        ],
      );

      await tester.pump();

      expect(find.byType(CircularProgressIndicator), findsOneWidget);

      // Wait for the delayed future to complete
      await tester.pump(const Duration(milliseconds: 100));
    });

    testWidgets('shows error display on error', (WidgetTester tester) async {
      await tester.pumpWithProviders(
        const FactoryPage(),
        overrides: [
          factoriesProvider.overrideWith((ref) async {
            throw Exception('Failed to load factories');
          }),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          currentFactoryIdProvider.overrideWith((ref) => null),
        ],
      );

      await tester.pump();

      expect(find.textContaining('Failed to load factories'), findsOneWidget);
    });

    testWidgets('shows factory selector when factories exist', (
      WidgetTester tester,
    ) async {
      if (!_isRustLibAvailable()) {
        markTestSkipped('Rust library not available');
        return;
      }

      await tester.pumpWithProviders(
        const FactoryPage(),
        overrides: [
          factoriesProvider.overrideWith(
            (ref) async => <String, Factory>{'factory-123': null as Factory},
          ),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          currentFactoryIdProvider.overrideWith((ref) => 'factory-123'),
        ],
      );

      await tester.pumpAndSettle(const Duration(milliseconds: 100));

      expect(find.text('Factory:'), findsOneWidget);
      expect(find.byType(DropdownButton<String>), findsOneWidget);
    });

    testWidgets('shows tabs when factories exist', (WidgetTester tester) async {
      if (!_isRustLibAvailable()) {
        markTestSkipped('Rust library not available');
        return;
      }

      await tester.pumpWithProviders(
        const FactoryPage(),
        overrides: [
          factoriesProvider.overrideWith(
            (ref) async => <String, Factory>{'factory-123': null as Factory},
          ),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          currentFactoryIdProvider.overrideWith((ref) => 'factory-123'),
        ],
      );

      await tester.pumpAndSettle(const Duration(milliseconds: 100));

      expect(find.text('Production Lines'), findsOneWidget);
      expect(find.text('Raw Inputs'), findsOneWidget);
      expect(find.text('Power Generators'), findsOneWidget);
    });

    testWidgets('shows multiple factories in dropdown', (
      WidgetTester tester,
    ) async {
      if (!_isRustLibAvailable()) {
        markTestSkipped('Rust library not available');
        return;
      }

      await tester.pumpWithProviders(
        const FactoryPage(),
        overrides: [
          factoriesProvider.overrideWith(
            (ref) async => <String, Factory>{
              'factory-abc-123': null as Factory,
              'factory-xyz-789': null as Factory,
            },
          ),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          currentFactoryIdProvider.overrideWith((ref) => 'factory-abc-123'),
        ],
      );

      await tester.pumpAndSettle(const Duration(milliseconds: 100));

      // Tap dropdown to open it
      await tester.tap(find.byType(DropdownButton<String>));
      await tester.pumpAndSettle(const Duration(milliseconds: 100));

      // Should show truncated factory IDs
      expect(find.text('factory-a'), findsOneWidget);
      expect(find.text('factory-x'), findsOneWidget);
    });

    testWidgets('shows select factory hint when no factory selected', (
      WidgetTester tester,
    ) async {
      if (!_isRustLibAvailable()) {
        markTestSkipped('Rust library not available');
        return;
      }

      await tester.pumpWithProviders(
        const FactoryPage(),
        overrides: [
          factoriesProvider.overrideWith(
            (ref) async => <String, Factory>{'factory-123': null as Factory},
          ),
          preferencesProvider.overrideWith(() => MockPreferencesNotifier()),
          currentFactoryIdProvider.overrideWith((ref) => null),
        ],
      );

      await tester.pumpAndSettle(const Duration(milliseconds: 100));

      expect(find.text('Select a factory'), findsOneWidget);
    });

    testWidgets('displays tab content based on selected tab', (
      WidgetTester tester,
    ) async {
      if (!_isRustLibAvailable()) {
        markTestSkipped('Rust library not available');
        return;
      }

      await tester.pumpWithProviders(
        const FactoryPage(),
        overrides: [
          factoriesProvider.overrideWith(
            (ref) async => <String, Factory>{'factory-123': null as Factory},
          ),
          preferencesProvider.overrideWith(
            () => MockPreferencesNotifier(
              prefs: const UserPreferences(factoryViewTab: 'production'),
            ),
          ),
          currentFactoryIdProvider.overrideWith((ref) => 'factory-123'),
        ],
      );

      await tester.pumpAndSettle(const Duration(milliseconds: 100));

      // Should show production lines or a message for no factory selected
      expect(find.byType(TabBarView), findsOneWidget);
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
