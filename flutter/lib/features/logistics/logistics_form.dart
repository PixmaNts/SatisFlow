import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../shared/widgets/widgets.dart';
import '../engine/providers/providers.dart';

/// Transport type options available in the system.
enum TransportTypeOption {
  bus('Bus', Icons.directions_bus),
  train('Train', Icons.train),
  truck('Truck', Icons.local_shipping),
  drone('Drone', Icons.airplanemode_active);

  const TransportTypeOption(this.label, this.icon);

  final String label;
  final IconData icon;
}

/// Data class representing logistics line form values.
class LogisticsLineFormData {
  final String? fromFactoryId;
  final String? toFactoryId;
  final TransportTypeOption? transportType;
  final String transportDetail;

  const LogisticsLineFormData({
    this.fromFactoryId,
    this.toFactoryId,
    this.transportType,
    this.transportDetail = '',
  });

  LogisticsLineFormData copyWith({
    String? fromFactoryId,
    String? toFactoryId,
    TransportTypeOption? transportType,
    String? transportDetail,
  }) {
    return LogisticsLineFormData(
      fromFactoryId: fromFactoryId ?? this.fromFactoryId,
      toFactoryId: toFactoryId ?? this.toFactoryId,
      transportType: transportType ?? this.transportType,
      transportDetail: transportDetail ?? this.transportDetail,
    );
  }

  bool get isValid =>
      fromFactoryId != null &&
      toFactoryId != null &&
      transportType != null &&
      fromFactoryId != toFactoryId;
}

/// Form for creating or editing a logistics line.
class LogisticsLineForm extends ConsumerStatefulWidget {
  /// Existing logistics line ID for editing, null for creating.
  final String? editingId;

  /// Initial data for the form when editing.
  final LogisticsLineFormData? initialData;

  /// Callback when form is submitted successfully.
  final VoidCallback? onSuccess;

  const LogisticsLineForm({
    super.key,
    this.editingId,
    this.initialData,
    this.onSuccess,
  });

  @override
  ConsumerState<LogisticsLineForm> createState() => _LogisticsLineFormState();
}

class _LogisticsLineFormState extends ConsumerState<LogisticsLineForm> {
  late TextEditingController _detailController;
  String? _selectedFromFactory;
  String? _selectedToFactory;
  TransportTypeOption? _selectedTransportType;

  bool get _isEditing => widget.editingId != null;

  @override
  void initState() {
    super.initState();
    _detailController = TextEditingController(
      text: widget.initialData?.transportDetail ?? '',
    );
    _selectedFromFactory = widget.initialData?.fromFactoryId;
    _selectedToFactory = widget.initialData?.toFactoryId;
    _selectedTransportType = widget.initialData?.transportType;
  }

  @override
  void dispose() {
    _detailController.dispose();
    super.dispose();
  }

  String? _getDetailHint() {
    switch (_selectedTransportType) {
      case TransportTypeOption.truck:
        return 'Enter route or path description';
      case TransportTypeOption.train:
        return 'Enter track name';
      case TransportTypeOption.bus:
        return 'Enter direction or conveyor details';
      case TransportTypeOption.drone:
        return 'Enter flight path description';
      case null:
        return null;
    }
  }

  String? _getDetailLabel() {
    switch (_selectedTransportType) {
      case TransportTypeOption.truck:
        return 'Route/Path';
      case TransportTypeOption.train:
        return 'Track Name';
      case TransportTypeOption.bus:
        return 'Direction';
      case TransportTypeOption.drone:
        return 'Flight Path';
      case null:
        return 'Details';
    }
  }

