import 'package:flutter/material.dart';

/// Modal type for showing different modal styles.
enum AppModalType {
  /// Bottom sheet modal
  bottomSheet,

  /// Center dialog modal
  dialog,
}

/// A reusable modal wrapper for bottom sheets and dialogs.
///
/// Uses Theme.of(context) for all colors - no hardcoded values.
class AppModal extends StatelessWidget {
  /// The content to display inside the modal.
  final Widget child;

  /// Optional title for the modal.
  final String? title;

  /// Whether to show the close button.
  final bool showCloseButton;

  /// The type of modal to display.
  final AppModalType type;

  /// Whether the modal can be dismissed by tapping outside.
  final bool barrierDismissible;

  /// Optional action buttons to show at the bottom.
  final List<Widget>? actions;

  /// Maximum height for bottom sheet (as fraction of screen height).
  final double maxChildSize;

  /// Creates an AppModal.
  const AppModal({
    super.key,
    required this.child,
    this.title,
    this.showCloseButton = true,
    this.type = AppModalType.bottomSheet,
    this.barrierDismissible = true,
    this.actions,
    this.maxChildSize = 0.9,
  });

  /// Show as a bottom sheet.
  static Future<T?> showBottomSheet<T>({
    required BuildContext context,
    required Widget child,
    String? title,
    bool showCloseButton = true,
    bool barrierDismissible = true,
    List<Widget>? actions,
    double maxChildSize = 0.9,
  }) {
    return showModalBottomSheet<T>(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (context) => AppModal(
        title: title,
        showCloseButton: showCloseButton,
        type: AppModalType.bottomSheet,
        barrierDismissible: barrierDismissible,
        actions: actions,
        maxChildSize: maxChildSize,
        child: child,
      ),
    );
  }

  /// Show as a dialog.
  static Future<T?> showAsDialog<T>({
    required BuildContext context,
    required Widget child,
    String? title,
    bool showCloseButton = true,
    bool barrierDismissible = true,
    List<Widget>? actions,
  }) {
    return showDialog<T>(
      context: context,
      barrierDismissible: barrierDismissible,
      builder: (context) => AppModal(
        title: title,
        showCloseButton: showCloseButton,
        type: AppModalType.dialog,
        barrierDismissible: barrierDismissible,
        actions: actions,
        child: child,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return switch (type) {
      AppModalType.bottomSheet => _buildBottomSheet(
        context,
        theme,
        colorScheme,
      ),
      AppModalType.dialog => _buildDialog(context, theme, colorScheme),
    };
  }

  Widget _buildBottomSheet(
    BuildContext context,
    ThemeData theme,
    ColorScheme colorScheme,
  ) {
    return Container(
      constraints: BoxConstraints(
        maxHeight: MediaQuery.of(context).size.height * maxChildSize,
      ),
      decoration: BoxDecoration(
        color: colorScheme.surface,
        borderRadius: const BorderRadius.vertical(top: Radius.circular(16)),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          // Drag handle
          Container(
            margin: const EdgeInsets.symmetric(vertical: 12),
            width: 40,
            height: 4,
            decoration: BoxDecoration(
              color: colorScheme.onSurfaceVariant.withValues(alpha: 0.4),
              borderRadius: BorderRadius.circular(2),
            ),
          ),

          // Header
          if (title != null || showCloseButton)
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: Row(
                children: [
                  if (title != null)
                    Expanded(
                      child: Text(title!, style: theme.textTheme.titleLarge),
                    ),
                  if (showCloseButton)
                    IconButton(
                      icon: const Icon(Icons.close),
                      onPressed: () => Navigator.of(context).pop(),
                    ),
                ],
              ),
            ),

          // Content
          Flexible(
            child: SingleChildScrollView(
              padding: const EdgeInsets.all(16),
              child: child,
            ),
          ),

          // Actions
          if (actions != null && actions!.isNotEmpty)
            Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: actions!
                    .expand((widget) => [const SizedBox(width: 8), widget])
                    .skip(1)
                    .toList(),
              ),
            ),
        ],
      ),
    );
  }

  Widget _buildDialog(
    BuildContext context,
    ThemeData theme,
    ColorScheme colorScheme,
  ) {
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 400),
        child: Material(
          color: Colors.transparent,
          child: Container(
            decoration: BoxDecoration(
              color: colorScheme.surface,
              borderRadius: BorderRadius.circular(16),
            ),
            padding: const EdgeInsets.all(24),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                // Header
                if (title != null || showCloseButton)
                  Row(
                    children: [
                      if (title != null)
                        Expanded(
                          child: Text(
                            title!,
                            style: theme.textTheme.titleLarge,
                          ),
                        ),
                      if (showCloseButton)
                        IconButton(
                          icon: const Icon(Icons.close),
                          onPressed: () => Navigator.of(context).pop(),
                        ),
                    ],
                  ),

                // Content
                child,

                // Actions
                if (actions != null && actions!.isNotEmpty)
                  Padding(
                    padding: const EdgeInsets.only(top: 24),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.end,
                      children: actions!
                          .expand(
                            (widget) => [const SizedBox(width: 8), widget],
                          )
                          .skip(1)
                          .toList(),
                    ),
                  ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
