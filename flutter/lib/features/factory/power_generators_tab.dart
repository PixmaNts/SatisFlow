import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../shared/widgets/empty_state.dart';
import '../engine/providers/providers.dart';

/// Power generator CRUD is intentionally unavailable until the Rust FFI bridge
/// exposes the necessary endpoints.
class PowerGeneratorsTab extends ConsumerWidget {
  /// The currently selected factory (can be null if no factory selected).
  final dynamic factory;

  /// Creates a PowerGeneratorsTab.
  const PowerGeneratorsTab({super.key, this.factory});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    if (factory == null) {
      return const _PowerGeneratorsPlaceholder();
    }

    final currentFactoryId = ref.watch(currentFactoryIdProvider);
    final shortId = _shortFactoryId(currentFactoryId);

    return EmptyState(
      icon: Icons.electric_bolt,
      title: 'Power Generators unavailable',
      subtitle:
          'Factory $shortId is selected, but power generator CRUD is '
          'unavailable because the Rust FFI bridge does not expose power '
          'generator functions yet.',
    );
  }
}

/// Placeholder widget shown when no factory is selected.
class _PowerGeneratorsPlaceholder extends StatelessWidget {
  const _PowerGeneratorsPlaceholder();

  @override
  Widget build(BuildContext context) {
    return const EmptyState(
      icon: Icons.electric_bolt,
      title: 'Power Generators',
      subtitle: 'Select a factory to view power generators',
    );
  }
}

String _shortFactoryId(String? id) {
  if (id == null || id.isEmpty) {
    return 'Unknown';
  }

  return id.length > 8 ? id.substring(0, 8) : id;
}
