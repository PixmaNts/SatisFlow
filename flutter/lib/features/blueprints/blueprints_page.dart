import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../shared/widgets/widgets.dart';
import '../engine/providers/providers.dart';
import '../../src/rust/api.dart';

/// Blueprints page - manage production line templates.
class BlueprintsPage extends ConsumerStatefulWidget {
  const BlueprintsPage({super.key});

  @override
  ConsumerState<BlueprintsPage> createState() => _BlueprintsPageState();
}

class _BlueprintsPageState extends ConsumerState<BlueprintsPage> {
  final _searchController = TextEditingController();
  String _searchQuery = '';
  final Set<String> _selectedFilters = {};
  final _addBlueprintKey = GlobalKey<_AddBlueprintContentState>();
  final _blueprintDetailsKey = GlobalKey<_BlueprintDetailsContentState>();

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final blueprintsAsync = ref.watch(blueprintTemplatesProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Blueprint Library'),
        centerTitle: false,
        actions: [
          IconButton(
            icon: const Icon(Icons.refresh),
            onPressed: () => ref.invalidate(blueprintTemplatesProvider),
            tooltip: 'Refresh',
          ),
        ],
      ),
      body: Column(
        children: [
          // Search and filter bar
          Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              children: [
                // Search bar
                AppSearchBar(
                  controller: _searchController,
                  hintText: 'Search blueprints...',
                  onChanged: (query) {
                    setState(() {
                      _searchQuery = query;
                    });
                  },
                ),
                const SizedBox(height: 12),

                // Filter chips
                FilterChipBar(
                  options: const [
                    FilterChipOption(
                      label: 'All',
                      value: 'all',
                      icon: Icons.grid_view,
                    ),
                    FilterChipOption(
                      label: 'Recent',
                      value: 'recent',
                      icon: Icons.history,
                    ),
                    FilterChipOption(
                      label: 'Favorites',
                      value: 'favorites',
                      icon: Icons.star,
                    ),
                  ],
                  selectedValues: _selectedFilters,
                  onSelectionChanged: (filters) {
                    setState(() {
                      _selectedFilters.clear();
                      _selectedFilters.addAll(filters);
                    });
                  },
                  multiSelect: false,
                  showClearButton: false,
                ),
              ],
            ),
          ),

