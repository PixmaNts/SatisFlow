import 'package:flutter_test/flutter_test.dart';
import 'package:satisflow/main.dart';
import 'package:satisflow/src/rust/frb_generated.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());
  testWidgets('App launches successfully', (WidgetTester tester) async {
    await tester.pumpWidget(const SatisflowApp());
    await tester.pumpAndSettle();
    // Verify the app launches
    expect(find.byType(SatisflowApp), findsOneWidget);
  });
}
