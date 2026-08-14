import 'package:flutter/material.dart';

/// An error display widget with retry button.
///
/// Uses Theme.of(context) for all colors - no hardcoded values.
class ErrorDisplay extends StatelessWidget {
  /// The error message to display.
  final String message;

  /// Optional error details/stack trace.
  final String? details;

  /// Callback when retry button is pressed.
  final VoidCallback? onRetry;

  /// The retry button label.
  final String retryLabel;

  /// Whether to show the retry button.
  final bool showRetryButton;

  /// The icon to display.
  final IconData icon;

  /// The size of the icon.
  final double iconSize;

  /// Creates an ErrorDisplay.
  const ErrorDisplay({
    super.key,
    required this.message,
    this.details,
    this.onRetry,
    this.retryLabel = 'Retry',
    this.showRetryButton = true,
    this.icon = Icons.error_outline,
    this.iconSize = 64,
  });

  /// Creates an ErrorDisplay for network errors.
  const ErrorDisplay.network({
    super.key,
    this.message = 'Network error',
    this.details = 'Check your internet connection and try again',
    this.onRetry,
    this.retryLabel = 'Retry',
    this.showRetryButton = true,
    this.icon = Icons.wifi_off,
    this.iconSize = 64,
  });

  /// Creates an ErrorDisplay for server errors.
  const ErrorDisplay.server({
    super.key,
    this.message = 'Server error',
    this.details = 'Something went wrong on our end. Please try again later',
    this.onRetry,
    this.retryLabel = 'Retry',
    this.showRetryButton = true,
    this.icon = Icons.cloud_off,
    this.iconSize = 64,
  });

  /// Creates an ErrorDisplay for permission errors.
  const ErrorDisplay.permission({
    super.key,
    this.message = 'Permission denied',
    this.details = 'You don\'t have permission to access this resource',
    this.onRetry,
    this.retryLabel = 'Request Access',
    this.showRetryButton = true,
    this.icon = Icons.lock_outline,
    this.iconSize = 64,
  });

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
            Icon(icon, size: iconSize, color: colorScheme.error),
            const SizedBox(height: 16),
            Text(
              message,
              style: theme.textTheme.headlineSmall?.copyWith(
                color: colorScheme.onSurface,
              ),
              textAlign: TextAlign.center,
            ),
            if (details != null) ...[
              const SizedBox(height: 8),
              Text(
                details!,
                style: theme.textTheme.bodyMedium?.copyWith(
                  color: colorScheme.onSurfaceVariant,
                ),
                textAlign: TextAlign.center,
              ),
            ],
            if (showRetryButton && onRetry != null) ...[
              const SizedBox(height: 24),
              FilledButton.icon(
                onPressed: onRetry,
                icon: const Icon(Icons.refresh),
                label: Text(retryLabel),
                style: FilledButton.styleFrom(
                  backgroundColor: colorScheme.error,
                  foregroundColor: colorScheme.onError,
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }
}
