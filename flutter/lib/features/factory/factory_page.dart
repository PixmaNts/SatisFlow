import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/providers/preferences_provider.dart';
import '../../shared/widgets/widgets.dart';
import '../engine/providers/providers.dart';
import 'power_generators_tab.dart';
import 'production_lines_tab.dart';
import 'raw_inputs_tab.dart';

/// Factory page - manage production lines within factories.
class FactoryPage extends ConsumerWidget {
  const FactoryPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final factoriesAsync = ref.watch(factoriesProvider);
    final currentFactoryId = ref.watch(currentFactoryIdProvider);
    final factoryViewTab = ref.watch(factoryViewTabProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('Factory'), centerTitle: false),
      body: factoriesAsync.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (error, stack) => Center(
          child: ErrorDisplay(
            message: 'Failed to load factories: $error',
            onRetry: () => ref.invalidate(factoriesProvider),
          ),
        ),
        data: (factories) {
          if (factories.isEmpty) {
            return EmptyState(
              icon: Icons.factory,
              title: 'No factories',
              subtitle: 'Create a factory to get started',
            );
          }

          // Find the currently selected factory
          final selectedFactory = currentFactoryId != null
              ? factories[currentFactoryId]
              : null;

          return Column(
            children: [
              // Factory selector dropdown
              Padding(
                padding: const EdgeInsets.all(16.0),
                child: Row(
                  children: [
                    Text(
                      'Factory:',
                      style: Theme.of(context).textTheme.titleMedium,
                    ),
                    const SizedBox(width: 16),
                    Expanded(
                      child: DropdownButton<String>(
                        value: currentFactoryId,
                        hint: const Text('Select a factory'),
                        isExpanded: true,
                        items: factories.entries.map((entry) {
                          final shortId = entry.key.length > 8
                              ? entry.key.substring(0, 8)
                              : entry.key;
                          return DropdownMenuItem(
                            value: entry.key,
                            child: Text(shortId),
                          );
                        }).toList(),
                        onChanged: (factoryId) {
                          ref.read(currentFactoryIdProvider.notifier).state =
                              factoryId;
                        },
                      ),
                    ),
                  ],
                ),
              ),

              // Tab bar
              Expanded(
                child: DefaultTabController(
                  length: 3,
                  initialIndex: _getTabIndex(factoryViewTab),
                  child: Column(
                    children: [
                      TabBar(
                        onTap: (index) {
                          final tabName = _getTabName(index);
                          ref
                              .read(preferencesProvider.notifier)
                              .setFactoryViewTab(tabName);
                        },
                        tabs: const [
                          Tab(text: 'Production Lines'),
                          Tab(text: 'Raw Inputs'),
                          Tab(text: 'Power Generators'),
                        ],
                      ),
                      Expanded(
                        child: TabBarView(
                          children: [
                            // Production Lines tab
                            _buildProductionLinesTab(ref, selectedFactory),
                            // Raw Inputs tab
                            buildRawInputsTab(selectedFactory),
                            // Power Generators tab
                            PowerGeneratorsTab(factory: selectedFactory),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          );
        },
      ),
    );
  }

  Widget _buildProductionLinesTab(WidgetRef ref, dynamic factory) {
    return buildProductionLinesTab(ref, factory);
  }

  int _getTabIndex(String tabName) {
    switch (tabName) {
      case 'production':
        return 0;
      case 'raw_inputs':
        return 1;
      case 'power_generators':
        return 2;
      default:
        return 0;
    }
  }

  String _getTabName(int index) {
    switch (index) {
      case 0:
        return 'production';
      case 1:
        return 'raw_inputs';
      case 2:
        return 'power_generators';
      default:
        return 'production';
    }
  }
}
