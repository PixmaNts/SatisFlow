import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../shared/widgets/confirm_dialog.dart';
import '../../features/engine/providers/providers.dart';
import '../providers/save_load_provider.dart';

/// Navigation destination descriptor.
class _NavDestination {
  final String label;
  final IconData icon;
  final IconData selectedIcon;
  final String routePath;

  const _NavDestination({
    required this.label,
    required this.icon,
    required this.selectedIcon,
    required this.routePath,
  });
}

/// All top-level navigation destinations.
const List<_NavDestination> _destinations = [
  _NavDestination(
    label: 'Dashboard',
    icon: Icons.dashboard_outlined,
    selectedIcon: Icons.dashboard,
    routePath: '/',
  ),
  _NavDestination(
    label: 'Factory',
    icon: Icons.factory_outlined,
    selectedIcon: Icons.factory,
    routePath: '/factory',
  ),
  _NavDestination(
    label: 'Logistics',
    icon: Icons.local_shipping_outlined,
    selectedIcon: Icons.local_shipping,
    routePath: '/logistics',
  ),
  _NavDestination(
    label: 'Blueprints',
    icon: Icons.architecture_outlined,
    selectedIcon: Icons.architecture,
    routePath: '/blueprints',
  ),
];

/// Width threshold at which we switch from BottomNavigationBar to NavigationRail.
const double _kBreakpoint = 600;

/// Responsive scaffold that shows a [NavigationRail] on wide screens (>600px)
/// and a [BottomNavigationBar] on narrow screens (<=600px).
class ScaffoldWithNav extends ConsumerWidget {
  final StatefulNavigationShell navigationShell;

  const ScaffoldWithNav({super.key, required this.navigationShell});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return LayoutBuilder(
      builder: (context, constraints) {
        final isWide = constraints.maxWidth > _kBreakpoint;
        final selectedIndex = navigationShell.currentIndex;

        if (isWide) {
          return Scaffold(
            appBar: AppBar(
              title: const Text('SatisFlow'),
              actions: [
                _SaveButton(),
                _LoadButton(),
                _ResetButton(),
                const SizedBox(width: 8),
              ],
            ),
            body: Row(
              children: [
                NavigationRail(
                  selectedIndex: selectedIndex,
                  onDestinationSelected: _onTap,
                  extended: constraints.maxWidth >= 900,
                  destinations: _destinations
                      .map(
                        (d) => NavigationRailDestination(
                          icon: Icon(d.icon),
                          selectedIcon: Icon(d.selectedIcon),
                          label: Text(d.label),
                        ),
                      )
                      .toList(),
                ),
                const VerticalDivider(thickness: 1, width: 1),
                Expanded(child: navigationShell),
              ],
            ),
          );
        }

        return Scaffold(
          appBar: AppBar(
            title: const Text('SatisFlow'),
            actions: [
              _SaveButton(),
              _LoadButton(),
              _ResetButton(),
              const SizedBox(width: 8),
            ],
          ),
          body: navigationShell,
          bottomNavigationBar: NavigationBar(
            selectedIndex: selectedIndex,
            onDestinationSelected: _onTap,
            destinations: _destinations
                .map(
                  (d) => NavigationDestination(
                    icon: Icon(d.icon),
                    selectedIcon: Icon(d.selectedIcon),
                    label: d.label,
                  ),
                )
                .toList(),
          ),
        );
      },
    );
  }

  void _onTap(int index) {
    navigationShell.goBranch(
      index,
      initialLocation: index == navigationShell.currentIndex,
    );
  }
}

/// Button that triggers save-to-file.
class _SaveButton extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final saveState = ref.watch(saveToFileController);

    return IconButton(
      icon: saveState.isLoading
          ? const SizedBox(
              width: 20,
              height: 20,
              child: CircularProgressIndicator(strokeWidth: 2),
            )
          : const Icon(Icons.save_outlined),
      tooltip: 'Save to File',
      onPressed: saveState.isLoading
          ? null
          : () async {
              final result = await ref
                  .read(saveToFileController.notifier)
                  .save();

              if (context.mounted) {
                if (result.hasError) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                      content: Text('Save failed: ${result.error}'),
                      backgroundColor: Theme.of(context).colorScheme.error,
                    ),
                  );
                } else if (result.hasValue) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(content: Text('State saved successfully')),
                  );
                }
              }
            },
    );
  }
}

/// Button that triggers load-from-file.
class _LoadButton extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loadState = ref.watch(loadFromFileController);

    return IconButton(
      icon: loadState.isLoading
          ? const SizedBox(
              width: 20,
              height: 20,
              child: CircularProgressIndicator(strokeWidth: 2),
            )
          : const Icon(Icons.folder_open_outlined),
      tooltip: 'Load from File',
      onPressed: loadState.isLoading
          ? null
          : () async {
              final result = await ref
                  .read(loadFromFileController.notifier)
                  .load();

              if (context.mounted) {
                if (result.hasError) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                      content: Text('Load failed: ${result.error}'),
                      backgroundColor: Theme.of(context).colorScheme.error,
                    ),
                  );
                } else if (result.hasValue) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(content: Text('State loaded successfully')),
                  );
                }
              }
            },
    );
  }
}

/// Button that triggers engine reset with confirmation dialog.
class _ResetButton extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final resetState = ref.watch(resetEngineController);

    return IconButton(
      icon: resetState.isLoading
          ? const SizedBox(
              width: 20,
              height: 20,
              child: CircularProgressIndicator(strokeWidth: 2),
            )
          : const Icon(Icons.restart_alt_outlined),
      tooltip: 'Reset Engine',
      onPressed: resetState.isLoading
          ? null
          : () async {
              final confirmed = await ConfirmDialog.showDelete(
                context: context,
                title: 'Reset Engine',
                message:
                    'This will delete all factories, logistics lines, and blueprints. This action cannot be undone.',
                confirmLabel: 'Reset',
              );

              if (confirmed != true || !context.mounted) return;

              final result = await ref
                  .read(resetEngineController.notifier)
                  .reset();

              if (context.mounted) {
                if (result.hasError) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                      content: Text('Reset failed: ${result.error}'),
                      backgroundColor: Theme.of(context).colorScheme.error,
                    ),
                  );
                } else if (result.hasValue) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(content: Text('Engine reset successfully')),
                  );
                }
              }
            },
    );
  }
}