          // Blueprint grid
          Expanded(
            child: blueprintsAsync.when(
              loading: () => const Center(child: CircularProgressIndicator()),
              error: (error, stack) => Center(
                child: ErrorDisplay(
                  message: 'Failed to load blueprints: $error',
                  onRetry: () => ref.invalidate(blueprintTemplatesProvider),
                ),
              ),
              data: (blueprints) {
                // Filter blueprints based on search query
                final filteredBlueprints = _filterBlueprints(blueprints);

                if (filteredBlueprints.isEmpty) {
                  return _buildEmptyState();
                }

                return _buildBlueprintGrid(filteredBlueprints);
              },
            ),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton.extended(
        onPressed: () => _showAddBlueprintDialog(),
        icon: const Icon(Icons.add),
        label: const Text('Add Blueprint'),
      ),
    );
  }

  /// Filter blueprints based on search query and selected filters.
  Map<String, dynamic> _filterBlueprints(Map<String, dynamic> blueprints) {
    var filtered = blueprints;

    // Apply search filter
    if (_searchQuery.isNotEmpty) {
      filtered = Map.fromEntries(
        filtered.entries.where((entry) {
          // Since ProductionLineBlueprint is opaque, we use the ID for search
          // In a real implementation, you would extract name/description from the blueprint
          final id = entry.key.toLowerCase();
          return id.contains(_searchQuery.toLowerCase());
        }),
      );
    }

    // Apply category filters
    if (_selectedFilters.contains('recent')) {
      // Placeholder: In real implementation, filter by creation date
      // For now, just return the first few items
      final entries = filtered.entries.take(5).toList();
      filtered = Map.fromEntries(entries);
    } else if (_selectedFilters.contains('favorites')) {
      // Placeholder: In real implementation, filter by favorites
      // For now, return empty as we don't have favorites data
      filtered = {};
    }

    return filtered;
  }

  /// Build the empty state widget.
  Widget _buildEmptyState() {
    if (_searchQuery.isNotEmpty || _selectedFilters.isNotEmpty) {
      return EmptyState.search(
        title: 'No blueprints found',
        subtitle: 'Try adjusting your search or filters',
        action: AppButton.text(
          label: 'Clear filters',
          onPressed: () {
            setState(() {
              _searchController.clear();
              _searchQuery = '';
              _selectedFilters.clear();
            });
          },
        ),
      );
    }

    return EmptyState(
      icon: Icons.architecture,
      title: 'No blueprints yet',
      subtitle: 'Create your first blueprint to get started',
      action: AppButton.primary(
        label: 'Create Blueprint',
        icon: Icons.add,
        onPressed: () => _showAddBlueprintDialog(),
      ),
    );
  }

  /// Build the blueprint grid.
  Widget _buildBlueprintGrid(Map<String, dynamic> blueprints) {
    final entries = blueprints.entries.toList();

    return LayoutBuilder(
      builder: (context, constraints) {
        // Calculate number of columns based on available width
        final crossAxisCount = (constraints.maxWidth / 300).floor().clamp(1, 4);

        return GridView.builder(
          padding: const EdgeInsets.fromLTRB(16, 0, 16, 80),
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: crossAxisCount,
            childAspectRatio: 1.2,
            crossAxisSpacing: 16,
            mainAxisSpacing: 16,
          ),
          itemCount: entries.length,
          itemBuilder: (context, index) {
            final entry = entries[index];
            return _BlueprintCard(
              id: entry.key,
              blueprint: entry.value,
              onTap: () => _showBlueprintDetails(entry.key, entry.value),
              onDelete: () => _confirmDeleteBlueprint(entry.key),
            );
          },
        );
      },
    );
  }

  /// Show add blueprint dialog.
  void _showAddBlueprintDialog() {
    AppModal.showBottomSheet(
      context: context,
      title: 'Add Blueprint',
      actions: [
        AppButton.text(
          label: 'Cancel',
          onPressed: () => Navigator.pop(context),
        ),
        AppButton.primary(
          label: 'Add',
          onPressed: () async {
            final json = _addBlueprintKey.currentState?.getBlueprintJson();
            if (json == null) {
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(
                  content: Text('Please enter a blueprint name'),
                  duration: Duration(seconds: 2),
                ),
              );
              return;
            }
            Navigator.pop(context);
            // Show loading indicator
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('Creating blueprint...'),
                duration: Duration(seconds: 1),
              ),
            );
            try {
              final blueprint = await ffiBlueprintFromJson(json: json);
              final engine = await ref.read(engineProvider.future);
              await ffiAddBlueprintTemplate(
                engine: engine,
                blueprint: blueprint,
              );
              ref.invalidate(blueprintTemplatesProvider);
              if (mounted) {
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(
                    content: Text('Blueprint created successfully'),
                    duration: Duration(seconds: 2),
                  ),
                );
              }
            } catch (e) {
              if (mounted) {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content: Text('Failed to create blueprint: $e'),
                    duration: const Duration(seconds: 3),
                  ),
                );
              }
            }
          },
        ),
      ],
      child: _AddBlueprintContent(key: _addBlueprintKey),
    );
  }

  /// Show blueprint details/edit dialog.
  void _showBlueprintDetails(String id, dynamic blueprint) {
    AppModal.showBottomSheet(
      context: context,
      title: 'Blueprint Details',
      actions: [
        AppButton.text(
          label: 'Cancel',
          onPressed: () => Navigator.pop(context),
        ),
        AppButton.primary(
          label: 'Update',
          onPressed: () async {
            final data = _blueprintDetailsKey.currentState?.getUpdatedData();
            if (data == null) {
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(
                  content: Text('Please enter a blueprint name'),
                  duration: Duration(seconds: 2),
                ),
              );
              return;
            }
            Navigator.pop(context);
            // Show loading indicator
            ScaffoldMessenger.of(context).showSnackBar(
              const SnackBar(
                content: Text('Updating blueprint...'),
                duration: Duration(seconds: 1),
              ),
            );
            try {
              final engine = await ref.read(engineProvider.future);
              await ffiUpdateBlueprintTemplate(
                engine: engine,
                id: id,
                name: data['name']!,
                description: data['description']!,
              );
              ref.invalidate(blueprintTemplatesProvider);
              if (mounted) {
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(
                    content: Text('Blueprint updated successfully'),
                    duration: Duration(seconds: 2),
                  ),
                );
              }
            } catch (e) {
              if (mounted) {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content: Text('Failed to update blueprint: $e'),
                    duration: const Duration(seconds: 3),
                  ),
                );
              }
            }
          },
        ),
      ],
      child: _BlueprintDetailsContent(
        key: _blueprintDetailsKey,
        id: id,
        blueprint: blueprint,
      ),
    );
  }

  /// Confirm and delete blueprint.
  Future<void> _confirmDeleteBlueprint(String id) async {
    final confirmed = await ConfirmDialog.showDelete(
      context: context,
      title: 'Delete Blueprint',
      message:
          'Are you sure you want to delete this blueprint? This action cannot be undone.',
    );

    if (confirmed == true && mounted) {
      final controller = ref.read(removeBlueprintTemplateController.notifier);
      await controller.remove(id: id);

      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Blueprint deleted'),
            duration: Duration(seconds: 2),
          ),
        );
      }
    }
  }
}