  Future<void> _submit() async {
    if (_selectedFromFactory == null ||
        _selectedToFactory == null ||
        _selectedTransportType == null) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Please fill in all fields')),
      );
      return;
    }

    if (_selectedFromFactory == _selectedToFactory) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Source and destination factories must be different'),
        ),
      );
      return;
    }

    try {
      final result = _isEditing
          ? await ref
                .read(updateLogisticsLineController.notifier)
                .updateLine(
                  id: widget.editingId!,
                  fromId: _selectedFromFactory!,
                  toId: _selectedToFactory!,
                  transportTypeName: _selectedTransportType!.label,
                  transportDetail: _detailController.text,
                )
          : await ref
                .read(createLogisticsLineController.notifier)
                .create(
                  fromId: _selectedFromFactory!,
                  toId: _selectedToFactory!,
                  transportTypeName: _selectedTransportType!.label,
                  transportDetail: _detailController.text,
                );

      if (!mounted) return;

      if (result.hasError) {
        ScaffoldMessenger.of(
          context,
        ).showSnackBar(SnackBar(content: Text('Failed: ${result.error}')));
        return;
      }

      if (!result.hasValue) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text(
              'Logistics update finished without a confirmed result. Please '
              'reload the page to verify the latest data.',
            ),
          ),
        );
        return;
      }

      Navigator.of(context).pop();
      widget.onSuccess?.call();

      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text(
            _isEditing ? 'Logistics line updated' : 'Logistics line created',
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

  @override
  Widget build(BuildContext context) {
    final factoriesAsync = ref.watch(factoriesProvider);

    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        // Source Factory Dropdown
        factoriesAsync.when(
          loading: () => const LinearProgressIndicator(),
          error: (error, _) => Text('Failed to load factories: $error'),
          data: (factories) {
            if (factories.isEmpty) {
              return const Text(
                'No factories available. Create a factory first.',
              );
            }

            final factoryEntries = factories.entries.toList();

            return Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Source Factory
                DropdownButtonFormField<String>(
                  initialValue: _selectedFromFactory,
                  decoration: const InputDecoration(
                    labelText: 'From Factory',
                    hintText: 'Select source factory',
                    border: OutlineInputBorder(),
                    prefixIcon: Icon(Icons.factory_outlined),
                  ),
                  isExpanded: true,
                  items: factoryEntries.map((entry) {
                    // Factory is opaque, use ID as display value
                    final shortId = entry.key.length > 8
                        ? entry.key.substring(0, 8)
                        : entry.key;
                    return DropdownMenuItem(
                      value: entry.key,
                      child: Text(shortId),
                    );
                  }).toList(),
                  onChanged: (value) {
                    setState(() => _selectedFromFactory = value);
                  },
                ),
                const SizedBox(height: 16),

                // Destination Factory
                DropdownButtonFormField<String>(
                  initialValue: _selectedToFactory,
                  decoration: const InputDecoration(
                    labelText: 'To Factory',
                    hintText: 'Select destination factory',
                    border: OutlineInputBorder(),
                    prefixIcon: Icon(Icons.factory_outlined),
                  ),
                  isExpanded: true,
                  items: factoryEntries.map((entry) {
                    // Factory is opaque, use ID as display value
                    final shortId = entry.key.length > 8
                        ? entry.key.substring(0, 8)
                        : entry.key;
                    return DropdownMenuItem(
                      value: entry.key,
                      child: Text(shortId),
                    );
                  }).toList(),
                  onChanged: (value) {
                    setState(() => _selectedToFactory = value);
                  },
                ),
                const SizedBox(height: 16),

                // Transport Type
                DropdownButtonFormField<TransportTypeOption>(
                  initialValue: _selectedTransportType,
                  decoration: InputDecoration(
                    labelText: 'Transport Type',
                    hintText: 'Select transport type',
                    border: const OutlineInputBorder(),
                    prefixIcon: Icon(
                      _selectedTransportType?.icon ?? Icons.commute,
                    ),
                  ),
                  isExpanded: true,
                  items: TransportTypeOption.values.map((option) {
                    return DropdownMenuItem(
                      value: option,
                      child: Row(
                        children: [
                          Icon(option.icon, size: 20),
                          const SizedBox(width: 8),
                          Text(option.label),
                        ],
                      ),
                    );
                  }).toList(),
                  onChanged: (value) {
                    setState(() {
                      _selectedTransportType = value;
                      _detailController.clear();
                    });
                  },
                ),
                const SizedBox(height: 16),

                // Transport Detail (varies by type)
                if (_selectedTransportType != null)
                  TextField(
                    controller: _detailController,
                    decoration: InputDecoration(
                      labelText: _getDetailLabel(),
                      hintText: _getDetailHint(),
                      border: const OutlineInputBorder(),
                      prefixIcon: Icon(_getDetailIcon()),
                    ),
                    maxLines: _selectedTransportType == TransportTypeOption.bus
                        ? 3
                        : 1,
                  ),
              ],
            );
          },
        ),
      ],
    );
  }

  IconData _getDetailIcon() {
    switch (_selectedTransportType) {
      case TransportTypeOption.truck:
        return Icons.route;
      case TransportTypeOption.train:
        return Icons.train;
      case TransportTypeOption.bus:
        return Icons.directions;
      case TransportTypeOption.drone:
        return Icons.flight;
      case null:
        return Icons.info;
    }
  }
}

/// Shows the logistics line form as a bottom sheet.
Future<void> showLogisticsLineForm(
  BuildContext context, {
  String? editingId,
  LogisticsLineFormData? initialData,
  VoidCallback? onSuccess,
}) {
  return AppModal.showBottomSheet(
    context: context,
    title: editingId != null ? 'Edit Logistics Line' : 'Add Logistics Line',
    child: LogisticsLineForm(
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
        label: editingId != null ? 'Update' : 'Create',
        onPressed: () {
          // Find the form state and submit
          final formState = context
              .findAncestorStateOfType<_LogisticsLineFormState>();
          formState?._submit();
        },
      ),
    ],
  );
}
