import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../shared/widgets/widgets.dart';
import '../engine/providers/providers.dart';

/// Builds the Raw Inputs tab content.
///
/// Raw input CRUD is intentionally unavailable until the Rust FFI bridge
/// exposes the necessary endpoints.
Widget buildRawInputsTab(dynamic factory) {
  if (factory == null) {
    return const _NoFactorySelected();
  }

  return const _RawInputsUnavailable();
}

/// Widget shown when no factory is selected.
class _NoFactorySelected extends StatelessWidget {
  const _NoFactorySelected();

  @override
  Widget build(BuildContext context) {
    return const EmptyState(
      icon: Icons.input_outlined,
      title: 'Raw Inputs',
      subtitle: 'Select a factory to view raw inputs',
    );
  }
}

/// Explains why raw inputs cannot be managed from Flutter yet.
class _RawInputsUnavailable extends ConsumerWidget {
  const _RawInputsUnavailable();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final currentFactoryId = ref.watch(currentFactoryIdProvider);
    final shortId = _shortFactoryId(currentFactoryId);

    return EmptyState(
      icon: Icons.input_outlined,
      title: 'Raw Inputs unavailable',
      subtitle:
          'Factory $shortId is selected, but raw input CRUD is unavailable '
          'because the Rust FFI bridge does not expose raw input functions yet.',
    );
  }
}

String _shortFactoryId(String? id) {
  if (id == null || id.isEmpty) {
    return 'Unknown';
  }

  return id.length > 8 ? id.substring(0, 8) : id;
}