/// Blueprint card widget.
class _BlueprintCard extends StatelessWidget {
  final String id;
  final dynamic blueprint;
  final VoidCallback onTap;
  final VoidCallback onDelete;

  const _BlueprintCard({
    required this.id,
    required this.blueprint,
    required this.onTap,
    required this.onDelete,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    // Line count unavailable since ProductionLineBlueprint is opaque
    final shortId = id.length > 8 ? id.substring(0, 8) : id;

    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: BorderSide(color: colorScheme.outlineVariant, width: 1),
      ),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(12),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header with icon and menu
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
                            color: colorScheme.onSurface,
                          ),
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
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
                  PopupMenuButton<String>(
                    icon: Icon(
                      Icons.more_vert,
                      color: colorScheme.onSurfaceVariant,
                    ),
                    onSelected: (value) {
                      if (value == 'delete') {
                        onDelete();
                      }
                    },
                    itemBuilder: (context) => [
                      PopupMenuItem(
                        value: 'delete',
                        child: Row(
                          children: [
                            Icon(
                              Icons.delete_outline,
                              size: 18,
                              color: colorScheme.error,
                            ),
                            const SizedBox(width: 8),
                            Text(
                              'Delete',
                              style: TextStyle(color: colorScheme.error),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                ],
              ),
              const SizedBox(height: 16),

              // ProductionLineBlueprint is opaque via FRB — fields like description and line count are inaccessible
              Expanded(
                child: Text(
                  'Production line template. Details unavailable (opaque type).',
                  style: theme.textTheme.bodyMedium?.copyWith(
                    color: colorScheme.onSurfaceVariant,
                  ),
                  maxLines: 3,
                  overflow: TextOverflow.ellipsis,
                ),
              ),
              const SizedBox(height: 12),

              // Footer with stats
              Row(
                children: [
                  _StatChip(
                    icon: Icons.linear_scale,
                    label: '—',
                    color: colorScheme.secondary,
                  ),
                  const SizedBox(width: 8),
                  _StatChip(
                    icon: Icons.access_time,
                    label: 'Recent',
                    color: colorScheme.tertiary,
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}

/// Stat chip widget for blueprint card.
class _StatChip extends StatelessWidget {
  final IconData icon;
  final String label;
  final Color color;

  const _StatChip({
    required this.icon,
    required this.label,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: color.withValues(alpha: 0.1),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: 14, color: color),
          const SizedBox(width: 4),
          Text(
            label,
            style: theme.textTheme.labelSmall?.copyWith(
              color: color,
              fontWeight: FontWeight.w500,
            ),
          ),
        ],
      ),
    );
  }
}

/// Add blueprint content widget.
class _AddBlueprintContent extends StatefulWidget {
  const _AddBlueprintContent({super.key});

  @override
  State<_AddBlueprintContent> createState() => _AddBlueprintContentState();
}

class _AddBlueprintContentState extends State<_AddBlueprintContent> {
  final _nameController = TextEditingController();
  final _descriptionController = TextEditingController();

  /// Returns a JSON string representing a simple blueprint with one production line.
  String? getBlueprintJson() {
    final name = _nameController.text.trim();
    if (name.isEmpty) return null;
    final description = _descriptionController.text.trim();
    // Create a simple blueprint with one Iron Ingot production line
    final blueprintJson = {
      'name': name,
      'description': description.isNotEmpty ? description : null,
      'production_lines': [
        {
          'name': 'Iron Ingot Production',
          'description': 'Basic iron ingot production',
          'recipe': 'IronIngot',
          'machine_groups': [
            {'number_of_machine': 1, 'oc_value': 100.0, 'somersloop': 0},
          ],
        },
      ],
    };
    return jsonEncode(blueprintJson);
  }

  @override
  void dispose() {
    _nameController.dispose();
    _descriptionController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        TextField(
          controller: _nameController,
          decoration: const InputDecoration(
            labelText: 'Blueprint Name',
            hintText: 'Enter blueprint name',
            border: OutlineInputBorder(),
          ),
          textInputAction: TextInputAction.next,
        ),
        const SizedBox(height: 16),
        TextField(
          controller: _descriptionController,
          decoration: const InputDecoration(
            labelText: 'Description',
            hintText: 'Enter blueprint description',
            border: OutlineInputBorder(),
          ),
          maxLines: 3,
          textInputAction: TextInputAction.done,
        ),
        const SizedBox(height: 16),
        Text(
          'A default Iron Ingot production line will be added. You can edit the blueprint later.',
          style: theme.textTheme.bodySmall?.copyWith(
            color: colorScheme.onSurfaceVariant,
          ),
        ),
      ],
    );
  }
}

/// Blueprint details content widget.
class _BlueprintDetailsContent extends StatefulWidget {
  final String id;
  final dynamic blueprint;

  const _BlueprintDetailsContent({
    super.key,
    required this.id,
    required this.blueprint,
  });

  @override
  State<_BlueprintDetailsContent> createState() =>
      _BlueprintDetailsContentState();
}

class _BlueprintDetailsContentState extends State<_BlueprintDetailsContent> {
  late TextEditingController _nameController;
  late TextEditingController _descriptionController;

  /// Returns updated name and description, or null if name is empty.
  Map<String, String>? getUpdatedData() {
    final name = _nameController.text.trim();
    if (name.isEmpty) return null;
    return {'name': name, 'description': _descriptionController.text.trim()};
  }

  @override
  void initState() {
    super.initState();
    // Placeholder values since ProductionLineBlueprint is opaque
    final shortId = widget.id.length > 8
        ? widget.id.substring(0, 8)
        : widget.id;
    _nameController = TextEditingController(text: 'Blueprint $shortId');
    _descriptionController = TextEditingController(
      text: 'Production line template with multiple lines.',
    );
  }

  @override
  void dispose() {
    _nameController.dispose();
    _descriptionController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Blueprint ID
        Container(
          padding: const EdgeInsets.all(12),
          decoration: BoxDecoration(
            color: colorScheme.surfaceContainerHighest.withValues(alpha: 0.3),
            borderRadius: BorderRadius.circular(8),
          ),
          child: Row(
            children: [
              Icon(
                Icons.fingerprint,
                size: 20,
                color: colorScheme.onSurfaceVariant,
              ),
              const SizedBox(width: 8),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      'Blueprint ID',
                      style: theme.textTheme.labelSmall?.copyWith(
                        color: colorScheme.onSurfaceVariant,
                      ),
                    ),
                    Text(
                      widget.id,
                      style: theme.textTheme.bodyMedium?.copyWith(
                        color: colorScheme.onSurface,
                        fontFamily: 'monospace',
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
        const SizedBox(height: 24),

        // Editable fields
        TextField(
          controller: _nameController,
          decoration: const InputDecoration(
            labelText: 'Blueprint Name',
            border: OutlineInputBorder(),
          ),
          textInputAction: TextInputAction.next,
        ),
        const SizedBox(height: 16),
        TextField(
          controller: _descriptionController,
          decoration: const InputDecoration(
            labelText: 'Description',
            border: OutlineInputBorder(),
          ),
          maxLines: 3,
          textInputAction: TextInputAction.done,
        ),
        const SizedBox(height: 16),
        Text(
          'Note: Only name and description can be updated. To modify production lines, please delete and recreate the blueprint.',
          style: theme.textTheme.bodySmall?.copyWith(
            color: colorScheme.onSurfaceVariant,
          ),
        ),
      ],
    );
  }
}
