import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../shared/widgets/widgets.dart';
import '../../src/rust/api.dart';
import '../engine/providers/providers.dart';

/// Builds the Production Lines tab content.
///
/// Uses shared widgets: AppDataTable, ConfirmDialog, AppSearchBar, FilterChipBar.
/// Key providers: factoriesProvider, currentFactoryIdProvider, blueprintTemplatesProvider.
///
/// Note: Production lines are created by instantiating blueprints into factories.
/// The Factory type is opaque, so we work with blueprint templates and use
/// ffiInstantiateBlueprintIntoFactory to create production lines.
Widget buildProductionLinesTab(WidgetRef ref, dynamic factory) {
  if (factory == null) {
    return const Center(
      child: EmptyState(
        icon: Icons.line_axis,
        title: 'Production Lines',
        subtitle: 'Select a factory to manage production lines',
      ),
    );
  }

  final factoryId = factory.id as String;
  final shortId = factoryId.length > 8 ? factoryId.substring(0, 8) : factoryId;

  return _ProductionLinesTabContent(factoryId: factoryId, shortId: shortId);
}

/// Production Lines tab content with CRUD for blueprint instantiation.
class _ProductionLinesTabContent extends ConsumerStatefulWidget {
  final String factoryId;
  final String shortId;

  const _ProductionLinesTabContent({
    required this.factoryId,
    required this.shortId,
  });

  @override
  ConsumerState<_ProductionLinesTabContent> createState() =>
      _ProductionLinesTabContentState();
}

class _ProductionLinesTabContentState
    extends ConsumerState<_ProductionLinesTabContent> {
  final TextEditingController _searchController = TextEditingController();
  String _searchQuery = '';
  bool _isInstantiating = false;

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final blueprintsAsync = ref.watch(blueprintTemplatesProvider);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Header
        Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text('Production Lines', style: theme.textTheme.titleLarge),
              const SizedBox(height: 4),
              Text(
                'Factory: ${widget.shortId}',
                style: theme.textTheme.bodyMedium?.copyWith(
                  color: theme.colorScheme.onSurfaceVariant,
                ),
              ),
              const SizedBox(height: 8),
              Text(
                'Select a blueprint template to instantiate into this factory.',
                style: theme.textTheme.bodySmall?.copyWith(
                  color: theme.colorScheme.onSurfaceVariant,
                ),
              ),
            ],
          ),
        ),

        // Search bar
        Padding(
          padding: const EdgeInsets.symmetric(horizontal: 16),
          child: AppSearchBar(
            controller: _searchController,
            hintText: 'Search blueprints...',
            onChanged: (query) {
              setState(() {
                _searchQuery = query;
              });
            },
            onClear: () {
              setState(() {
                _searchQuery = '';
              });
            },
          ),
        ),

        const SizedBox(height: 16),

        // Blueprint list
        Expanded(
          child: blueprintsAsync.when(
            loading: () => const Center(child: CircularProgressIndicator()),
            error: (error, _) => ErrorDisplay(
              message: 'Failed to load blueprints: $error',
              onRetry: () => ref.invalidate(blueprintTemplatesProvider),
            ),
            data: (blueprints) {
              final filteredBlueprints = _filterBlueprints(blueprints);

              if (filteredBlueprints.isEmpty) {
                return _buildEmptyState();
              }

              return _buildBlueprintList(filteredBlueprints);
            },
          ),
        ),
      ],
    );
  }

  Map<String, ProductionLineBlueprint> _filterBlueprints(
    Map<String, ProductionLineBlueprint> blueprints,
  ) {
    if (_searchQuery.isEmpty) {
      return blueprints;
    }

    final query = _searchQuery.toLowerCase();
    return Map.fromEntries(
      blueprints.entries.where((entry) {
        final id = entry.key.toLowerCase();
        return id.contains(query);
      }),
    );
  }

  Widget _buildEmptyState() {
    if (_searchQuery.isNotEmpty) {
      return EmptyState.search(
        title: 'No blueprints found',
        subtitle: 'Try adjusting your search',
        action: AppButton.text(
          label: 'Clear search',
          onPressed: () {
            setState(() {
              _searchQuery = '';
              _searchController.clear();
            });
          },
        ),
      );
    }

    return EmptyState(
      icon: Icons.architecture,
      title: 'No blueprints available',
      subtitle: 'Create a blueprint in the Blueprint Library first',
    );
  }

  Widget _buildBlueprintList(Map<String, ProductionLineBlueprint> blueprints) {
    final entries = blueprints.entries.toList();

    return ListView.builder(
      padding: const EdgeInsets.symmetric(horizontal: 16),
      itemCount: entries.length,
      itemBuilder: (context, index) {
        final entry = entries[index];
        return _BlueprintCard(
          id: entry.key,
          blueprint: entry.value,
          onInstantiate: () => _instantiateBlueprint(entry.key),
          isLoading: _isInstantiating,
        );
      },
    );
  }

  Future<void> _instantiateBlueprint(String blueprintId) async {
    if (_isInstantiating) return;

    setState(() {
      _isInstantiating = true;
    });

    final messenger = ScaffoldMessenger.of(context);

    try {
      final engine = await ref.read(engineProvider.future);
      await ffiInstantiateBlueprintIntoFactory(
        engine: engine,
        factoryIdStr: widget.factoryId,
        blueprintIdStr: blueprintId,
      );

      if (!mounted) return;

      ref.invalidate(factoriesProvider);
      ref.invalidate(factoryByIdProvider(widget.factoryId));

      messenger.showSnackBar(
        const SnackBar(
          content: Text(
            'Blueprint instantiation call completed. Refresh the factory data '
            'to verify the created production lines.',
          ),
          duration: Duration(seconds: 2),
        ),
      );
    } on Exception catch (e) {
      if (!mounted) return;
      messenger.showSnackBar(
        SnackBar(
          content: Text('Failed to instantiate blueprint: $e'),
          backgroundColor: Theme.of(context).colorScheme.error,
          duration: const Duration(seconds: 3),
        ),
      );
    } finally {
      if (mounted) {
        setState(() {
          _isInstantiating = false;
        });
      }
    }
  }
}

