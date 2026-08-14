import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/providers/save_load_provider.dart';
import '../../src/rust/api.dart';
import '../../shared/widgets/widgets.dart';
import '../engine/providers/providers.dart';

/// Production line type options for blueprint templates.
enum ProductionLineType {
  manufacturing('Manufacturing', Icons.precision_manufacturing),
  smelting('Smelting', Icons.fireplace),
  refining('Refining', Icons.water_drop),
  packaging('Packaging', Icons.inventory_2),
  other('Other', Icons.build);

  const ProductionLineType(this.label, this.icon);

  final String label;
  final IconData icon;
}

/// Data class representing blueprint form values.
class BlueprintFormData {
  final String name;
  final String description;
  final ProductionLineType? productionLineType;

  const BlueprintFormData({
    this.name = '',
    this.description = '',
    this.productionLineType,
  });

  BlueprintFormData copyWith({
    String? name,
    String? description,
    ProductionLineType? productionLineType,
  }) {
    return BlueprintFormData(
      name: name ?? this.name,
      description: description ?? this.description,
      productionLineType: productionLineType ?? this.productionLineType,
    );
  }

  bool get isValid => name.trim().isNotEmpty && productionLineType != null;
}

/// Form for creating or editing a blueprint template.
class BlueprintForm extends ConsumerStatefulWidget {
  /// Existing blueprint ID for editing, null for creating.
  final String? editingId;

  /// Initial data for the form when editing.
  final BlueprintFormData? initialData;

  /// Callback when form is submitted successfully.
  final VoidCallback? onSuccess;

  const BlueprintForm({
    super.key,
    this.editingId,
    this.initialData,
    this.onSuccess,
  });

  @override
  ConsumerState<BlueprintForm> createState() => _BlueprintFormState();
}

class _BlueprintFormState extends ConsumerState<BlueprintForm> {
  late TextEditingController _nameController;
  late TextEditingController _descriptionController;
  ProductionLineType? _selectedType;
  final _formKey = GlobalKey<FormState>();

  bool get _isEditing => widget.editingId != null;

  @override
  void initState() {
    super.initState();
    _nameController = TextEditingController(
      text: widget.initialData?.name ?? '',
    );
    _descriptionController = TextEditingController(
      text: widget.initialData?.description ?? '',
    );
    _selectedType = widget.initialData?.productionLineType;
  }

  @override
  void dispose() {
    _nameController.dispose();
    _descriptionController.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    if (!_formKey.currentState!.validate()) {
      return;
    }

    final name = _nameController.text.trim();
    final description = _descriptionController.text.trim();

    if (!_isEditing && _selectedType == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please select a production line type')),
      );
      return;
    }

    try {
      if (_isEditing) {
        final editingId = widget.editingId;
        if (editingId == null) {
          return;
        }

        final engine = await ref.read(engineProvider.future);
        await ffiUpdateBlueprintTemplate(
          engine: engine,
          id: editingId,
          name: name,
          description: description,
        );
        ref.invalidate(blueprintTemplatesProvider);

        if (!mounted) return;

        Navigator.of(context).pop();
        widget.onSuccess?.call();
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Blueprint updated successfully')),
        );
        return;
      }

