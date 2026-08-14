import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/providers/preferences_provider.dart';
import '../../shared/widgets/search_bar.dart';
import '../../shared/widgets/filter_chip_bar.dart';
import '../../shared/widgets/confirm_dialog.dart';
import '../../shared/widgets/empty_state.dart';
import '../../shared/widgets/error_display.dart';
import '../../shared/widgets/loading_overlay.dart';
import '../../src/rust/api.dart';
import '../engine/providers/providers.dart';
import 'logistics_form.dart';

/// Logistics page - manage transport between factories.
class LogisticsPage extends ConsumerStatefulWidget {
  const LogisticsPage({super.key});

  @override
  ConsumerState<LogisticsPage> createState() => _LogisticsPageState();
}

class _LogisticsPageState extends ConsumerState<LogisticsPage> {
  final TextEditingController _searchController = TextEditingController();
  String _searchQuery = '';
  Set<String> _selectedTransportTypes = {};

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  String _getTransportType(
    String logisticsId,
    Map<String, String> transportTypes,
  ) {
    return transportTypes[logisticsId] ?? 'Unknown';
  }

  @override
  Widget build(BuildContext context) {
    final logisticsAsync = ref.watch(logisticsLinesProvider);
    final factoriesAsync = ref.watch(factoriesProvider);
    final transportTypesAsync = ref.watch(logisticsTransportTypesProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Logistics'),
        actions: [
          IconButton(
            icon: const Icon(Icons.filter_list_off),
            onPressed: _clearFilters,
            tooltip: 'Clear filters',
          ),
        ],
      ),
      body: logisticsAsync.when(
        loading: () => const LoadingOverlay(isLoading: true, child: SizedBox()),
        error: (error, stack) => ErrorDisplay(
          message: 'Failed to load logistics lines',
          onRetry: () => ref.invalidate(logisticsLinesProvider),
        ),
        data: (logisticsMap) {
          return factoriesAsync.when(
            loading: () =>
                const LoadingOverlay(isLoading: true, child: SizedBox()),
            error: (error, stack) => ErrorDisplay(
              message: 'Failed to load factories',
              onRetry: () => ref.invalidate(factoriesProvider),
            ),
            data: (factoriesMap) {
              return transportTypesAsync.when(
                loading: () =>
                    const LoadingOverlay(isLoading: true, child: SizedBox()),
                error: (error, stack) => ErrorDisplay(
                  message: 'Failed to load logistics transport types',
                  onRetry: () =>
                      ref.invalidate(logisticsTransportTypesProvider),
                ),
                data: (transportTypes) {
                  final logisticsList = logisticsMap.entries.toList();
                  final filtered = _applyFilters(
                    logisticsList,
                    factoriesMap,
                    transportTypes,
                  );
                  final grouped = _groupByTransportType(
                    filtered,
                    transportTypes,
                  );

                  return _buildContent(grouped, factoriesMap);
                },
              );
            },
          );
        },
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _addLogisticsLine,
        child: const Icon(Icons.add),
      ),
    );
  }

  Widget _buildContent(
    Map<String, List<MapEntry<String, dynamic>>> grouped,
    Map<String, dynamic> factoriesMap,
  ) {
    if (grouped.isEmpty) {
      return EmptyState(
        icon: Icons.local_shipping,
        title: 'No logistics lines',
        subtitle: 'Create your first logistics line to connect factories.',
        action: ElevatedButton(
          onPressed: _addLogisticsLine,
          child: const Text('Add Logistics Line'),
        ),
      );
    }

    return Column(
      children: [
        // Search bar
        Padding(
          padding: const EdgeInsets.all(16.0),
          child: AppSearchBar(
            hintText: 'Search logistics lines...',
            controller: _searchController,
            onChanged: (value) {
              setState(() {
                _searchQuery = value;
              });
            },
          ),
        ),
        // Filter chips
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16.0),
          child: FilterChipBar(
            options: const [
              FilterChipOption(label: 'Bus', value: 'Bus', icon: Icons.train),
              FilterChipOption(
                label: 'Train',
                value: 'Train',
                icon: Icons.train,
              ),
              FilterChipOption(
                label: 'Truck',
                value: 'Truck',
                icon: Icons.local_shipping,
              ),
              FilterChipOption(
                label: 'Drone',
                value: 'Drone',
                icon: Icons.airplanemode_active,
              ),
            ],
            selectedValues: _selectedTransportTypes,
            onSelectionChanged: (selected) {
              setState(() {
                _selectedTransportTypes = selected;
              });
            },
            onClear: _clearFilters,
          ),
        ),
        const SizedBox(height: 8),
        // Grouped list
        Expanded(
          child: ListView.builder(
            itemCount: grouped.length,
            itemBuilder: (context, index) {
              final transportType = grouped.keys.elementAt(index);
              final lines = grouped[transportType]!;
              return _buildTransportGroup(transportType, lines, factoriesMap);
            },
          ),
        ),
      ],
    );
  }

  Widget _buildTransportGroup(
    String transportType,
    List<MapEntry<String, dynamic>> lines,
    Map<String, dynamic> factoriesMap,
  ) {
    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: ExpansionTile(
        title: Text(
          transportType,
          style: Theme.of(
            context,
          ).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.bold),
        ),
        subtitle: Text('${lines.length} line${lines.length == 1 ? '' : 's'}'),
        initiallyExpanded: true,
        children: lines.map((entry) {
          final id = entry.key;
          final flux = entry.value;
          final fromFactory = factoriesMap[flux.fromFactory];
          final toFactory = factoriesMap[flux.toFactory];
          final fromName = fromFactory?.name ?? 'Unknown';
          final toName = toFactory?.name ?? 'Unknown';
          final detail = (flux.transportDetails as String?) ?? '';

          return _buildLogisticsTile(
            id,
            fromName,
            toName,
            transportType,
            detail,
          );
        }).toList(),
      ),
    );
  }

  Widget _buildLogisticsTile(
    String id,
    String fromName,
    String toName,
    String transportType,
    String detail,
  ) {
    return Dismissible(
      key: Key(id),
      direction: DismissDirection.endToStart,
      background: Container(
        alignment: Alignment.centerRight,
        padding: const EdgeInsets.only(right: 16),
        color: Theme.of(context).colorScheme.error,
        child: Icon(Icons.delete, color: Theme.of(context).colorScheme.onError),
      ),
      confirmDismiss: (direction) async {
        return await ConfirmDialog.showDelete(
          context: context,
          title: 'Delete Logistics Line',
          message:
              'Are you sure you want to delete this logistics line from $fromName to $toName?',
        );
      },
      onDismissed: (direction) {
        _deleteLogisticsLine(id);
      },
      child: ListTile(
        title: Text('$fromName → $toName'),
        subtitle: Text('$transportType: $detail'),
        trailing: const Icon(Icons.chevron_right),
        onTap: () => _editLogisticsLine(id),
      ),
    );
  }

  List<MapEntry<String, dynamic>> _applyFilters(
    List<MapEntry<String, dynamic>> logisticsList,
    Map<String, dynamic> factoriesMap,
    Map<String, String> transportTypes,
  ) {
    var filtered = logisticsList;

    // Filter by search query
    if (_searchQuery.isNotEmpty) {
      final query = _searchQuery.toLowerCase();
      filtered = filtered.where((entry) {
        final flux = entry.value;
        final fromFactory = factoriesMap[flux.fromFactory];
        final toFactory = factoriesMap[flux.toFactory];
        final fromName = fromFactory?.name?.toLowerCase() ?? '';
        final toName = toFactory?.name?.toLowerCase() ?? '';
        final detail = flux.transportDetails?.toLowerCase() ?? '';
        return fromName.contains(query) ||
            toName.contains(query) ||
            detail.contains(query);
      }).toList();
    }

    // Filter by transport type
    if (_selectedTransportTypes.isNotEmpty) {
      filtered = filtered.where((entry) {
        final transportType = _getTransportType(entry.key, transportTypes);
        return _selectedTransportTypes.contains(transportType);
      }).toList();
    }

    return filtered;
  }

  Map<String, List<MapEntry<String, dynamic>>> _groupByTransportType(
    List<MapEntry<String, dynamic>> logisticsList,
    Map<String, String> transportTypes,
  ) {
    final Map<String, List<MapEntry<String, dynamic>>> grouped = {};

    for (final entry in logisticsList) {
      final transportType = _getTransportType(entry.key, transportTypes);
      grouped.putIfAbsent(transportType, () => []).add(entry);
    }

    return grouped;
  }

  void _clearFilters() {
    setState(() {
      _searchQuery = '';
      _searchController.clear();
      _selectedTransportTypes.clear();
    });
    // Also reset preferences filters
    ref.read(preferencesProvider.notifier).resetLogisticsFilters();
  }

  void _addLogisticsLine() {
    showLogisticsLineForm(
      context,
      onSuccess: () {
        // Refresh is handled automatically by the provider invalidation
      },
    );
  }

  void _editLogisticsLine(String id) async {
    // Get the existing logistics line data
    final logisticsAsync = await ref.read(logisticsLineByIdProvider(id).future);
    if (!mounted || logisticsAsync == null) return;

    // Cast to dynamic since LogisticsFlux is opaque
    final flux = logisticsAsync as dynamic;

    // Parse the transport type from the string representation
    final transportTypeStr = await ffiGetLogisticsTransportType(flux: flux);
    final transportType = TransportTypeOption.values.firstWhere(
      (t) => t.label == transportTypeStr,
      orElse: () => TransportTypeOption.truck,
    );

    if (!mounted) return;

    showLogisticsLineForm(
      context,
      editingId: id,
      initialData: LogisticsLineFormData(
        fromFactoryId: flux.fromFactory as String?,
        toFactoryId: flux.toFactory as String?,
        transportType: transportType,
        transportDetail: (flux.transportDetails as String?) ?? '',
      ),
      onSuccess: () {
        // Refresh is handled automatically by the provider invalidation
      },
    );
  }

  void _deleteLogisticsLine(String id) async {
    final messenger = ScaffoldMessenger.of(context);
    try {
      await ref.read(deleteLogisticsLineController.notifier).delete(id: id);
      if (!mounted) return;
      messenger.showSnackBar(
        const SnackBar(content: Text('Logistics line deleted')),
      );
    } catch (error) {
      if (!mounted) return;
      messenger.showSnackBar(
        SnackBar(content: Text('Failed to delete: $error')),
      );
    }
  }
}