/// Card widget displaying a blueprint with instantiate action.
class _BlueprintCard extends StatelessWidget {
  final String id;
  final ProductionLineBlueprint blueprint;
  final VoidCallback onInstantiate;
  final bool isLoading;

  const _BlueprintCard({
    required this.id,
    required this.blueprint,
    required this.onInstantiate,
    required this.isLoading,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;
    final shortId = id.length > 8 ? id.substring(0, 8) : id;

    return Card(
      margin: const EdgeInsets.only(bottom: 12),
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
            // Header row with icon and ID
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.all(8),
                  decoration: BoxDecoration(
                    color: colorScheme.primaryContainer,
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Icon(
                    Icons.architecture,
                    size: 20,
                    color: colorScheme.onPrimaryContainer,
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        'Blueprint $shortId',
                        style: theme.textTheme.titleMedium?.copyWith(
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                      Text(
                        'ID: $shortId',
                        style: theme.textTheme.bodySmall?.copyWith(
                          color: colorScheme.onSurfaceVariant,
                        ),
                      ),
                    ],
                  ),
                ),
              ],
            ),

            const SizedBox(height: 16),

            // Description placeholder
            Text(
              'Production line template. Instantiate this blueprint to create production lines in the factory.',
              style: theme.textTheme.bodyMedium?.copyWith(
                color: colorScheme.onSurfaceVariant,
              ),
            ),

            const SizedBox(height: 16),

            // Action button
            Row(
              mainAxisAlignment: MainAxisAlignment.end,
              children: [
                AppButton.primary(
                  label: isLoading ? 'Instantiating...' : 'Instantiate',
                  icon: isLoading ? Icons.hourglass_empty : Icons.play_arrow,
                  onPressed: isLoading ? null : onInstantiate,
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
