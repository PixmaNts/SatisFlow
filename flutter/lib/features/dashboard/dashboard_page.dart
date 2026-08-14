import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../shared/widgets/widgets.dart';
import '../engine/providers/providers.dart';

/// Dashboard page - main entry point showing factory overview.
class DashboardPage extends ConsumerWidget {
  const DashboardPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Scaffold(
      body: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header
              Text(
                'Dashboard',
                style: theme.textTheme.headlineMedium?.copyWith(
                  fontWeight: FontWeight.bold,
                  color: colorScheme.onSurface,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                'Factory overview and statistics',
                style: theme.textTheme.bodyMedium?.copyWith(
                  color: colorScheme.onSurfaceVariant,
                ),
              ),
              const SizedBox(height: 24),

              // Main content
              Expanded(
                child: SingleChildScrollView(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      // Power Overview Section
                      _PowerOverviewSection(),
                      const SizedBox(height: 24),

                      // Item Balance Section
                      _ItemBalanceSection(),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

/// Power overview section showing total power generation vs consumption.
class _PowerOverviewSection extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    final powerStatsDataAsync = ref.watch(powerStatsDataProvider);
    final factoriesAsync = ref.watch(factoriesProvider);

    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: BorderSide(color: colorScheme.outlineVariant, width: 1),
      ),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Section header
            Row(
              children: [
                Icon(Icons.bolt, color: colorScheme.primary, size: 24),
                const SizedBox(width: 8),
                Text(
                  'Power Overview',
                  style: theme.textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.w600,
                    color: colorScheme.onSurface,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),

            // Power stats content
            powerStatsDataAsync.when(
              loading: () => const Center(
                child: Padding(
                  padding: EdgeInsets.all(32),
                  child: CircularProgressIndicator(),
                ),
              ),
              error: (error, stack) => _ErrorDisplay(
                message: 'Failed to load power statistics',
                error: error.toString(),
              ),
              data: (powerStatsData) {
                return factoriesAsync.when(
                  loading: () => const Center(
                    child: Padding(
                      padding: EdgeInsets.all(32),
                      child: CircularProgressIndicator(),
                    ),
                  ),
                  error: (error, stack) => _ErrorDisplay(
                    message: 'Failed to load factories',
                    error: error.toString(),
                  ),
                  data: (factories) {
                    return _PowerStatsContent(
                      powerStatsData: powerStatsData,
                      factoryCount: factories.length,
                    );
                  },
                );
              },
            ),
          ],
        ),
      ),
    );
  }
}

/// Content widget for power statistics.
class _PowerStatsContent extends StatelessWidget {
  final PowerStatsData powerStatsData;
  final int factoryCount;

  const _PowerStatsContent({
    required this.powerStatsData,
    required this.factoryCount,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    // Extract power stats values
    final totalGeneration = powerStatsData.totalGeneration;
    final totalConsumption = powerStatsData.totalConsumption;
    final powerBalance = powerStatsData.powerBalance;
    final factoryStats = powerStatsData.factoryStats;

    // Determine power status
    final hasSurplus = powerBalance > 0;
    final isBalanced = powerBalance.abs() < 0.001;
    final statusColor = isBalanced
        ? colorScheme.tertiary
        : hasSurplus
        ? colorScheme.primary
        : colorScheme.error;
    final statusText = isBalanced
        ? 'Balanced'
        : hasSurplus
        ? 'Surplus'
        : 'Deficit';

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Summary cards
        Row(
          children: [
            Expanded(
              child: _StatCard(
                title: 'Generation',
                value: '${totalGeneration.toStringAsFixed(1)} MW',
                icon: Icons.power,
                color: colorScheme.primary,
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: _StatCard(
                title: 'Consumption',
                value: '${totalConsumption.toStringAsFixed(1)} MW',
                icon: Icons.electric_meter,
                color: colorScheme.secondary,
              ),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: _StatCard(
                title: 'Balance',
                value: '${powerBalance.toStringAsFixed(1)} MW',
                icon: hasSurplus ? Icons.trending_up : Icons.trending_down,
                color: statusColor,
                subtitle: statusText,
              ),
            ),
          ],
        ),
        const SizedBox(height: 16),

        // Factory count
        Row(
          children: [
            Icon(Icons.factory, size: 20, color: colorScheme.onSurfaceVariant),
            const SizedBox(width: 8),
            Text(
              '$factoryCount ${factoryCount == 1 ? 'Factory' : 'Factories'}',
              style: theme.textTheme.bodyMedium?.copyWith(
                color: colorScheme.onSurfaceVariant,
              ),
            ),
          ],
        ),
        const SizedBox(height: 16),

        // Factory power breakdown
        if (factoryStats.isNotEmpty) ...[
          Text(
            'Factory Breakdown',
            style: theme.textTheme.titleSmall?.copyWith(
              fontWeight: FontWeight.w600,
              color: colorScheme.onSurface,
            ),
          ),
          const SizedBox(height: 12),
          ...factoryStats.map((factoryStat) {
            return _FactoryPowerTile(factoryStat: factoryStat);
          }),
        ],
      ],
    );
  }
}

