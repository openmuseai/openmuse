import 'package:appflowy/plugins/dsh_agent/dsh_embedded_view.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('parses a DSH diff deep link without exposing protocol internals', () {
    final message = DshResourceOpenMessage.fromJson({
      'type': 'resource.diff.open',
      'path': 'lib/main.dart',
      'cwd': '/workspace',
      'comparisonId': 'comparison:123',
      'changeId': 'change:456',
      'line': 18,
    });

    expect(message.path, 'lib/main.dart');
    expect(message.cwd, '/workspace');
    expect(message.comparisonId, 'comparison:123');
    expect(message.changeId, 'change:456');
    expect(message.line, 18);
  });

  test('rejects malformed or oversized DSH diff links', () {
    expect(
      () => DshResourceOpenMessage.fromJson({
        'path': '',
        'cwd': '/workspace',
        'comparisonId': 'comparison:123',
      }),
      throwsFormatException,
    );
    expect(
      () => DshResourceOpenMessage.fromJson({
        'path': 'file.dart',
        'cwd': '/workspace',
        'comparisonId': List.filled(513, 'x').join(),
      }),
      throwsFormatException,
    );
  });
}
