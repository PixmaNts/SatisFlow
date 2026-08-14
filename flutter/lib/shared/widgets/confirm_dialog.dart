import 'package:flutter/material.dart';

/// A confirmation dialog for delete/action confirmations.
///
/// Uses Theme.of(context) for all colors - no hardcoded values.
class ConfirmDialog extends StatelessWidget {
  /// The dialog title.
  final String title;

  /// The dialog message/body.
  final String message;

  /// The confirm button label.
  final String confirmLabel;

  /// The cancel button label.
  final String cancelLabel;

  /// The confirm button variant.
  final ConfirmDialogVariant variant;

  /// Optional icon to display.
  final IconData? icon;

  /// Whether the confirm button is loading.
  final bool isLoading;

  /// Creates a ConfirmDialog.
  const ConfirmDialog({
    super.key,
    required this.title,
    required this.message,
    this.confirmLabel = 'Confirm',
    this.cancelLabel = 'Cancel',
    this.variant = ConfirmDialogVariant.primary,
    this.icon,
    this.isLoading = false,
  });

  /// Show a confirmation dialog.
  static Future<bool?> show({
    required BuildContext context,
    required String title,
    required String message,
    String confirmLabel = 'Confirm',
    String cancelLabel = 'Cancel',
    ConfirmDialogVariant variant = ConfirmDialogVariant.primary,
    IconData? icon,
    bool barrierDismissible = true,
  }) {
    return showDialog<bool>(
      context: context,
      barrierDismissible: barrierDismissible,
      builder: (context) => ConfirmDialog(
        title: title,
        message: message,
        confirmLabel: confirmLabel,
        cancelLabel: cancelLabel,
        variant: variant,
        icon: icon,
      ),
    );
  }

  /// Show a delete confirmation dialog.
  static Future<bool?> showDelete({
    required BuildContext context,
    required String title,
    required String message,
    String confirmLabel = 'Delete',
    String cancelLabel = 'Cancel',
  }) {
    return show(
      context: context,
      title: title,
      message: message,
      confirmLabel: confirmLabel,
      cancelLabel: cancelLabel,
      variant: ConfirmDialogVariant.danger,
      icon: Icons.delete_outline,
    );
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return AlertDialog(
      icon: icon != null
          ? Icon(icon, color: _getIconColor(colorScheme), size: 32)
          : null,
      title: Text(title),
      content: Text(message, style: theme.textTheme.bodyMedium),
      actions: [
        TextButton(
          onPressed: () => Navigator.of(context).pop(false),
          child: Text(cancelLabel),
        ),
        FilledButton(
          onPressed: isLoading ? null : () => Navigator.of(context).pop(true),
          style: FilledButton.styleFrom(
            backgroundColor: _getButtonColor(colorScheme),
            foregroundColor: _getButtonTextColor(colorScheme),
          ),
          child: isLoading
              ? SizedBox(
                  width: 16,
                  height: 16,
                  child: CircularProgressIndicator(
                    strokeWidth: 2,
                    valueColor: AlwaysStoppedAnimation<Color>(
                      _getButtonTextColor(colorScheme),
                    ),
                  ),
                )
              : Text(confirmLabel),
        ),
      ],
    );
  }

  Color _getIconColor(ColorScheme colorScheme) {
    return switch (variant) {
      ConfirmDialogVariant.primary => colorScheme.primary,
      ConfirmDialogVariant.danger => colorScheme.error,
      ConfirmDialogVariant.warning => colorScheme.tertiary,
    };
  }

  Color _getButtonColor(ColorScheme colorScheme) {
    return switch (variant) {
      ConfirmDialogVariant.primary => colorScheme.primary,
      ConfirmDialogVariant.danger => colorScheme.error,
      ConfirmDialogVariant.warning => colorScheme.tertiary,
    };
  }

  Color _getButtonTextColor(ColorScheme colorScheme) {
    return switch (variant) {
      ConfirmDialogVariant.primary => colorScheme.onPrimary,
      ConfirmDialogVariant.danger => colorScheme.onError,
      ConfirmDialogVariant.warning => colorScheme.onTertiary,
    };
  }
}

/// Dialog variant for different confirmation types.
enum ConfirmDialogVariant {
  /// Standard confirmation (primary color)
  primary,

  /// Destructive action confirmation (error color)
  danger,

  /// Warning confirmation (tertiary color)
  warning,
}