/// A stat card widget.
class _StatCard extends StatelessWidget {
  final String title;
  final String value;
  final IconData icon;
  final Color color;
  final String? subtitle;

  const _StatCard({
    required this.title,
    required this.value,
    required this.icon,
    required this.color,
    this.subtitle,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: colorScheme.surfaceContainerHighest.withValues(alpha: 0.3),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: colorScheme.outlineVariant, width: 1),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(icon, size: 16, color: color),
              const SizedBox(width: 6),
              Text(
                title,
                style: theme.textTheme.labelSmall?.copyWith(
                  color: colorScheme.onSurfaceVariant,
                ),
              ),
            ],
          ),
          const SizedBox(height: 4),
          Text(
            value,
            style: theme.textTheme.titleMedium?.copyWith(
              fontWeight: FontWeight.bold,
              color: colorScheme.onSurface,
            ),
          ),
          if (subtitle != null) ...[
            const SizedBox(height: 2),
            Text(
              subtitle!,
              style: theme.textTheme.labelSmall?.copyWith(
                color: color,
                fontWeight: FontWeight.w500,
              ),
            ),
          ],
        ],
      ),
    );
  }
}

/// Factory power tile widget.
class _FactoryPowerTile extends StatelessWidget {
  final FactoryPowerStatsData factoryStat;

  const _FactoryPowerTile({required this.factoryStat});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    final factoryName = factoryStat.factoryName;
    final generation = factoryStat.generation;
    final consumption = factoryStat.consumption;
    final balance = factoryStat.balance;
    final generatorCount = factoryStat.generatorCount;

    final hasSurplus = balance > 0;
    final isBalanced = balance.abs() < 0.001;
    final balanceColor = isBalanced
        ? colorScheme.tertiary
        : hasSurplus
        ? colorScheme.primary
        : colorScheme.error;

