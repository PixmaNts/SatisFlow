import 'package:flutter/material.dart';

/// Extension methods on BuildContext for convenient access to theme and media query data.
extension ContextExtensions on BuildContext {
  // Theme shortcuts

  /// The current theme data.
  ThemeData get theme => Theme.of(this);

  /// The current color scheme.
  ColorScheme get colorScheme => Theme.of(this).colorScheme;

  /// The current text theme.
  TextTheme get textTheme => Theme.of(this).textTheme;

  /// The current platform theme data.
  // PlatformThemeData get platformTheme => PlatformTheme.of(this);

  // Color shortcuts

  /// Primary color.
  Color get primaryColor => colorScheme.primary;

  /// Primary container color.
  Color get primaryContainer => colorScheme.primaryContainer;

  /// Secondary color.
  Color get secondaryColor => colorScheme.secondary;

  /// Secondary container color.
  Color get secondaryContainer => colorScheme.secondaryContainer;

  /// Tertiary color.
  Color get tertiaryColor => colorScheme.tertiary;

  /// Tertiary container color.
  Color get tertiaryContainer => colorScheme.tertiaryContainer;

  /// Error color.
  Color get errorColor => colorScheme.error;

  /// Error container color.
  Color get errorContainer => colorScheme.errorContainer;

  /// Background color.
  Color get backgroundColor => colorScheme.surface;

  /// Surface color.
  Color get surfaceColor => colorScheme.surface;

  /// Surface variant color.
  Color get surfaceVariantColor => colorScheme.surfaceContainerHighest;

  /// On primary color.
  Color get onPrimaryColor => colorScheme.onPrimary;

  /// On secondary color.
  Color get onSecondaryColor => colorScheme.onSecondary;

  /// On error color.
  Color get onErrorColor => colorScheme.onError;

  /// On background color.
  Color get onBackgroundColor => colorScheme.onSurface;

  /// On surface color.
  Color get onSurfaceColor => colorScheme.onSurface;

  /// On surface variant color.
  Color get onSurfaceVariantColor => colorScheme.onSurfaceVariant;

  /// Outline color.
  Color get outlineColor => colorScheme.outline;

  /// Outline variant color.
  Color get outlineVariantColor => colorScheme.outlineVariant;

  // Text style shortcuts

  /// Display large text style.
  TextStyle? get displayLarge => textTheme.displayLarge;

  /// Display medium text style.
  TextStyle? get displayMedium => textTheme.displayMedium;

  /// Display small text style.
  TextStyle? get displaySmall => textTheme.displaySmall;

  /// Headline large text style.
  TextStyle? get headlineLarge => textTheme.headlineLarge;

  /// Headline medium text style.
  TextStyle? get headlineMedium => textTheme.headlineMedium;

  /// Headline small text style.
  TextStyle? get headlineSmall => textTheme.headlineSmall;

  /// Title large text style.
  TextStyle? get titleLarge => textTheme.titleLarge;

  /// Title medium text style.
  TextStyle? get titleMedium => textTheme.titleMedium;

  /// Title small text style.
  TextStyle? get titleSmall => textTheme.titleSmall;

  /// Body large text style.
  TextStyle? get bodyLarge => textTheme.bodyLarge;

  /// Body medium text style.
  TextStyle? get bodyMedium => textTheme.bodyMedium;

  /// Body small text style.
  TextStyle? get bodySmall => textTheme.bodySmall;

  /// Label large text style.
  TextStyle? get labelLarge => textTheme.labelLarge;

  /// Label medium text style.
  TextStyle? get labelMedium => textTheme.labelMedium;

  /// Label small text style.
  TextStyle? get labelSmall => textTheme.labelSmall;

  // MediaQuery shortcuts

  /// The media query data.
  MediaQueryData get mediaQuery => MediaQuery.of(this);

  /// The screen size.
  Size get screenSize => mediaQuery.size;

  /// The screen width.
  double get screenWidth => screenSize.width;

  /// The screen height.
  double get screenHeight => screenSize.height;

  /// The device pixel ratio.
  double get devicePixelRatio => mediaQuery.devicePixelRatio;

