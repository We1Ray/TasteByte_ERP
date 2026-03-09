import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:tastebyte_erp_mobile/main.dart';

void main() {
  testWidgets('App renders login screen', (WidgetTester tester) async {
    await tester.pumpWidget(
      const ProviderScope(child: TasteByteApp()),
    );
    await tester.pumpAndSettle();

    expect(find.text('TasteByte ERP'), findsWidgets);
  });
}
