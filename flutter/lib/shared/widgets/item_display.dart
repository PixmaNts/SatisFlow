import 'package:flutter/material.dart';

/// A widget that displays an item with its icon and name.
///
/// Uses Theme.of(context) for all colors - no hardcoded values.
class ItemDisplay extends StatelessWidget {
  /// The item name to display.
  final String name;

  /// The icon to display.
  final IconData icon;

  /// Optional icon color. If null, uses theme color.
  final Color? iconColor;

  /// Optional icon size. Defaults to 24.
  final double iconSize;

  /// Text style for the name. If null, uses theme bodyLarge.
  final TextStyle? textStyle;

  /// Spacing between icon and text. Defaults to 8.
  final double spacing;

  /// Main axis alignment.
  final MainAxisAlignment mainAxisAlignment;

  /// Cross axis alignment.
  final CrossAxisAlignment crossAxisAlignment;

  /// Whether to wrap in a Card widget.
  final bool inCard;

  /// Optional callback when tapped.
  final VoidCallback? onTap;

  /// Creates an ItemDisplay.
  const ItemDisplay({
    super.key,
    required this.name,
    required this.icon,
    this.iconColor,
    this.iconSize = 24,
    this.textStyle,
    this.spacing = 8,
    this.mainAxisAlignment = MainAxisAlignment.start,
    this.crossAxisAlignment = CrossAxisAlignment.center,
    this.inCard = false,
    this.onTap,
  });

  /// Creates a compact ItemDisplay with smaller icon and text.
  const ItemDisplay.compact({
    super.key,
    required this.name,
    required this.icon,
    this.iconColor,
    this.iconSize = 16,
    this.textStyle,
    this.spacing = 4,
    this.mainAxisAlignment = MainAxisAlignment.start,
    this.crossAxisAlignment = CrossAxisAlignment.center,
    this.inCard = false,
    this.onTap,
  });

  /// Creates a large ItemDisplay with bigger icon and text.
  const ItemDisplay.large({
    super.key,
    required this.name,
    required this.icon,
    this.iconColor,
    this.iconSize = 32,
    this.textStyle,
    this.spacing = 12,
    this.mainAxisAlignment = MainAxisAlignment.start,
    this.crossAxisAlignment = CrossAxisAlignment.center,
    this.inCard = false,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    final content = Row(
      mainAxisAlignment: mainAxisAlignment,
      crossAxisAlignment: crossAxisAlignment,
      mainAxisSize: MainAxisSize.min,
      children: [
        Icon(icon, size: iconSize, color: iconColor ?? colorScheme.primary),
        SizedBox(width: spacing),
        Flexible(
          child: Text(
            name,
            style: textStyle ?? theme.textTheme.bodyLarge,
            overflow: TextOverflow.ellipsis,
          ),
        ),
      ],
    );

    if (inCard) {
      return Card(
        child: Padding(padding: const EdgeInsets.all(12), child: content),
      );
    }

    if (onTap != null) {
      return InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(8),
        child: Padding(padding: const EdgeInsets.all(8), child: content),
      );
    }

    return content;
  }
}