  /// The text scale factor.
  double get textScaleFactor => mediaQuery.textScaler.scale(1.0);

  /// The platform brightness.
  Brightness get platformBrightness => mediaQuery.platformBrightness;

  /// Whether the device is in dark mode.
  bool get isDarkMode => platformBrightness == Brightness.dark;

  /// Whether the device is in landscape orientation.
  bool get isLandscape => mediaQuery.orientation == Orientation.landscape;

  /// Whether the device is in portrait orientation.
  bool get isPortrait => mediaQuery.orientation == Orientation.portrait;

  /// The bottom padding (safe area).
  double get bottomPadding => mediaQuery.padding.bottom;

  /// The top padding (safe area).
  double get topPadding => mediaQuery.padding.top;

  /// The left padding (safe area).
  double get leftPadding => mediaQuery.padding.left;

  /// The right padding (safe area).
  double get rightPadding => mediaQuery.padding.right;

  /// The view insets (keyboard).
  double get viewInsetsBottom => mediaQuery.viewInsets.bottom;

  /// Whether the keyboard is visible.
  bool get isKeyboardVisible => viewInsetsBottom > 0;

  // Responsive helpers

  /// Whether the screen is small (width < 600).
  bool get isSmallScreen => screenWidth < 600;

  /// Whether the screen is medium (600 <= width < 900).
  bool get isMediumScreen => screenWidth >= 600 && screenWidth < 900;

  /// Whether the screen is large (width >= 900).
  bool get isLargeScreen => screenWidth >= 900;

  /// Whether the screen is extra large (width >= 1200).
  bool get isExtraLargeScreen => screenWidth >= 1200;

  // Navigation shortcuts

  /// Pop the current route.
  void pop<T>([T? result]) => Navigator.of(this).pop(result);

  /// Push a new route.
  Future<T?> push<T>(Route<T> route) => Navigator.of(this).push(route);

  /// Push a named route.
  Future<T?> pushNamed<T>(String routeName, {Object? arguments}) =>
      Navigator.of(this).pushNamed<T>(routeName, arguments: arguments);

  /// Push and remove all routes.
  Future<T?> pushAndRemoveAll<T>(Route<T> route) =>
      Navigator.of(this).pushAndRemoveUntil(route, (route) => false);

  /// Push a replacement route.
  Future<T?> pushReplacement<T, TO>(Route<T> newRoute, {TO? result}) =>
      Navigator.of(this).pushReplacement(newRoute, result: result);

  /// Whether can pop.
  bool get canPop => Navigator.of(this).canPop();

  // SnackBar shortcuts

  /// Show a snackbar.
  void showSnackBar(
    String message, {
    Duration duration = const Duration(seconds: 4),
    SnackBarAction? action,
  }) {
    ScaffoldMessenger.of(this).showSnackBar(
      SnackBar(content: Text(message), duration: duration, action: action),
    );
  }

  /// Show an error snackbar.
  void showErrorSnackBar(String message) {
    showSnackBar(message, duration: const Duration(seconds: 6));
  }

  /// Show a success snackbar.
  void showSuccessSnackBar(String message) {
    showSnackBar(message);
  }

  // Dialog shortcuts

  /// Show an alert dialog.
  Future<bool?> showAlertDialog({
    required String title,
    required String content,
    String confirmLabel = 'OK',
    String? cancelLabel,
  }) {
    return showDialog<bool>(
      context: this,
      builder: (context) => AlertDialog(
        title: Text(title),
        content: Text(content),
        actions: [
          if (cancelLabel != null)
            TextButton(
              onPressed: () => Navigator.of(context).pop(false),
              child: Text(cancelLabel),
            ),
          TextButton(
            onPressed: () => Navigator.of(context).pop(true),
            child: Text(confirmLabel),
          ),
        ],
      ),
    );
  }

  // Focus shortcuts

  /// Unfocus the current focus node.
  void unfocus() => FocusScope.of(this).unfocus();

  /// Request focus on a focus node.
  void requestFocus(FocusNode focusNode) =>
      FocusScope.of(this).requestFocus(focusNode);
}
