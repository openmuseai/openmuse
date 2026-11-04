import 'dart:convert';
import 'dart:io';
import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mcp_dart/mcp_dart.dart';
import 'package:path/path.dart' as p;
import 'package:openmuse_linux/src/host/mcp_market.dart';
import 'package:openmuse_linux/src/host/mcp_bin_registry.dart';
import 'package:openmuse_linux/src/host/mcp_terminal_service.dart';
import 'package:openmuse_linux/src/host/mcp_capability_gateway.dart';
import 'package:openmuse_linux/src/host/mcp_installed_app.dart';

void main() {
  test(
    'Workspace CLI uses installed release providers, preserves files and packages an independent MCP App',
    () async {
      final ecosystem = Platform.environment['OPENMUSE_ECOSYSTEM_TEST_ROOT']!;
      final root = await Directory.systemTemp.createTemp(
        'devkit workspace tck ',
      );
      final workspace = await Directory(
        p.join(root.path, 'workspace with spaces'),
      ).create();
      await File(
        p.join(workspace.path, 'existing.txt'),
      ).writeAsString('Keep existing workspace files');
      final node = Platform.environment['OPENMUSE_NODE'] ?? 'node';
      final market = McpRegistryMarket(
        origin: Uri.parse('http://127.0.0.1'),
        installRoot: Directory(p.join(root.path, 'mcpb')),
        nodeExecutable: node,
      );
      Future<McpMarketEntry> install(
        String leaf,
        String version,
        String relative,
      ) async {
        final bytes = await File(p.join(ecosystem, relative)).readAsBytes();
        final entry = McpMarketEntry(
          name: 'io.openmuse/$leaf',
          title: leaf,
          description: 'Contract TCK release fixture',
          version: version,
          packageUri: Uri.parse('http://127.0.0.1/$leaf.mcpb'),
          sha256: sha256.convert(bytes).toString(),
        );
        await market.install(entry, bundleBytes: bytes);
        return entry;
      }

      final kit = await install(
        'developer-kit',
        '0.2.0',
        'plugins/developer-kit/dist/developer-kit-0.2.0.mcpb',
      );
      await install(
        'genoffice-pptx',
        '0.6.1',
        'plugins/genoffice-pptx/dist/genoffice-pptx-0.6.1.mcpb',
      );
      await install(
        'pdf-reader',
        '0.1.1',
        'plugins/pdf-reader/dist/pdf-reader-0.1.1.mcpb',
      );
      final environment = McpAppsEnvironment(
        market: market,
        nodeExecutable: node,
        scope: () => 'test-account',
      );
      var account = 'test-account';
      McpBundleSession? candidate;
      String? candidateDigest;
      final trace = <String>[];
      late McpTerminalService service;
      service = McpTerminalService(
        scope: () => account,
        tools: [
          ...McpAppsEnvironment.workspaceTools,
          ...McpCapabilityGateway.tools.where(
            (t) => {
              'host_describe',
              'capabilities_list',
              'development_open_bundle',
              'development_read_resource',
            }.contains(t['name']),
          ),
        ],
        call: (caller, authorizedRoot, name, args) async {
          trace.add(name);
          switch (name) {
            case 'host_describe':
              return environment.identity;
            case 'workspace_describe':
              return {
                'roots': [
                  {
                    'uri': Directory(authorizedRoot).uri.toString(),
                    'name': 'TCK workspace',
                  },
                ],
              };
            case 'capabilities_list':
              final providers = await environment.providers();
              try {
                return {'providers': providers.map((v) => v.toJson()).toList()};
              } finally {
                for (final provider in providers) {
                  await provider.close?.call();
                }
              }
            case 'registry_search':
              final query = (args['query'] as String).toLowerCase();
              return {
                'plugins': [
                  for (final entry in await market.installedEntries())
                    if (entry.name.contains(query))
                      {
                        'name': entry.name,
                        'version': entry.version,
                        'installed': true,
                      },
                ],
              };
            case 'development_open_bundle':
              // Protocol acceptance only. The native View is intentionally not
              // launched while the user is manually operating OpenMuse.
              final bytes = base64Decode(args['content'] as String);
              candidateDigest = sha256.convert(bytes).toString();
              expect(candidateDigest, args['sha256']);
              final zip = ZipDecoder().decodeBytes(bytes);
              final metadata =
                  jsonDecode(
                        utf8.decode(
                          zip.findFile('manifest.json')!.content as List<int>,
                        ),
                      )
                      as Map;
              final staging = McpRegistryMarket(
                origin: Uri.parse('http://127.0.0.1'),
                installRoot: Directory(p.join(root.path, 'candidate')),
                nodeExecutable: node,
              );
              final entry = McpMarketEntry(
                name: 'candidate/${metadata['name']}',
                title: 'Candidate',
                description: 'Headless protocol TCK',
                version: metadata['version'] as String,
                packageUri: Uri.parse('http://127.0.0.1/candidate.mcpb'),
                sha256: candidateDigest!,
              );
              await staging.install(entry, bundleBytes: bytes);
              candidate = await McpBundleRuntime(
                bundleDirectory: staging.directoryFor(entry),
                nodeExecutable: node,
              ).connect();
              final ui = await candidate!.client.readResource(
                const ReadResourceRequest(uri: 'ui://independent-plugin/main'),
              );
              expect(
                (ui.contents.single as TextResourceContents).text,
                contains('TCK Workspace edit'),
              );
              final providers = await environment.providers();
              final results = <String, Map<String, Object?>>{};
              final gateway = McpCapabilityGateway(
                describeHost: () => environment.identity,
                listProviders: environment.providers,
                chooseResource: (title, mime) async {
                  final slides = mime.contains('presentationml');
                  final fixture = File(
                    slides
                        ? Platform.environment['OPENMUSE_TEST_PPTX'] ??
                              p.join(
                                ecosystem,
                                'plugins/genoffice-pptx/dist/sample.pptx',
                              )
                        : p.join(
                            ecosystem,
                            'plugins/pdf-reader/fixtures/quarterly-brief.pdf',
                          ),
                  );
                  return GrantedResource(
                    title: p.basename(fixture.path),
                    mediaType: mime,
                    bytes: await fixture.readAsBytes(),
                  );
                },
                approve: (_, _) async => true,
                openDevelopment: (_) async => {},
                readDevelopment: (_) async => {},
                scope: () => account,
              );
              try {
                for (final provider in providers) {
                  final contract = provider.contracts.single,
                      tool =
                          (contract['operations'] as Map)['readSnapshot']
                              as String;
                  final isSlides = provider.id.endsWith('genoffice-pptx');
                  final resource = await gateway.call('resources_choose', {
                    'title': 'TCK fixture',
                    'mediaType': contract['mediaType'],
                  });
                  final binding = await gateway.call('capabilities_bind', {
                    'provider': provider.id,
                    'contract': contract['id'],
                    'resourceRef': resource['resourceRef'],
                    'tools': [tool],
                  });
                  final Map<String, Object?> result;
                  try {
                    result = await gateway.call('capabilities_call', {
                      'binding': binding['binding'],
                      'tool': tool,
                      'arguments': {},
                    });
                  } on JsonSchemaValidationException catch (error) {
                    // Schema diagnostics contain no resource values.
                    // ignore: avoid_print
                    print('${provider.id}: ${error.message} at ${error.path}');
                    rethrow;
                  }
                  final output = Map<String, Object?>.from(
                    result['structuredContent'] as Map,
                  );
                  expect(output['pageCount'], isSlides ? 23 : 2);
                  results[provider.id] = output;
                }
                final source = await File(
                  p.join(workspace.path, 'main.js'),
                ).readAsBytes();
                final report = {
                  'status': 'passed',
                  'codeSha256': sha256.convert(source).toString(),
                  'completedAt': DateTime.now().toUtc().toIso8601String(),
                  'providers': [
                    for (final provider in providers)
                      {
                        'id': provider.id,
                        'version': provider.version,
                        'sha256': provider.digest,
                        'pageCount': results[provider.id]!['pageCount'],
                        'source': results[provider.id]!['source'],
                      },
                  ],
                };
                final recorded = await candidate!.client.callTool(
                  CallToolRequest(
                    name: 'record_run',
                    arguments: {'report': report},
                  ),
                );
                expect(recorded.isError, isFalse);
              } finally {
                await gateway.close();
                for (final provider in providers) {
                  await provider.close?.call();
                }
              }
              return {
                'opened': false,
                'runRef': 'headless-tck',
                'sha256': candidateDigest,
                'channel': 'headless-protocol-tck',
              };
            case 'development_read_resource':
              return (await candidate!.client.readResource(
                ReadResourceRequest(uri: args['uri'] as String),
              )).toJson();
            default:
              throw StateError('Unexpected TCK call');
          }
        },
      );
      await service.start();
      final credentials = service.issue(workspace.path);
      final cli = await verifiedMcpBin(market, kit.name, 'muse');
      final launcher = Platform.environment['OPENMUSE_TEST_CLI_LAUNCHER'];
      String? shim;
      if (launcher != null) {
        shim = p.join(
          await refreshMcpBinShims(
            market,
            Directory(p.join(root.path, 'bin')),
            launcher,
          ),
          'muse',
        );
      }
      Future<ProcessResult> run(List<String> args, {String? directory}) =>
          Process.run(
            shim ?? node,
            [if (shim == null) cli.path, ...args],
            workingDirectory: directory ?? workspace.path,
            environment: credentials,
          );
      try {
        var result = await run(['search', 'pptx']);
        expect(result.exitCode, 0, reason: '${result.stderr}');
        expect('${result.stdout}', contains('genoffice-pptx'));
        expect(
          '${result.stdout}',
          contains('contracts/genoffice/slides-read/v1'),
        );
        result = await run([
          'init',
          '--name',
          'workspace-explainer',
          '--capability',
          'pptx',
          '--capability',
          'pdf',
        ]);
        expect(result.exitCode, 0, reason: '${result.stderr}');
        expect(
          await File(p.join(workspace.path, 'existing.txt')).readAsString(),
          'Keep existing workspace files',
        );
        final lock =
            jsonDecode(
                  await File(
                    p.join(workspace.path, 'devkit.lock.json'),
                  ).readAsString(),
                )
                as Map;
        expect(lock['dependencies'], hasLength(2));
        for (final (schema, document) in [
          ('lock', lock),
          (
            'project',
            jsonDecode(
              await File(
                p.join(workspace.path, 'devkit.project.json'),
              ).readAsString(),
            ),
          ),
        ]) {
          JsonSchema.fromJson(
            jsonDecode(
                  await File(
                    p.join(
                      ecosystem,
                      'contracts/devkit-v1/$schema.schema.json',
                    ),
                  ).readAsString(),
                )
                as Map<String, dynamic>,
          ).validate(document);
        }
        final source = File(p.join(workspace.path, 'main.js'));
        await source.writeAsString(
          '${await source.readAsString()}\n// TCK Workspace edit\n',
        );
        result = await run(['check']);
        expect(result.exitCode, 0, reason: '${result.stderr}');
        result = await run(['dev']);
        expect(result.exitCode, 0, reason: '${result.stderr}');
        expect('${result.stdout}', contains('headless-protocol-tck'));
        result = await run(['report']);
        expect(result.exitCode, 0, reason: '${result.stderr}');
        result = await run(['pack']);
        expect(result.exitCode, 0, reason: '${result.stderr}');
        expect('${result.stdout}', contains(candidateDigest!));
        result = await run([
          'init',
          '--name',
          'workspace-explainer',
          '--capability',
          'pptx',
        ]);
        expect(result.exitCode, isNot(0));
        expect(await source.readAsString(), contains('TCK Workspace edit'));
        final elsewhere = await Directory(
          p.join(root.path, 'outside'),
        ).create();
        result = await run(['check'], directory: elsewhere.path);
        expect(result.exitCode, isNot(0));
        expect('${result.stderr}', contains('Workspace'));
        account = 'different-account';
        result = await run(['search', 'pdf']);
        expect(result.exitCode, isNot(0));
        expect(
          trace,
          containsAll([
            'workspace_describe',
            'capabilities_list',
            'development_open_bundle',
            'development_read_resource',
          ]),
        );
        // A command must stop before execution when an installed release was
        // modified, rather than execute unreviewed local replacement bytes.
        await cli.writeAsString('// changed', mode: FileMode.append);
        await expectLater(
          verifiedMcpBin(market, kit.name, 'muse'),
          throwsStateError,
        );
      } finally {
        await candidate?.close();
        await service.close();
        await root.delete(recursive: true);
      }
    },
    timeout: const Timeout(Duration(minutes: 3)),
    skip: Platform.environment['OPENMUSE_ECOSYSTEM_TEST_ROOT'] == null
        ? 'Optional external plugin release fixtures require OPENMUSE_ECOSYSTEM_TEST_ROOT'
        : false,
  );
}
