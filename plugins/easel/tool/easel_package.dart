import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:archive/archive.dart';
import 'package:crypto/crypto.dart';

final class EaselPluginPackage {
  const EaselPluginPackage({required this.bytes, required this.sha256});

  final List<int> bytes;
  final String sha256;

  static const fileName = 'com.openmuse.easel-0.4.5.omplugin';
  static const pluginId = 'com.openmuse.easel';
  static const version = '0.4.5';
}

/// Packs the supplied Easel source tree into an optional installable package.
EaselPluginPackage buildEaselPackage(
  Directory easelRoot,
  Directory pluginRoot, {
  bool includeInstallHook = true,
}) {
  final script = _read(easelRoot, 'skills/shared/scripts/douyin_publish.py');
  final skills = {
    'shared/scripts/douyin_publish.py': script,
    'shared/scripts/web_publisher.py': _read(
      easelRoot,
      'skills/shared/scripts/web_publisher.py',
    ),
    for (final module in [
      'login_state.py',
      'content_guard.py',
      'platform_readback.py',
      'human_pace.py',
    ])
      'shared/scripts/$module': _read(
        easelRoot,
        'skills/shared/scripts/$module',
      ),
    'runtime/install.py': _read(pluginRoot, 'runtime/install.py'),
    'runtime/easel_cli.py': _read(pluginRoot, 'runtime/easel_cli.py'),
    'runtime/easel_social.py': _read(pluginRoot, 'runtime/easel_social.py'),
    'runtime/easel_article.py': _read(pluginRoot, 'runtime/easel_article.py'),
    'runtime/easel_zhihu_article.py': _read(pluginRoot, 'runtime/easel_zhihu_article.py'),
    'runtime/requirements.txt': _read(pluginRoot, 'runtime/requirements.txt'),
    'runtime/openmuse-icon.png': _read(pluginRoot, 'runtime/openmuse-icon.png'),
  };
  final targetJson = _targetJson();
  Map<String, Object?> artifact(String id, String kind, String digest) => {
    'id': id,
    'kind': kind,
    'target': targetJson,
    'digest': {'algorithm': 'sha256', 'value': digest},
    'license': 'Apache-2.0',
    'abi': 'openmuse.easel/v1',
  };
  final unsupported = [
    for (final item in [
      ['macos', 'aarch64', 'darwin'],
      ['macos', 'x86_64', 'darwin'],
      ['windows', 'x86_64', 'msvc'],
      ['windows', 'aarch64', 'msvc'],
      ['linux', 'x86_64', 'gnu'],
      ['linux', 'aarch64', 'gnu'],
      ['android', 'aarch64', 'bionic'],
      ['ios', 'aarch64', 'darwin'],
    ])
      if (item[0] != targetJson['os'] ||
          item[1] != targetJson['arch'] ||
          item[2] != targetJson['libc'])
        {
          'target': {'os': item[0], 'arch': item[1], 'libc': item[2]},
          'status': 'unsupported',
          'reason': 'Easel runs on the installing desktop only',
        },
  ];
  final manifest = {
    'manifest_version': 2,
    'id': EaselPluginPackage.pluginId,
    'name': 'Easel',
    'version': EaselPluginPackage.version,
    'protocol': {'major': 1, 'minor': 0},
    'ui_runtime': {'kind': 'none'},
    'execution_connector': {
      'kind': 'host-process',
      'protocol': 'openmuse.cli-python/v1',
    },
    'compatibility': {
      'targets': [
        {'target': targetJson, 'status': 'supported'},
        ...unsupported,
      ],
    },
    'artifacts': [
      artifact('easel-skills', 'runtime-closure', digestPayload(skills)),
    ],
    'activation_events': ['onStartup'],
    'requested_permissions': ['workspace.context.read'],
    'presentation': {'surfaces': <String>[], 'remote_capable': false},
    'contributes': {
      'cli': [
        for (final command in [
          'plan',
          'selftest',
          'check',
          'login',
          'whoami',
          'publish-video',
        ])
          {
            'group': 'easel',
            'namespace': 'douyin',
            'command': command,
            'artifact': 'easel-skills',
            'entrypoint': 'runtime/easel_social.py',
            'runtime': 'python3',
            'argv_prefix': ['douyin', command],
            'options': switch (command) {
              'plan' => [
                '--title',
                '--content',
                '--images',
                '--video',
                '--tags',
              ],
              'login' => [
                '--qr-out',
                '--status-file',
                '--timeout',
                '--profile-base',
              ],
              'whoami' => ['--profile-base'],
              'publish-video' => [
                '--title',
                '--content',
                '--video',
                '--tags',
                '--profile-base',
                '--status-file',
              ],
              _ => <String>[],
            },
            'switches': command == 'publish-video'
                ? ['--exec', '--headed']
                : <String>[],
            'description': switch (command) {
              'login' => '启动抖音扫码授权；默认在 OpenMuse 弹出二维码并输出授权状态',
              'whoami' => '检查当前抖音授权账号，输出 JSON',
              'publish-video' => '上传视频到抖音；--exec 才真正发布；输出发布状态与读回结果',
              'plan' => '离线检查发布素材并生成步骤预览',
              'check' => '检查 Playwright 与浏览器内核',
              _ => '离线自检',
            },
            'input_schema': {
              'type': 'object',
              'required': command == 'publish-video'
                  ? ['--title', '--video']
                  : <String>[],
              'properties': {
                for (final option in switch (command) {
                  'plan' => [
                    '--title',
                    '--content',
                    '--images',
                    '--video',
                    '--tags',
                  ],
                  'login' => [
                    '--qr-out',
                    '--status-file',
                    '--timeout',
                    '--profile-base',
                  ],
                  'whoami' => ['--profile-base'],
                  'publish-video' => [
                    '--title',
                    '--content',
                    '--video',
                    '--tags',
                    '--profile-base',
                    '--status-file',
                  ],
                  _ => <String>[],
                })
                  option: {'type': 'string'},
              },
            },
            'output_schema': switch (command) {
              'whoami' => {
                'contentType': 'application/json',
                'fields': ['loggedIn', 'name', 'avatar', 'error'],
              },
              'login' => {
                'contentType': 'text/plain',
                'interaction': 'openmuse.plugin-interaction/v1 image.challenge',
                'statusFile': 'JSON login state',
              },
              'publish-video' => {
                'contentType': 'text/plain',
                'exitCode': '0 after publish readback',
              },
              _ => {'contentType': 'text/plain'},
            },
            'effects': command == 'publish-video'
                ? ['workspace.read', 'network.publish']
                : ['workspace.read'],
          },
        for (final command in [
          'plan',
          'check',
          'selftest',
          'whoami',
          'login',
          'publish',
        ])
          {
            'group': 'easel',
            'namespace': 'zhihu',
            'command': command,
            'artifact': 'easel-skills',
            'entrypoint': 'runtime/easel_social.py',
            'runtime': 'python3',
            'argv_prefix': ['zhihu', command],
            'options': switch (command) {
              'plan' || 'publish' => ['--title', '--desc', '--tags'],
              'login' => ['--timeout'],
              _ => <String>[],
            },
            'switches': command == 'publish'
                ? ['--exec', '--headed']
                : <String>[],
            'description': switch (command) {
              'plan' => '知乎专栏纯文本文章发布步骤预览；当前不上传正文内嵌图片',
              'publish' => '知乎专栏纯文本文章发布；--exec 才提交。当前不上传正文内嵌图片，图文稿请先保留原稿',
              'login' => '启动知乎扫码授权；默认在 OpenMuse 弹出二维码并输出授权状态',
              'whoami' => '检查知乎授权账号，输出 JSON',
              'check' => '检查知乎发布所需 Playwright 与 Chromium',
              _ => '离线检查知乎发布配置',
            },
            'input_schema': {
              'type': 'object',
              'required': command == 'publish'
                  ? ['--title', '--desc']
                  : <String>[],
              'properties': {
                for (final option in switch (command) {
                  'plan' || 'publish' => ['--title', '--desc', '--tags'],
                  'login' => ['--timeout'],
                  _ => <String>[],
                })
                  option: {'type': 'string'},
              },
            },
            'output_schema': command == 'login'
                ? {
                    'contentType': 'text/plain',
                    'interaction':
                        'openmuse.plugin-interaction/v1 image.challenge',
                    'statusFile': 'JSON login state',
                  }
                : {'contentType': 'text/plain', 'exitCode': 'integer'},
            'effects': command == 'publish'
                ? ['workspace.read', 'network.publish']
                : ['workspace.read'],
          },
        {
          'group': 'easel',
          'namespace': 'zhihu',
          'command': 'inspect',
          'artifact': 'easel-skills',
          'entrypoint': 'runtime/easel_article.py',
          'runtime': 'python3',
          'argv_prefix': ['inspect'],
          'options': ['--article'],
          'description': '检查知乎 Markdown 图文稿和本地图片；输出 JSON 就绪状态及内嵌图片发布阻断原因',
          'input_schema': {
            'type': 'object',
            'required': ['--article'],
            'properties': {
              '--article': {'type': 'string', 'format': 'absolute-file-path'},
            },
          },
          'output_schema': {
            'contentType': 'application/json',
            'fields': [
              'stage',
              'article',
              'title',
              'imageCount',
              'images',
              'missingImages',
              'canPublishRequestedArticle',
              'blockingIssue',
            ],
          },
          'effects': ['workspace.read'],
        },
        {
          'group': 'easel',
          'namespace': 'zhihu',
          'command': 'publish-article',
          'artifact': 'easel-skills',
          'entrypoint': 'runtime/easel_zhihu_article.py',
          'runtime': 'python3',
          'argv_prefix': <String>[],
          'options': ['--article'],
          'switches': ['--exec', '--force'],
          'description': '将 Markdown 与本地图片按顺序写入知乎专栏；--exec 发布并记录回执，--force 才允许重复提交',
          'input_schema': {
            'type': 'object',
            'required': ['--article'],
            'properties': {
              '--article': {'type': 'string', 'format': 'absolute-file-path'},
            },
          },
          'output_schema': {
            'contentType': 'application/x-ndjson',
            'fields': ['stage', 'title', 'imageCount', 'url'],
          },
          'effects': ['workspace.read', 'network.publish'],
        },
        {
          'group': 'easel',
          'namespace': 'video',
          'command': 'process',
          'artifact': 'easel-skills',
          'entrypoint': 'runtime/easel_cli.py',
          'runtime': 'python3',
          'argv_prefix': ['process'],
          'options': ['--input', '--output', '--cover'],
          'description': '将输入视频加工为带 OpenMuse 封面片头的视频，并输出封面 PNG 与视频路径 JSON',
          'input_schema': {
            'type': 'object',
            'required': ['--input', '--output'],
            'properties': {
              '--input': {'type': 'string', 'format': 'absolute-file-path'},
              '--output': {'type': 'string', 'format': 'absolute-file-path'},
              '--cover': {'type': 'string', 'format': 'absolute-file-path'},
            },
          },
          'output_schema': {
            'contentType': 'application/x-ndjson',
            'stages': ['rendering', 'ready', 'error'],
            'readyFields': ['video', 'cover', 'input'],
          },
          'effects': ['workspace.read', 'workspace.write'],
        },
      ],
    },
    'install': {
      'steps': [
        {
          'type': 'workspace.choose',
          'id': 'workspace',
          'title': '选择 Easel 工作区',
        },
        {
          'type': 'workspace.layout',
          'directories': ['profiles', 'cache', 'outputs', 'state', 'runtime'],
        },
        {
          'type': includeInstallHook
              ? 'runtime.prepare'
              : 'workspace.materialize',
          'artifact': 'easel-skills',
          'into': 'skills',
          if (includeInstallHook) 'entrypoint': 'runtime/install.py',
          if (includeInstallHook) 'runtime': 'python3',
        },
      ],
    },
  };
  final archive = Archive();
  void add(String name, List<int> bytes) {
    archive.addFile(ArchiveFile(name, bytes.length, bytes));
  }

  add('openmuse.plugin.json', utf8.encode(jsonEncode(manifest)));
  for (final entry in skills.entries) {
    add('payload/easel-skills/${entry.key}', entry.value);
  }
  // archive 3.x returns nullable bytes; archive 4.x returns non-null bytes.
  // ignore: unnecessary_non_null_assertion
  final bytes = ZipEncoder().encode(archive)!;
  return EaselPluginPackage(
    bytes: bytes,
    sha256: sha256.convert(bytes).toString(),
  );
}