    return Padding(
      padding: const EdgeInsets.only(bottom: 8),
      child: Container(
        padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(
          color: colorScheme.surface,
          borderRadius: BorderRadius.circular(8),
          border: Border.all(color: colorScheme.outlineVariant, width: 1),
        ),
        child: Row(
          children: [
            // Factory name
            Expanded(
              flex: 2,
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    factoryName,
                    style: theme.textTheme.bodyMedium?.copyWith(
                      fontWeight: FontWeight.w600,
                      color: colorScheme.onSurface,
                    ),
                  ),
                  const SizedBox(height: 2),
                  Text(
                    '$generatorCount generators',
                    style: theme.textTheme.labelSmall?.copyWith(
                      color: colorScheme.onSurfaceVariant,
                    ),
                  ),
                ],
              ),
            ),

            // Generation
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.end,
                children: [
                  Text(
                    'Gen',
                    style: theme.textTheme.labelSmall?.copyWith(
                      color: colorScheme.onSurfaceVariant,
                    ),
                  ),
                  Text(
                    '${generation.toStringAsFixed(1)} MW',
                    style: theme.textTheme.bodySmall?.copyWith(
                      color: colorScheme.primary,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                ],
              ),
            ),

            // Consumption
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.end,
                children: [
                  Text(
                    'Use',
                    style: theme.textTheme.labelSmall?.copyWith(
                      color: colorScheme.onSurfaceVariant,
                    ),
                  ),
                  Text(
                    '${consumption.toStringAsFixed(1)} MW',
                    style: theme.textTheme.bodySmall?.copyWith(
                      color: colorScheme.secondary,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                ],
              ),
            ),

            // Balance
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.end,
                children: [
                  Text(
                    'Bal',
                    style: theme.textTheme.labelSmall?.copyWith(
                      color: colorScheme.onSurfaceVariant,
                    ),
                  ),
                  Text(
                    '${balance.toStringAsFixed(1)} MW',
                    style: theme.textTheme.bodySmall?.copyWith(
                      color: balanceColor,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

/// Item balance section showing production/consumption per item.
class _ItemBalanceSection extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    final updateCycleAsync = ref.watch(updateCycleProvider);

    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: BorderSide(color: colorScheme.outlineVariant, width: 1),
      ),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Section header
            Row(
              children: [
                Icon(Icons.inventory_2, color: colorScheme.primary, size: 24),
                const SizedBox(width: 8),
                Text(
                  'Item Balance',
                  style: theme.textTheme.titleLarge?.copyWith(
                    fontWeight: FontWeight.w600,
                    color: colorScheme.onSurface,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              'Production and consumption per item',
              style: theme.textTheme.bodySmall?.copyWith(
                color: colorScheme.onSurfaceVariant,
              ),
            ),
            const SizedBox(height: 16),

            // Item balance content
            updateCycleAsync.when(
              loading: () => const Center(
                child: Padding(
                  padding: EdgeInsets.all(32),
                  child: CircularProgressIndicator(),
                ),
              ),
              error: (error, stack) => _ErrorDisplay(
                message: 'Failed to load item balances',
                error: error.toString(),
              ),
              data: (itemBalances) {
                if (itemBalances.isEmpty) {
                  return Center(
                    child: Padding(
                      padding: const EdgeInsets.all(32),
                      child: Column(
                        children: [
                          Icon(
                            Icons.inventory_2_outlined,
                            size: 48,
                            color: colorScheme.onSurfaceVariant,
                          ),
                          const SizedBox(height: 16),
                          Text(
                            'No item data available',
                            style: theme.textTheme.bodyLarge?.copyWith(
                              color: colorScheme.onSurfaceVariant,
                            ),
                          ),
                          const SizedBox(height: 8),
                          Text(
                            'Add production lines to see item balances',
                            style: theme.textTheme.bodySmall?.copyWith(
                              color: colorScheme.onSurfaceVariant,
                            ),
                          ),
                        ],
                      ),
                    ),
                  );
                }

                // Convert to list and sort by absolute balance
                final items = itemBalances.entries.toList()
                  ..sort((a, b) => b.value.abs().compareTo(a.value.abs()));

                return AppDataTable<MapEntry<String, double>>(
                  columns: [
                    DataTableColumn.text<MapEntry<String, double>>(
                      label: 'Item',
                      value: (item) => item.key,
                    ),
                    DataTableColumn.number<MapEntry<String, double>>(
                      label: 'Balance',
                      value: (item) => item.value,
                      formatter: (value) {
                        final sign = value >= 0 ? '+' : '';
                        return '$sign${value.toStringAsFixed(1)}/min';
                      },
                    ),
                  ],
                  data: items,
                  showFilter: true,
                  filterHint: 'Search items...',
                  rowsPerPage: 10,
                  showPagination: items.length > 10,
                  emptyState: Center(
                    child: Padding(
                      padding: const EdgeInsets.all(32),
                      child: Text(
                        'No items found',
                        style: theme.textTheme.bodyLarge?.copyWith(
                          color: colorScheme.onSurfaceVariant,
                        ),
                      ),
                    ),
                  ),
                );
              },
            ),
          ],
        ),
      ),
    );
  }
}

/// Error display widget.
class _ErrorDisplay extends StatelessWidget {
  final String message;
  final String error;

  const _ErrorDisplay({required this.message, required this.error});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Center(
      child: Padding(
        padding: const EdgeInsets.all(32),
        child: Column(
          children: [
            Icon(Icons.error_outline, size: 48, color: colorScheme.error),
            const SizedBox(height: 16),
            Text(
              message,
              style: theme.textTheme.bodyLarge?.copyWith(
                color: colorScheme.error,
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 8),
            Text(
              error,
              style: theme.textTheme.bodySmall?.copyWith(
                color: colorScheme.onSurfaceVariant,
              ),
              textAlign: TextAlign.center,
              maxLines: 3,
              overflow: TextOverflow.ellipsis,
            ),
          ],
        ),
      ),
    );
  }
}
