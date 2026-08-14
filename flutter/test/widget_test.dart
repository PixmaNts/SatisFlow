// Main widget test entry point for SatisFlow app.
// This file imports and runs all widget tests.

import 'package:flutter_test/flutter_test.dart';

// Widget tests
import 'widgets/app_button_test.dart' as app_button;
import 'widgets/confirm_dialog_test.dart' as confirm_dialog;

// Page tests
import 'pages/dashboard_page_test.dart' as dashboard;
import 'pages/factory_page_test.dart' as factory_page;
import 'pages/logistics_page_test.dart' as logistics;

void main() {
  group('SatisFlow Widget Tests', () {
    group('Shared Widgets', () {
      app_button.main();
      confirm_dialog.main();
    });

    group('Page Widgets', () {
      dashboard.main();
      factory_page.main();
      logistics.main();
    });
  });
}
