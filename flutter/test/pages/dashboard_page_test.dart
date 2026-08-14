// Widget tests for DashboardPage

import 'dart:ffi';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:satisflow/features/dashboard/dashboard_page.dart';
import 'package:satisflow/features/engine/providers/providers.dart';
import 'package:satisflow/src/rust/api.dart';

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
  group('DashboardPage', () {
    testWidgets('renders without error', (WidgetTester tester) async {
      await tester.pumpWithProviders(
        const DashboardPage(),
        overrides: [
          powerStatsDataProvider.overrideWith(
            (ref) async => const PowerStatsData(
              totalGeneration: 100.0,
              totalConsumption: 50.0,
              powerBalance: 50.0,
              factoryStats: [],
            ),
          ),
          factoriesProvider.overrideWith((ref) async => {}),
          updateCycleProvider.overrideWith((ref) async => {}),
        ],
      );

      await tester.pump();

      expect(find.byType(DashboardPage), findsOneWidget);
      expect(find.text('Dashboard'), findsOneWidget);
      expect(find.text('Factory overview and statistics'), findsOneWidget);
    });

    testWidgets('shows loading indicators while data loads', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const DashboardPage(),
        overrides: [
          powerStatsDataProvider.overrideWith((ref) async {
            // Simulate loading
            await Future.delayed(const Duration(milliseconds: 100));
            return const PowerStatsData(
              totalGeneration: 0.0,
              totalConsumption: 0.0,
              powerBalance: 0.0,
              factoryStats: [],
            );
          }),
          factoriesProvider.overrideWith((ref) async => {}),
          updateCycleProvider.overrideWith((ref) async => {}),
        ],
      );

      await tester.pump();

      // Should show loading indicators initially
      expect(find.byType(CircularProgressIndicator), findsWidgets);

      // Wait for the delayed future to complete
      await tester.pump(const Duration(milliseconds: 100));
    });

    testWidgets('displays Power Overview section', (WidgetTester tester) async {
      await tester.pumpWithProviders(
        const DashboardPage(),
        overrides: [
          powerStatsDataProvider.overrideWith(
            (ref) async => const PowerStatsData(
              totalGeneration: 150.5,
              totalConsumption: 75.2,
              powerBalance: 75.3,
              factoryStats: [],
            ),
          ),
          factoriesProvider.overrideWith((ref) async => {}),
          updateCycleProvider.overrideWith((ref) async => {}),
        ],
      );

      await tester.pump();

      expect(find.text('Power Overview'), findsOneWidget);
    });

    testWidgets('displays Item Balance section', (WidgetTester tester) async {
      await tester.pumpWithProviders(
        const DashboardPage(),
        overrides: [
          powerStatsDataProvider.overrideWith(
            (ref) async => const PowerStatsData(
              totalGeneration: 0.0,
              totalConsumption: 0.0,
              powerBalance: 0.0,
              factoryStats: [],
            ),
          ),
          factoriesProvider.overrideWith((ref) async => {}),
          updateCycleProvider.overrideWith((ref) async => {}),
        ],
      );

      await tester.pump();

      expect(find.text('Item Balance'), findsOneWidget);
      expect(find.text('Production and consumption per item'), findsOneWidget);
    });

    testWidgets('shows factory count in Power Overview', (
      WidgetTester tester,
    ) async {
      if (!_isRustLibAvailable()) {
        markTestSkipped('Rust library not available');
        return;
      }

      await tester.pumpWithProviders(
        const DashboardPage(),
        overrides: [
          powerStatsDataProvider.overrideWith(
            (ref) async => const PowerStatsData(
              totalGeneration: 100.0,
              totalConsumption: 50.0,
              powerBalance: 50.0,
              factoryStats: [],
            ),
          ),
          factoriesProvider.overrideWith(
            (ref) async => <String, Factory>{
              'factory-1': null as Factory,
              'factory-2': null as Factory,
            },
          ),
          updateCycleProvider.overrideWith((ref) async => {}),
        ],
      );

      await tester.pumpAndSettle(const Duration(milliseconds: 100));

      expect(find.text('2 Factories'), findsOneWidget);
    });

    testWidgets('shows single factory text when count is 1', (
      WidgetTester tester,
    ) async {
      if (!_isRustLibAvailable()) {
        markTestSkipped('Rust library not available');
        return;
      }

      await tester.pumpWithProviders(
        const DashboardPage(),
        overrides: [
          powerStatsDataProvider.overrideWith(
            (ref) async => const PowerStatsData(
              totalGeneration: 100.0,
              totalConsumption: 50.0,
              powerBalance: 50.0,
              factoryStats: [],
            ),
          ),
          factoriesProvider.overrideWith(
            (ref) async => <String, Factory>{'factory-1': null as Factory},
          ),
          updateCycleProvider.overrideWith((ref) async => {}),
        ],
      );

      await tester.pumpAndSettle(const Duration(milliseconds: 100));

      expect(find.text('1 Factory'), findsOneWidget);
    });

    testWidgets('displays item balances in table', (WidgetTester tester) async {
      await tester.pumpWithProviders(
        const DashboardPage(),
        overrides: [
          powerStatsDataProvider.overrideWith(
            (ref) async => const PowerStatsData(
              totalGeneration: 0.0,
              totalConsumption: 0.0,
              powerBalance: 0.0,
              factoryStats: [],
            ),
          ),
          factoriesProvider.overrideWith((ref) async => {}),
          updateCycleProvider.overrideWith(
            (ref) async => {
              'Iron Plate': 120.0,
              'Iron Rod': -60.0,
              'Screw': 240.0,
            },
          ),
        ],
      );

      await tester.pump();

      expect(find.text('Iron Plate'), findsOneWidget);
      expect(find.text('Iron Rod'), findsOneWidget);
      expect(find.text('Screw'), findsOneWidget);
    });

    testWidgets('shows empty state when no item balances', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const DashboardPage(),
        overrides: [
          powerStatsDataProvider.overrideWith(
            (ref) async => const PowerStatsData(
              totalGeneration: 0.0,
              totalConsumption: 0.0,
              powerBalance: 0.0,
              factoryStats: [],
            ),
          ),
          factoriesProvider.overrideWith((ref) async => {}),
          updateCycleProvider.overrideWith((ref) async => {}),
        ],
      );

      await tester.pump();

      expect(find.text('No item data available'), findsOneWidget);
      expect(
        find.text('Add production lines to see item balances'),
        findsOneWidget,
      );
    });

    testWidgets('displays error state when power stats fail', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const DashboardPage(),
        overrides: [
          powerStatsDataProvider.overrideWith((ref) async {
            throw Exception('Failed to load power stats');
          }),
          factoriesProvider.overrideWith((ref) async => {}),
          updateCycleProvider.overrideWith((ref) async => {}),
        ],
      );

      await tester.pump();

      expect(find.text('Failed to load power statistics'), findsOneWidget);
    });

    testWidgets(
      'displays factory power breakdown when factories have power stats',
      (WidgetTester tester) async {
        await tester.pumpWithProviders(
          const DashboardPage(),
          overrides: [
            powerStatsDataProvider.overrideWith(
              (ref) async => PowerStatsData(
                totalGeneration: 200.0,
                totalConsumption: 100.0,
                powerBalance: 100.0,
                factoryStats: [
                  FactoryPowerStatsData(
                    factoryId: 'factory-1',
                    factoryName: 'Main Factory',
                    generation: 150.0,
                    consumption: 75.0,
                    balance: 75.0,
                    generatorCount: 2,
                    generatorTypes: ['Biomass Burner', 'Coal Power'],
                  ),
                ],
              ),
            ),
            factoriesProvider.overrideWith((ref) async => {}),
            updateCycleProvider.overrideWith((ref) async => {}),
          ],
        );

        await tester.pump();

        expect(find.text('Factory Breakdown'), findsOneWidget);
        expect(find.text('Main Factory'), findsOneWidget);
        expect(find.text('2 generators'), findsOneWidget);
      },
    );

    testWidgets('displays power stats with surplus status', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const DashboardPage(),
        overrides: [
          powerStatsDataProvider.overrideWith(
            (ref) async => const PowerStatsData(
              totalGeneration: 200.0,
              totalConsumption: 100.0,
              powerBalance: 100.0,
              factoryStats: [],
            ),
          ),
          factoriesProvider.overrideWith((ref) async => {}),
          updateCycleProvider.overrideWith((ref) async => {}),
        ],
      );

      await tester.pump();

      expect(find.text('Surplus'), findsOneWidget);
    });

    testWidgets('displays power stats with deficit status', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const DashboardPage(),
        overrides: [
          powerStatsDataProvider.overrideWith(
            (ref) async => const PowerStatsData(
              totalGeneration: 100.0,
              totalConsumption: 200.0,
              powerBalance: -100.0,
              factoryStats: [],
            ),
          ),
          factoriesProvider.overrideWith((ref) async => {}),
          updateCycleProvider.overrideWith((ref) async => {}),
        ],
      );

      await tester.pump();

      expect(find.text('Deficit'), findsOneWidget);
    });

    testWidgets('displays power stats with balanced status', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithProviders(
        const DashboardPage(),
        overrides: [
          powerStatsDataProvider.overrideWith(
            (ref) async => const PowerStatsData(
              totalGeneration: 100.0,
              totalConsumption: 100.0,
              powerBalance: 0.0,
              factoryStats: [],
            ),
          ),
          factoriesProvider.overrideWith((ref) async => {}),
          updateCycleProvider.overrideWith((ref) async => {}),
        ],
      );

      await tester.pump();

      expect(find.text('Balanced'), findsOneWidget);
    });
  });
}
