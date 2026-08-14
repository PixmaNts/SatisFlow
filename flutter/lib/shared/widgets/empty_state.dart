import 'package:flutter/material.dart';

/// An empty state placeholder widget with icon and message.
///
/// Uses Theme.of(context) for all colors - no hardcoded values.
class EmptyState extends StatelessWidget {
  /// The icon to display.
  final IconData icon;

  /// The title message.
  final String title;

  /// Optional subtitle/description.
  final String? subtitle;

  /// Optional action button.
  final Widget? action;

  /// The size of the icon. Defaults to 64.
  final double iconSize;

  /// The color of the icon. If null, uses theme color.
  final Color? iconColor;

  /// Creates an EmptyState.
  const EmptyState({
    super.key,
    required this.icon,
    required this.title,
    this.subtitle,
    this.action,
    this.iconSize = 64,
    this.iconColor,
  });

  /// Creates an EmptyState with a search icon.
  const EmptyState.search({
    super.key,
    this.title = 'No results found',
    this.subtitle,
    this.action,
    this.iconSize = 64,
    this.iconColor,
  }) : icon = Icons.search_off;

  /// Creates an EmptyState with an inbox icon.
  const EmptyState.inbox({
    super.key,
    this.title = 'No items yet',
    this.subtitle,
    this.action,
    this.iconSize = 64,
    this.iconColor,
  }) : icon = Icons.inbox_outlined;

  /// Creates an EmptyState with a folder icon.
  const EmptyState.folder({
    super.key,
    this.title = 'No files',
    this.subtitle,
    this.action,
    this.iconSize = 64,
    this.iconColor,
  }) : icon = Icons.folder_outlined;

  /// Creates an EmptyState with a network error icon.
  const EmptyState.networkError({
    super.key,
    this.title = 'No connection',
    this.subtitle = 'Check your internet connection and try again',
    this.action,
    this.iconSize = 64,
    this.iconColor,
  }) : icon = Icons.wifi_off_outlined;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Center(
      child: Padding(
        padding: const EdgeInsets.all(32),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(
              icon,
              size: iconSize,
              color: iconColor ?? colorScheme.onSurfaceVariant,
            ),
            const SizedBox(height: 16),
            Text(
              title,
              style: theme.textTheme.headlineSmall?.copyWith(
                color: colorScheme.onSurface,
              ),
              textAlign: TextAlign.center,
            ),
            if (subtitle != null) ...[
              const SizedBox(height: 8),
              Text(
                subtitle!,
                style: theme.textTheme.bodyMedium?.copyWith(
                  color: colorScheme.onSurfaceVariant,
                ),
                textAlign: TextAlign.center,
              ),
            ],
            if (action != null) ...[const SizedBox(height: 24), action!],
          ],
        ),
      ),
    );
  }
}
