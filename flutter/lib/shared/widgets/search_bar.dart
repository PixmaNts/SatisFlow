import 'package:flutter/material.dart';

/// A search/filter input widget with clear button.
///
/// Uses Theme.of(context) for all colors - no hardcoded values.
class AppSearchBar extends StatefulWidget {
  /// The search hint text.
  final String hintText;

  /// Callback when search text changes.
  final ValueChanged<String>? onChanged;

  /// Callback when search is submitted.
  final ValueChanged<String>? onSubmitted;

  /// Callback when clear button is pressed.
  final VoidCallback? onClear;

  /// Initial search text.
  final String initialText;

  /// Whether to show the clear button.
  final bool showClearButton;

  /// Whether the search bar is enabled.
  final bool enabled;

  /// Focus node for the search field.
  final FocusNode? focusNode;

  /// Text editing controller.
  final TextEditingController? controller;

  /// Optional leading icon.
  final IconData? leadingIcon;

  /// Optional trailing widget.
  final Widget? trailing;

  /// Creates an AppSearchBar.
  const AppSearchBar({
    super.key,
    this.hintText = 'Search...',
    this.onChanged,
    this.onSubmitted,
    this.onClear,
    this.initialText = '',
    this.showClearButton = true,
    this.enabled = true,
    this.focusNode,
    this.controller,
    this.leadingIcon,
    this.trailing,
  });

  @override
  State<AppSearchBar> createState() => _AppSearchBarState();
}

class _AppSearchBarState extends State<AppSearchBar> {
  late TextEditingController _controller;
  late FocusNode _focusNode;
  bool _hasText = false;

  @override
  void initState() {
    super.initState();
    _controller =
        widget.controller ?? TextEditingController(text: widget.initialText);
    _focusNode = widget.focusNode ?? FocusNode();
    _hasText = _controller.text.isNotEmpty;
    _controller.addListener(_onTextChanged);
  }

  @override
  void dispose() {
    if (widget.controller == null) {
      _controller.dispose();
    }
    if (widget.focusNode == null) {
      _focusNode.dispose();
    }
    super.dispose();
  }

  void _onTextChanged() {
    final hasText = _controller.text.isNotEmpty;
    if (hasText != _hasText) {
      setState(() {
        _hasText = hasText;
      });
    }
    widget.onChanged?.call(_controller.text);
  }

  void _handleClear() {
    _controller.clear();
    widget.onClear?.call();
    _focusNode.requestFocus();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return TextField(
      controller: _controller,
      focusNode: _focusNode,
      enabled: widget.enabled,
      onSubmitted: widget.onSubmitted,
      decoration: InputDecoration(
        hintText: widget.hintText,
        hintStyle: TextStyle(color: colorScheme.onSurfaceVariant),
        prefixIcon: widget.leadingIcon != null
            ? Icon(widget.leadingIcon, color: colorScheme.onSurfaceVariant)
            : const Icon(Icons.search),
        suffixIcon: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (widget.showClearButton && _hasText)
              IconButton(
                icon: const Icon(Icons.clear),
                onPressed: _handleClear,
                tooltip: 'Clear',
              ),
            if (widget.trailing != null) widget.trailing!,
          ],
        ),
        filled: true,
        fillColor: colorScheme.surfaceContainerHighest.withValues(alpha: 0.4),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(28),
          borderSide: BorderSide.none,
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(28),
          borderSide: BorderSide.none,
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(28),
          borderSide: BorderSide(color: colorScheme.primary, width: 2),
        ),
        contentPadding: const EdgeInsets.symmetric(
          horizontal: 16,
          vertical: 12,
        ),
      ),
      style: theme.textTheme.bodyLarge,
    );
  }
}
