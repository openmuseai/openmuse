import 'package:appflowy/plugins/resource_surface/engine_registry.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:flutter/material.dart';

/// Formats Helix itself can open. Keep this next to the Helix surface.
const helixSupportedExtensions = <String>{
  'c',
  'cc',
  'cpp',
  'cs',
  'css',
  'dart',
  'diff',
  'go',
  'h',
  'hpp',
  'ini',
  'java',
  'js',
  'json',
  'jsx',
  'kt',
  'log',
  'lua',
  'md',
  'mjs',
  'py',
  'rb',
  'rs',
  'scss',
  'sh',
  'sql',
  'swift',
  'toml',
  'ts',
  'tsx',
  'txt',
  'xml',
  'yaml',
  'yml',
  'zsh',
};

const helixEngineSpec = MuseResourceEngineSpec(
  engine: MuseLocalEngine.helix,
  id: 'muse.helix',
  label: 'Helix',
  icon: Icons.code,
  extensions: helixSupportedExtensions,
  priority: 20,
);
