// Widget tests for AppButton

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:satisflow/shared/widgets/app_button.dart';

import '../helpers/test_helpers.dart';

void main() {
  group('AppButton', () {
    testWidgets('renders primary button with label', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithApp(const AppButton(label: 'Test Button'));

      expect(find.text('Test Button'), findsOneWidget);
      expect(find.byType(FilledButton), findsOneWidget);
    });

    testWidgets('renders secondary button', (WidgetTester tester) async {
      await tester.pumpWithApp(
        const AppButton.secondary(label: 'Secondary Button'),
      );

      expect(find.text('Secondary Button'), findsOneWidget);
      expect(find.byType(OutlinedButton), findsOneWidget);
    });

    testWidgets('renders danger button', (WidgetTester tester) async {
      await tester.pumpWithApp(const AppButton.danger(label: 'Delete'));

      expect(find.text('Delete'), findsOneWidget);
      expect(find.byType(FilledButton), findsOneWidget);
    });

    testWidgets('renders text button', (WidgetTester tester) async {
      await tester.pumpWithApp(const AppButton.text(label: 'Text Button'));

      expect(find.text('Text Button'), findsOneWidget);
      expect(find.byType(TextButton), findsOneWidget);
    });

    testWidgets('calls onPressed when tapped', (WidgetTester tester) async {
      bool wasPressed = false;

      await tester.pumpWithApp(
        AppButton(label: 'Press Me', onPressed: () => wasPressed = true),
      );

      await tester.tap(find.text('Press Me'));
      await tester.pump();

      expect(wasPressed, isTrue);
    });

    testWidgets('renders with icon', (WidgetTester tester) async {
      await tester.pumpWithApp(
        const AppButton(label: 'With Icon', icon: Icons.add),
      );

      expect(find.text('With Icon'), findsOneWidget);
      expect(find.byIcon(Icons.add), findsOneWidget);
    });

    testWidgets('shows loading indicator when isLoading is true', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithApp(
        const AppButton(label: 'Loading', isLoading: true),
      );

      expect(find.byType(CircularProgressIndicator), findsOneWidget);
      expect(find.text('Loading'), findsNothing);
    });

    testWidgets('is disabled when onPressed is null', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithApp(
        const AppButton(label: 'Disabled', onPressed: null),
      );

      final button = tester.widget<FilledButton>(find.byType(FilledButton));
      expect(button.onPressed, isNull);
    });

    testWidgets('is disabled when isLoading is true', (
      WidgetTester tester,
    ) async {
      bool wasPressed = false;

      await tester.pumpWithApp(
        AppButton(
          label: 'Loading',
          isLoading: true,
          onPressed: () => wasPressed = true,
        ),
      );

      await tester.tap(find.byType(FilledButton));
      await tester.pump();

      expect(wasPressed, isFalse);
    });

    testWidgets('renders full width button', (WidgetTester tester) async {
      await tester.pumpWithApp(
        const AppButton(label: 'Full Width', fullWidth: true),
      );

      expect(find.text('Full Width'), findsOneWidget);
      final sizedBox = tester.widget<SizedBox>(find.byType(SizedBox).first);
      expect(sizedBox.width, double.infinity);
    });

    testWidgets('renders different sizes', (WidgetTester tester) async {
      // Small
      await tester.pumpWithApp(
        const AppButton(label: 'Small', size: AppButtonSize.small),
      );
      expect(find.text('Small'), findsOneWidget);

      // Medium (default)
      await tester.pumpWithApp(
        const AppButton(label: 'Medium', size: AppButtonSize.medium),
      );
      expect(find.text('Medium'), findsOneWidget);

      // Large
      await tester.pumpWithApp(
        const AppButton(label: 'Large', size: AppButtonSize.large),
      );
      expect(find.text('Large'), findsOneWidget);
    });

    testWidgets('constructor factories create correct variants', (
      WidgetTester tester,
    ) async {
      // Test each named constructor
      await tester.pumpWithApp(
        Column(
          children: [
            const AppButton.primary(label: 'Primary'),
            const AppButton.secondary(label: 'Secondary'),
            const AppButton.danger(label: 'Danger'),
            const AppButton.text(label: 'Text'),
          ],
        ),
      );

      expect(find.text('Primary'), findsOneWidget);
      expect(find.text('Secondary'), findsOneWidget);
      expect(find.text('Danger'), findsOneWidget);
      expect(find.text('Text'), findsOneWidget);
    });
  });
}
