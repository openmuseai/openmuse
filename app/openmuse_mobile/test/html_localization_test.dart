import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_mobile/mobile_plugins/scheduled_tasks_plugin.dart';
import 'package:openmuse_mobile/mobile_plugins/workspace_files_plugin.dart';

void main() {
  test(
    'host-owned HTML changes language without translating user values or losing editor controls',
    () async {
      final catalog = OpenMuseCatalog.builtIn;
      final locale = OpenMuseLocaleController(
        initialTag: catalog.templateLocale,
      );
      final tasks = MobileScheduledTasksPlugin(strings: locale.strings);
      final files = MobileWorkspaceFilesPlugin(strings: locale.strings);
      final documents = {
        'tasks': tasks.buildMobileDetailDocument(Brightness.light),
        'files': files.buildMobileDetailDocument(Brightness.light),
      };
      final next = catalog.namespaceData(catalog.fallbackLocale, 'html');
      final registry =
          jsonDecode(
                File(
                  '../../packages/openmuse_i18n/assets/catalog.json',
                ).readAsStringSync(),
              )
              as Map;
      final templates = <Object?>[];
      for (final definition in (registry['entries'] as Map).values) {
        for (final source in (definition['messages'] as Map).values) {
          final parsed = MessageTemplate(source as String);
          templates.add({
            'source': source,
            'nodes': parsed.toData(),
            'parameters': parsed.parameters.toList(),
          });
        }
      }
      final directory = await Directory.systemTemp.createTemp('openmuse-html-');
      try {
        final fixture = File('${directory.path}/fixture.json');
        await fixture.writeAsString(
          jsonEncode({
            'documents': documents,
            'next': next,
            'templates': templates,
            'nextTitle': {
              'tasks': catalog.format(
                catalog.fallbackLocale,
                'html.scheduled_tasks',
              ),
              'files': catalog.format(
                catalog.fallbackLocale,
                'html.workspace_files',
              ),
            },
          }),
        );
        final result = await Process.run('node', [
          '../../services/config-control/test/check-html-runtime.mjs',
          fixture.path,
        ]);
        expect(
          result.exitCode,
          0,
          reason: '${result.stdout}\n${result.stderr}',
        );
      } finally {
        await directory.delete(recursive: true);
        locale.dispose();
      }
    },
  );
}
