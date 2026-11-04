import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:path_provider/path_provider.dart';

bool _recording = false;

/// Native assertions remain authoritative when a USB VM-service attach fails.
/// This file is written only after the test binding completes the whole suite.
void recordDeviceResult(IntegrationTestWidgetsFlutterBinding binding) {
  if (_recording) return;
  _recording = true;
  final errors = <Map<String, String?>>[];
  final previous = reportTestException;
  reportTestException = (details, description) {
    errors.add({
      'test': description,
      'message': details.exceptionAsString(),
      'stack': details.stack?.toString(),
    });
    previous(details, description);
  };
  unawaited(
    binding.allTestsPassed.future.then((passed) async {
      final support = await getApplicationSupportDirectory();
      final results = {
        'passed': passed,
        'errors': errors,
        'configuration': {
          'liveAccount': const bool.fromEnvironment(
            'OPENMUSE_ACCEPTANCE_LIVE_ACCOUNT',
          ),
          'liveDesktop': const bool.fromEnvironment(
            'OPENMUSE_ACCEPTANCE_LIVE_DESKTOP',
          ),
          'liveAI': const bool.fromEnvironment('OPENMUSE_ACCEPTANCE_LIVE_AI'),
          'deleteAccount': const bool.fromEnvironment(
            'OPENMUSE_ACCEPTANCE_DELETE_ACCOUNT',
          ),
        },
        'milestones': binding.reportData?['milestones'],
        'tests': binding.results.map(
          (name, result) => MapEntry(name, result.toString()),
        ),
      };
      await File(
        '${support.path}/acceptance-results.json',
      ).writeAsString(jsonEncode(results), flush: true);
    }),
  );
}
