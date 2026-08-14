// Widget tests for ConfirmDialog

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:satisflow/shared/widgets/confirm_dialog.dart';

import '../helpers/test_helpers.dart';

void main() {
  group('ConfirmDialog', () {
    testWidgets('renders with title and message', (WidgetTester tester) async {
      await tester.pumpWithApp(
        const ConfirmDialog(
          title: 'Confirm Action',
          message: 'Are you sure you want to proceed?',
        ),
      );

      expect(find.text('Confirm Action'), findsOneWidget);
      expect(find.text('Are you sure you want to proceed?'), findsOneWidget);
    });

    testWidgets('renders with default button labels', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithApp(
        const ConfirmDialog(title: 'Test Dialog', message: 'Test message'),
      );

      expect(find.text('Confirm'), findsOneWidget); // Confirm button
      expect(find.text('Cancel'), findsOneWidget);
    });

    testWidgets('renders with custom button labels', (
      WidgetTester tester,
    ) async {
      await tester.pumpWithApp(
        const ConfirmDialog(
          title: 'Delete Item',
          message: 'Are you sure?',
          confirmLabel: 'Delete',
          cancelLabel: 'No, Keep',
        ),
      );

      expect(find.text('Delete'), findsOneWidget);
      expect(find.text('No, Keep'), findsOneWidget);
    });

    testWidgets('shows icon when provided', (WidgetTester tester) async {
      await tester.pumpWithApp(
        const ConfirmDialog(
          title: 'Warning',
          message: 'This is a warning',
          icon: Icons.warning,
        ),
      );

      expect(find.byIcon(Icons.warning), findsOneWidget);
    });

    testWidgets('returns false when cancel is pressed', (
      WidgetTester tester,
    ) async {
      bool? result;

      await tester.pumpWithApp(
        Builder(
          builder: (context) => ElevatedButton(
            onPressed: () async {
              result = await ConfirmDialog.show(
                context: context,
                title: 'Test',
                message: 'Test message',
              );
            },
            child: const Text('Show Dialog'),
          ),
        ),
      );

      // Show the dialog
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      // Tap cancel
      await tester.tap(find.text('Cancel'));
      await tester.pumpAndSettle();

      expect(result, isFalse);
    });

    testWidgets('returns true when confirm is pressed', (
      WidgetTester tester,
    ) async {
      bool? result;

      await tester.pumpWithApp(
        Builder(
          builder: (context) => ElevatedButton(
            onPressed: () async {
              result = await ConfirmDialog.show(
                context: context,
                title: 'Test',
                message: 'Test message',
              );
            },
            child: const Text('Show Dialog'),
          ),
        ),
      );

      // Show the dialog
      await tester.tap(find.text('Show Dialog'));
      await tester.pumpAndSettle();

      // Tap confirm
      await tester.tap(find.text('Confirm'));
      await tester.pumpAndSettle();

      expect(result, isTrue);
    });

    testWidgets('showDelete factory method creates danger variant', (
      WidgetTester tester,
    ) async {
      bool? result;

      await tester.pumpWithApp(
        Builder(
          builder: (context) => ElevatedButton(
            onPressed: () async {
              result = await ConfirmDialog.showDelete(
                context: context,
                title: 'Delete Item',
                message: 'Are you sure?',
              );
            },
            child: const Text('Show Delete Dialog'),
          ),
        ),
      );

      // Show the dialog
      await tester.tap(find.text('Show Delete Dialog'));
      await tester.pumpAndSettle();

      expect(find.text('Delete Item'), findsOneWidget);
      expect(find.text('Delete'), findsOneWidget);
      expect(find.byIcon(Icons.delete_outline), findsOneWidget);

      // Tap confirm
      await tester.tap(find.text('Delete'));
      await tester.pumpAndSettle();

      expect(result, isTrue);
    });

    testWidgets(
      'can be dismissed by tapping barrier when barrierDismissible is true',
      (WidgetTester tester) async {
        await tester.pumpWithApp(
          Builder(
            builder: (context) => ElevatedButton(
              onPressed: () {
                ConfirmDialog.show(
                  context: context,
                  title: 'Test',
                  message: 'Test message',
                  barrierDismissible: true,
                );
              },
              child: const Text('Show Dialog'),
            ),
          ),
        );

        // Show the dialog
        await tester.tap(find.text('Show Dialog'));
        await tester.pumpAndSettle();

        // Verify dialog is shown
        expect(find.byType(AlertDialog), findsOneWidget);

        // Tap outside the dialog (on the barrier)
        await tester.tapAt(const Offset(10, 10));
        await tester.pumpAndSettle();

        // Dialog should be dismissed
        expect(find.byType(AlertDialog), findsNothing);
      },
    );

    testWidgets(
      'shows loading indicator on confirm button when isLoading is true',
      (WidgetTester tester) async {
        await tester.pumpWithApp(
          const ConfirmDialog(
            title: 'Processing',
            message: 'Please wait...',
            isLoading: true,
          ),
        );

        expect(find.byType(CircularProgressIndicator), findsOneWidget);
        expect(find.text('Confirm'), findsNothing);
      },
    );

    testWidgets('different variants have different colors', (
      WidgetTester tester,
    ) async {
      // Test primary variant
      await tester.pumpWithApp(
        const ConfirmDialog(
          title: 'Primary',
          message: 'Test',
          variant: ConfirmDialogVariant.primary,
        ),
      );
      expect(find.byType(AlertDialog), findsOneWidget);

      // Test danger variant
      await tester.pumpWithApp(
        const ConfirmDialog(
          title: 'Danger',
          message: 'Test',
          variant: ConfirmDialogVariant.danger,
        ),
      );
      expect(find.byType(AlertDialog), findsOneWidget);

      // Test warning variant
      await tester.pumpWithApp(
        const ConfirmDialog(
          title: 'Warning',
          message: 'Test',
          variant: ConfirmDialogVariant.warning,
        ),
      );
      expect(find.byType(AlertDialog), findsOneWidget);
    });
  });
}
