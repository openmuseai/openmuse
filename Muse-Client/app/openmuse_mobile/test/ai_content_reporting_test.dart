import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/ai_content_reporting.dart';

void main() {
  AiContentReport draft() => AiContentReport(
    category: 'harmful',
    response: 'QA response',
    comment: '',
    provider: 'QA',
    model: 'qa-model',
  );

  test(
    'sends only the selected response after authentication and a durable receipt',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final report = draft();
      final handled = server.first.then((request) async {
        expect(request.uri.toString(), '/v1/content-reports');
        expect(request.headers.value('Authorization'), 'Bearer qa-token');
        final body = jsonDecode(await utf8.decoder.bind(request).join()) as Map;
        expect(body.keys.toSet(), {
          'id',
          'category',
          'response',
          'comment',
          'provider',
          'model',
        });
        expect(body['response'], 'QA response');
        request.response.statusCode = 201;
        request.response.write(jsonEncode({'accepted': true, 'id': report.id}));
        await request.response.close();
      });
      try {
        final client = AiContentReportClient(
          origin: Uri.parse('http://127.0.0.1:${server.port}'),
          allowInsecureLoopback: true,
          accessToken: () async => 'qa-token',
          currentAccount: () => 'a',
        );
        expect(await client.submit(report, account: 'a'), report.id);
        await handled;
      } finally {
        await server.close(force: true);
      }
    },
  );

  test(
    'erases reports through the same authenticated account before account deletion',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final handled = server.first.then((request) async {
        expect(request.method, 'DELETE');
        expect(request.uri.path, '/v1/content-reports');
        expect(request.headers.value('Authorization'), 'Bearer qa-token');
        request.response.write('{"deleted":true}');
        await request.response.close();
      });
      try {
        final client = AiContentReportClient(
          origin: Uri.parse('http://127.0.0.1:${server.port}'),
          allowInsecureLoopback: true,
          accessToken: () async => 'qa-token',
          currentAccount: () => 'a',
        );
        await client.eraseForAccount(account: 'a');
        await handled;
      } finally {
        await server.close(force: true);
      }
    },
  );
  test(
    'account change while retrieving the token prevents transmission',
    () async {
      var account = 'a';
      final client = AiContentReportClient(
        origin: Uri.parse('https://unused.example'),
        accessToken: () async {
          account = 'b';
          return 'qa-token';
        },
        currentAccount: () => account,
      );
      await expectLater(
        client.submit(draft(), account: 'a'),
        throwsA(
          isA<AiReportFailure>().having((e) => e.signIn, 'sign in', true),
        ),
      );
    },
  );

  test(
    'server error, redirect and unconfirmed receipt are never success',
    () async {
      for (final status in [302, 503, 201]) {
        final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
        final handled = server.first.then((request) async {
          request.response.statusCode = status;
          request.response.headers.set(
            'Location',
            'https://must-not-contact.example/',
          );
          request.response.write('{"accepted":true,"id":"different-report"}');
          await request.response.close();
        });
        try {
          final client = AiContentReportClient(
            origin: Uri.parse('http://127.0.0.1:${server.port}'),
            allowInsecureLoopback: true,
            accessToken: () async => 'qa-token',
            currentAccount: () => 'a',
          );
          await expectLater(
            client.submit(draft(), account: 'a'),
            throwsA(isA<AiReportFailure>()),
          );
          await handled;
        } finally {
          await server.close(force: true);
        }
      }
    },
  );

  test(
    'clips the response without splitting Unicode and does not accept an unsafe origin',
    () {
      final text = List.filled(6001, String.fromCharCode(0x1f600)).join();
      final excerpt = reportExcerpt(text);
      expect(excerpt.runes.length, 6000);
      expect(utf8.decode(utf8.encode(excerpt)), excerpt);
      for (final origin in [
        'http://public.example',
        'https://user:pass@example.com',
        'https://example.com/?key=bad',
      ]) {
        expect(
          () => AiContentReportClient(
            origin: Uri.parse(origin),
            accessToken: () async => null,
            currentAccount: () => null,
          ),
          throwsFormatException,
        );
      }
    },
  );
}