      if (!mounted) return;

      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text(
            'Blueprint creation from this form is unavailable. Use Import from '
            'JSON until the Rust FFI bridge exposes structured blueprint '
            'creation.',
          ),
        ),
      );
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Failed: $e')));
    }
  }

  Future<void> _importBlueprint() async {
    try {
      final result = await ref
          .read(importBlueprintController.notifier)
          .import();

      result.when(
        data: (_) {
          if (!mounted) return;
          Navigator.of(context).pop();
          widget.onSuccess?.call();
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('Blueprint imported successfully')),
          );
        },
        loading: () {},
        error: (error, _) {
          if (!mounted) return;
          ScaffoldMessenger.of(
            context,
          ).showSnackBar(SnackBar(content: Text('Import failed: $error')));
        },
      );
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Import failed: $e')));
    }
  }

  Future<void> _exportBlueprint() async {
    if (widget.editingId == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Cannot export: No blueprint selected for editing'),
        ),
      );
      return;
    }

    try {
      final result = await ref
          .read(exportBlueprintController.notifier)
          .export(blueprintId: widget.editingId!);

      result.when(
        data: (_) {
          if (!mounted) return;
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('Blueprint exported successfully')),
          );
        },
        loading: () {},
        error: (error, _) {
          if (!mounted) return;
          ScaffoldMessenger.of(
            context,
          ).showSnackBar(SnackBar(content: Text('Export failed: $error')));
        },
      );
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('Export failed: $e')));
    }
  }

  @override
  Widget build(BuildContext context) {
    final colorScheme = Theme.of(context).colorScheme;

    return Form(
      key: _formKey,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          // Name field
          TextFormField(
            controller: _nameController,
            decoration: const InputDecoration(
              labelText: 'Name *',
              hintText: 'Enter blueprint name',
              border: OutlineInputBorder(),
              prefixIcon: Icon(Icons.label_outline),
            ),
            validator: (value) {
              if (value == null || value.trim().isEmpty) {
                return 'Name is required';
              }
              return null;
            },
          ),
          const SizedBox(height: 16),

          // Description field
          TextFormField(
            controller: _descriptionController,
            decoration: const InputDecoration(
              labelText: 'Description',
              hintText: 'Enter blueprint description (optional)',
              border: OutlineInputBorder(),
              prefixIcon: Icon(Icons.description_outlined),
            ),
            maxLines: 3,
          ),
          const SizedBox(height: 16),

          // Production Line Type dropdown
          DropdownButtonFormField<ProductionLineType>(
            initialValue: _selectedType,
            decoration: InputDecoration(
              labelText: 'Production Line Type *',
              hintText: 'Select type',
              border: const OutlineInputBorder(),
              prefixIcon: Icon(_selectedType?.icon ?? Icons.category_outlined),
            ),
            isExpanded: true,
            validator: (value) {
              if (!_isEditing && value == null) {
                return 'Production line type is required';
              }
              return null;
            },
            items: ProductionLineType.values.map((type) {
              return DropdownMenuItem(
                value: type,
                child: Row(
                  children: [
                    Icon(type.icon, size: 20),
                    const SizedBox(width: 8),
                    Text(type.label),
                  ],
                ),
              );
            }).toList(),
            onChanged: _isEditing
                ? null
                : (value) {
                    setState(() => _selectedType = value);
                  },
          ),
          const SizedBox(height: 24),

          if (!_isEditing) ...[
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: colorScheme.surfaceContainerHighest.withValues(
                  alpha: 0.35,
                ),
                borderRadius: BorderRadius.circular(8),
                border: Border.all(color: colorScheme.outlineVariant),
              ),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Icon(
                    Icons.info_outline,
                    size: 20,
                    color: colorScheme.onSurfaceVariant,
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Text(
                      'Structured blueprint creation is not available from this '
                      'form yet. Import a blueprint JSON file instead until the '
                      'Rust FFI bridge exposes blueprint creation APIs.',
                      style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                        color: colorScheme.onSurfaceVariant,
                      ),
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 24),
          ],

          // Import/Export actions
          if (!_isEditing) ...[
            const Divider(),
            const SizedBox(height: 8),
            Text(
              'Or import from file',
              style: Theme.of(context).textTheme.titleSmall,
            ),
            const SizedBox(height: 8),
            AppButton.secondary(
              label: 'Import from JSON',
              icon: Icons.upload_file,
              onPressed: _importBlueprint,
            ),
          ],

          if (_isEditing) ...[
            const Divider(),
            const SizedBox(height: 8),
            Text(
              'Export options',
              style: Theme.of(context).textTheme.titleSmall,
            ),
            const SizedBox(height: 8),
            AppButton.secondary(
              label: 'Export to JSON',
              icon: Icons.download,
              onPressed: _exportBlueprint,
            ),
          ],
        ],
      ),
    );
  }
}

/// Shows the blueprint form as a bottom sheet.
Future<void> showBlueprintForm(
  BuildContext context, {
  String? editingId,
  BlueprintFormData? initialData,
  VoidCallback? onSuccess,
}) {
  final formKey = GlobalKey<_BlueprintFormState>();

  return AppModal.showBottomSheet(
    context: context,
    title: editingId != null ? 'Edit Blueprint' : 'Create Blueprint',
    child: BlueprintForm(
      key: formKey,
      editingId: editingId,
      initialData: initialData,
      onSuccess: onSuccess,
    ),
    actions: [
      AppButton.secondary(
        label: 'Cancel',
        onPressed: () => Navigator.of(context).pop(),
      ),
      AppButton.primary(
        label: editingId != null ? 'Save' : 'Create',
        onPressed: () => formKey.currentState?._submit(),
      ),
    ],
  );
}

/// Shows a dialog to confirm blueprint deletion.
Future<bool> showBlueprintDeleteConfirmation(
  BuildContext context, {
  required String blueprintName,
}) async {
  final result = await showDialog<bool>(
    context: context,
    builder: (context) => AlertDialog(
      title: const Text('Delete Blueprint'),
      content: Text('Are you sure you want to delete "$blueprintName"?'),
      actions: [
        AppButton.text(
          label: 'Cancel',
          onPressed: () => Navigator.of(context).pop(false),
        ),
        AppButton.danger(
          label: 'Delete',
          onPressed: () => Navigator.of(context).pop(true),
        ),
      ],
    ),
  );

  return result ?? false;
}