Map<String, String> _targetJson() => switch (Abi.current()) {
  Abi.macosArm64 => {'os': 'macos', 'arch': 'aarch64', 'libc': 'darwin'},
  Abi.macosX64 => {'os': 'macos', 'arch': 'x86_64', 'libc': 'darwin'},
  Abi.linuxArm64 => {'os': 'linux', 'arch': 'aarch64', 'libc': 'gnu'},
  Abi.linuxX64 => {'os': 'linux', 'arch': 'x86_64', 'libc': 'gnu'},
  Abi.windowsArm64 => {'os': 'windows', 'arch': 'aarch64', 'libc': 'msvc'},
  Abi.windowsX64 => {'os': 'windows', 'arch': 'x86_64', 'libc': 'msvc'},
  _ => throw UnsupportedError('Unsupported Easel target: ${Abi.current()}'),
};

String digestPayload(Map<String, List<int>> files) {
  final builder = BytesBuilder(copy: false);
  for (final path in files.keys.toList()..sort()) {
    builder.add(utf8.encode(path));
    builder.add(const [0]);
    builder.add(files[path]!);
    builder.add(const [0]);
  }
  return sha256.convert(builder.takeBytes()).toString();
}

void writeEaselPackage(Directory output, EaselPluginPackage package) {
  output.createSync(recursive: true);
  File(
    '${output.path}/${EaselPluginPackage.fileName}',
  ).writeAsBytesSync(package.bytes, flush: true);
  File('${output.path}/catalog.json').writeAsStringSync(
    jsonEncode({
      'packages': [
        {
          'id': EaselPluginPackage.pluginId,
          'version': EaselPluginPackage.version,
          'url': EaselPluginPackage.fileName,
          'sha256': package.sha256,
          'size': package.bytes.length,
        },
      ],
    }),
    flush: true,
  );
}

List<int> _read(Directory root, String relative) {
  final file = File('${root.path}/$relative');
  if (!file.existsSync()) {
    throw FileSystemException('Easel 源文件不存在', file.path);
  }
  return file.readAsBytesSync();
}
