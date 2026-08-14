import 'package:flutter/material.dart';

/// A horizontal bar of filter chips.
///
/// Uses Theme.of(context) for all colors - no hardcoded values.
class FilterChipBar extends StatelessWidget {
  /// The list of filter options.
  final List<FilterChipOption> options;

  /// The currently selected filter values.
  final Set<String> selectedValues;

  /// Callback when selection changes.
  final ValueChanged<Set<String>>? onSelectionChanged;

  /// Whether multiple selections are allowed.
  final bool multiSelect;

  /// Whether to show a "clear all" button when filters are active.
  final bool showClearButton;

  /// Callback when clear button is pressed.
  final VoidCallback? onClear;

  /// Spacing between chips.
  final double spacing;

  /// The scroll direction.
  final Axis scrollDirection;

  /// Creates a FilterChipBar.
  const FilterChipBar({
    super.key,
    required this.options,
    required this.selectedValues,
    this.onSelectionChanged,
    this.multiSelect = true,
    this.showClearButton = true,
    this.onClear,
    this.spacing = 8,
    this.scrollDirection = Axis.horizontal,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Row(
      children: [
        Expanded(
          child: SingleChildScrollView(
            scrollDirection: scrollDirection,
            child: Wrap(
              spacing: spacing,
              runSpacing: spacing / 2,
              children: options.map((option) {
                final isSelected = selectedValues.contains(option.value);
                return FilterChip(
                  label: Text(option.label),
                  avatar: option.icon != null
                      ? Icon(
                          option.icon,
                          size: 18,
                          color: isSelected
                              ? colorScheme.onPrimaryContainer
                              : colorScheme.onSurfaceVariant,
                        )
                      : null,
                  selected: isSelected,
                  onSelected: (selected) =>
                      _handleSelection(option.value, selected),
                  selectedColor: colorScheme.primaryContainer,
                  checkmarkColor: colorScheme.onPrimaryContainer,
                  backgroundColor: colorScheme.surfaceContainerHighest,
                  labelStyle: TextStyle(
                    color: isSelected
                        ? colorScheme.onPrimaryContainer
                        : colorScheme.onSurfaceVariant,
                  ),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(8),
                  ),
                  side: BorderSide(
                    color: isSelected
                        ? colorScheme.primary
                        : colorScheme.outline.withValues(alpha: 0.5),
                  ),
                );
              }).toList(),
            ),
          ),
        ),
        if (showClearButton && selectedValues.isNotEmpty) ...[
          const SizedBox(width: 8),
          IconButton(
            icon: const Icon(Icons.filter_list_off),
            onPressed: onClear,
            tooltip: 'Clear filters',
            iconSize: 20,
          ),
        ],
      ],
    );
  }

  void _handleSelection(String value, bool selected) {
    if (onSelectionChanged == null) return;

    final newSelection = Set<String>.from(selectedValues);

    if (multiSelect) {
      if (selected) {
        newSelection.add(value);
      } else {
        newSelection.remove(value);
      }
    } else {
      if (selected) {
        newSelection.clear();
        newSelection.add(value);
      } else {
        newSelection.remove(value);
      }
    }

    onSelectionChanged!(newSelection);
  }
}

/// A filter chip option.
class FilterChipOption {
  /// The display label.
  final String label;

  /// The unique value.
  final String value;

  /// Optional icon.
  final IconData? icon;

  /// Creates a FilterChipOption.
  const FilterChipOption({required this.label, required this.value, this.icon});
}
