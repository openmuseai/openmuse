import 'package:appflowy/plugins/resource_surface/engine_registry.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:flutter/material.dart';

/// Formats iOffice Word itself can open.
const iofficeSupportedExtensions = <String>{'docx'};

const iofficeEngineSpec = MuseResourceEngineSpec(
  engine: MuseLocalEngine.ioffice,
  id: 'muse.ioffice',
  label: 'iOffice',
  icon: Icons.description_outlined,
  extensions: iofficeSupportedExtensions,
  priority: 30,
);
