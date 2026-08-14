import 'package:flutter/material.dart';

/// A loading overlay widget that displays a spinner over content.
///
/// Uses Theme.of(context) for all colors - no hardcoded values.
class LoadingOverlay extends StatelessWidget {
  /// Whether the overlay is visible.
  final bool isLoading;

  /// The content to display behind the overlay.
  final Widget child;

  /// Optional loading message to display.
  final String? message;

  /// The color of the overlay background.
  final Color? backgroundColor;

  /// The color of the loading indicator.
  final Color? indicatorColor;

  /// The size of the loading indicator.
  final double indicatorSize;

  /// Creates a LoadingOverlay.
  const LoadingOverlay({
    super.key,
    required this.isLoading,
    required this.child,
    this.message,
    this.backgroundColor,
    this.indicatorColor,
    this.indicatorSize = 40,
  });

  /// Show a full-screen loading overlay.
  static Widget fullScreen({
    required bool isLoading,
    String? message,
    Color? backgroundColor,
    Color? indicatorColor,
    double indicatorSize = 40,
  }) {
    return Stack(
      children: [
        if (isLoading)
          Positioned.fill(
            child: _LoadingOverlayContent(
              message: message,
              backgroundColor: backgroundColor,
              indicatorColor: indicatorColor,
              indicatorSize: indicatorSize,
            ),
          ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        child,
        if (isLoading)
          Positioned.fill(
            child: _LoadingOverlayContent(
              message: message,
              backgroundColor: backgroundColor,
              indicatorColor: indicatorColor,
              indicatorSize: indicatorSize,
            ),
          ),
      ],
    );
  }
}

class _LoadingOverlayContent extends StatelessWidget {
  final String? message;
  final Color? backgroundColor;
  final Color? indicatorColor;
  final double indicatorSize;

  const _LoadingOverlayContent({
    this.message,
    this.backgroundColor,
    this.indicatorColor,
    this.indicatorSize = 40,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colorScheme = theme.colorScheme;

    return Container(
      color: backgroundColor ?? colorScheme.surface.withValues(alpha: 0.8),
      child: Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            SizedBox(
              width: indicatorSize,
              height: indicatorSize,
              child: CircularProgressIndicator(
                strokeWidth: 3,
                valueColor: AlwaysStoppedAnimation<Color>(
                  indicatorColor ?? colorScheme.primary,
                ),
              ),
            ),
            if (message != null) ...[
              const SizedBox(height: 16),
              Text(
                message!,
                style: theme.textTheme.bodyLarge?.copyWith(
                  color: colorScheme.onSurface,
                ),
                textAlign: TextAlign.center,
              ),
            ],
          ],
        ),
      ),
    );
  }
}
