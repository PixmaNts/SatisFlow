import 'package:flutter/material.dart';

/// Button variants for different use cases.
enum AppButtonVariant {
  /// Primary action button (filled, prominent)
  primary,

  /// Secondary action button (outlined)
  secondary,

  /// Danger/destructive action button (red accent)
  danger,

  /// Text-only button (no background)
  text,
}

/// A styled button widget with multiple variants following Material 3 design.
///
/// Uses Theme.of(context) for all colors - no hardcoded values.
class AppButton extends StatelessWidget {
  /// The button label text.
  final String label;

  /// Callback when button is pressed.
  final VoidCallback? onPressed;

  /// Button variant style.
  final AppButtonVariant variant;

  /// Optional icon to display before the label.
  final IconData? icon;

  /// Button size.
  final AppButtonSize size;

  /// Whether the button is in loading state.
  final bool isLoading;

  /// Whether the button takes full width.
  final bool fullWidth;

  /// Creates an AppButton.
  const AppButton({
    super.key,
    required this.label,
    this.onPressed,
    this.variant = AppButtonVariant.primary,
    this.icon,
    this.size = AppButtonSize.medium,
    this.isLoading = false,
    this.fullWidth = false,
  });

  /// Creates a primary button.
  const AppButton.primary({
    super.key,
    required this.label,
    this.onPressed,
    this.icon,
    this.size = AppButtonSize.medium,
    this.isLoading = false,
    this.fullWidth = false,
  }) : variant = AppButtonVariant.primary;

  /// Creates a secondary button.
  const AppButton.secondary({
    super.key,
    required this.label,
    this.onPressed,
    this.icon,
    this.size = AppButtonSize.medium,
    this.isLoading = false,
    this.fullWidth = false,
  }) : variant = AppButtonVariant.secondary;

  /// Creates a danger button.
  const AppButton.danger({
    super.key,
    required this.label,
    this.onPressed,
    this.icon,
    this.size = AppButtonSize.medium,
    this.isLoading = false,
    this.fullWidth = false,
  }) : variant = AppButtonVariant.danger;

  /// Creates a text button.
  const AppButton.text({
    super.key,
    required this.label,
    this.onPressed,
    this.icon,
    this.size = AppButtonSize.medium,
    this.isLoading = false,
    this.fullWidth = false,
  }) : variant = AppButtonVariant.text;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    // Determine colors based on variant
    final (backgroundColor, foregroundColor, borderColor) = _getColors(
      colorScheme,
    );

    // Determine padding based on size
    final padding = _getPadding();
    final textStyle = _getTextStyle(theme);

    // Build the button content
    Widget child = _buildContent(theme, foregroundColor);

    // Wrap in SizedBox if fullWidth
    if (fullWidth) {
      child = SizedBox(width: double.infinity, child: child);
    }

    // Return appropriate button type based on variant
    return switch (variant) {
      AppButtonVariant.primary || AppButtonVariant.danger => FilledButton(
        onPressed: isLoading ? null : onPressed,
        style: FilledButton.styleFrom(
          backgroundColor: backgroundColor,
          foregroundColor: foregroundColor,
          padding: padding,
          textStyle: textStyle,
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
        ),
        child: child,
      ),
      AppButtonVariant.secondary => OutlinedButton(
        onPressed: isLoading ? null : onPressed,
        style: OutlinedButton.styleFrom(
          foregroundColor: foregroundColor,
          side: BorderSide(color: borderColor!),
          padding: padding,
          textStyle: textStyle,
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
        ),
        child: child,
      ),
      AppButtonVariant.text => TextButton(
        onPressed: isLoading ? null : onPressed,
        style: TextButton.styleFrom(
          foregroundColor: foregroundColor,
          padding: padding,
          textStyle: textStyle,
        ),
        child: child,
      ),
    };
  }

  /// Get colors based on button variant.
  (Color? backgroundColor, Color foregroundColor, Color? borderColor)
  _getColors(ColorScheme colorScheme) {
    return switch (variant) {
      AppButtonVariant.primary => (
        colorScheme.primary,
        colorScheme.onPrimary,
        null,
      ),
      AppButtonVariant.secondary => (
        null,
        colorScheme.primary,
        colorScheme.outline,
      ),
      AppButtonVariant.danger => (colorScheme.error, colorScheme.onError, null),
      AppButtonVariant.text => (null, colorScheme.primary, null),
    };
  }

  /// Get padding based on button size.
  EdgeInsetsGeometry _getPadding() {
    return switch (size) {
      AppButtonSize.small => const EdgeInsets.symmetric(
        horizontal: 12,
        vertical: 8,
      ),
      AppButtonSize.medium => const EdgeInsets.symmetric(
        horizontal: 16,
        vertical: 12,
      ),
      AppButtonSize.large => const EdgeInsets.symmetric(
        horizontal: 24,
        vertical: 16,
      ),
    };
  }

  /// Get text style based on button size.
  TextStyle _getTextStyle(ThemeData theme) {
    final baseStyle = theme.textTheme.labelLarge ?? const TextStyle();

    return switch (size) {
      AppButtonSize.small => baseStyle.copyWith(fontSize: 12),
      AppButtonSize.medium => baseStyle,
      AppButtonSize.large => baseStyle.copyWith(fontSize: 16),
    };
  }

  /// Build the button content with optional icon and loading indicator.
  Widget _buildContent(ThemeData theme, Color foregroundColor) {
    if (isLoading) {
      return SizedBox(
        width: _getIconSize(),
        height: _getIconSize(),
        child: CircularProgressIndicator(
          strokeWidth: 2,
          valueColor: AlwaysStoppedAnimation<Color>(foregroundColor),
        ),
      );
    }

    if (icon != null) {
      return Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: _getIconSize()),
          const SizedBox(width: 8),
          Text(label),
        ],
      );
    }

    return Text(label);
  }

  /// Get icon size based on button size.
  double _getIconSize() {
    return switch (size) {
      AppButtonSize.small => 16,
      AppButtonSize.medium => 20,
      AppButtonSize.large => 24,
    };
  }
}

/// Button size variants.
enum AppButtonSize {
  /// Small button
  small,

  /// Medium button (default)
  medium,

  /// Large button
  large,
}
