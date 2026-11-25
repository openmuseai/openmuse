import 'package:muse_collab_contract/muse_collab_contract.dart';
import 'package:test/test.dart';

void main() {
  const document = CollaborationBinding(
    protocol: 'test.exchange/1',
    engine: 'test.engine/1',
    encoding: 'test.bytes/1',
    schema: 'example.document/1',
    epoch: 0,
  );
  test(
    'document and independent workbook can use the same neutral binding type',
    () {
      final restored = CollaborationBinding.fromJson(document.toJson());
      expect(restored.compatibleWith(document), isTrue);
      final workbook = CollaborationBinding.fromJson({
        ...document.toJson(),
        'schema': 'example.workbook/1',
      });
      expect(workbook.schema, 'example.workbook/1');
      expect(workbook.compatibleWith(document), isFalse);
    },
  );
  test('unknown contract, extra fields and invalid epoch are rejected', () {
    for (final value in [
      {...document.toJson(), 'contract': 'unknown/2'},
      {...document.toJson(), 'epoch': -1},
      {...document.toJson(), 'epoch': 0.5},
      {...document.toJson(), 'paragraph': {}},
      {...document.toJson()}..remove('engine'),
    ]) {
      expect(() => CollaborationBinding.fromJson(value), throwsFormatException);
    }
  });
  test(
    'engine, encoding, schema, protocol or epoch cannot change implicitly',
    () {
      for (final entry in {
        'protocol': 'other',
        'engine': 'other',
        'encoding': 'other',
        'schema': 'other',
        'epoch': 1,
      }.entries) {
        expect(
          CollaborationBinding.fromJson({
            ...document.toJson(),
            entry.key: entry.value,
          }).compatibleWith(document),
          isFalse,
        );
      }
    },
  );
}
