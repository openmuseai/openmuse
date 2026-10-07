import 'parse.dart';

final _calendarDate = RegExp(r'^\d{4}-\d{2}-\d{2}$');

final class RemoteClientHello {
  RemoteClientHello({
    required this.protocolMajor,
    required this.protocolMinor,
    required List<String> components,
    required List<String> mediaFormats,
    required this.webSnapshot,
    required this.webInteractive,
    required this.maxControlBytes,
  }) : components = List<String>.unmodifiable(components),
       mediaFormats = List<String>.unmodifiable(mediaFormats) {
    if (protocolMajor != RemoteSurfaceLimits.protocolMajor ||
        protocolMinor != RemoteSurfaceLimits.protocolMinor) {
      throw const RemoteSurfaceFormatException(
        'incompatible remote surface protocol',
      );
    }
    if (components.isEmpty ||
        components.length != components.toSet().length ||
        components.any((item) => !remoteSurfaceComponents.contains(item))) {
      throw const RemoteSurfaceFormatException(
        'components must be unique known values',
      );
    }
    if (mediaFormats.isEmpty ||
        mediaFormats.length != mediaFormats.toSet().length ||
        mediaFormats.any((item) => !remoteMediaFormats.contains(item))) {
      throw const RemoteSurfaceFormatException(
        'mediaFormats must be unique known values',
      );
    }
    if (maxControlBytes < 1024 ||
        maxControlBytes > RemoteSurfaceLimits.maxControlBytes) {
      throw const RemoteSurfaceFormatException(
        'maxControlBytes is outside the supported range',
      );
    }
  }

  static final mobileV1 = RemoteClientHello(
    protocolMajor: RemoteSurfaceLimits.protocolMajor,
    protocolMinor: RemoteSurfaceLimits.protocolMinor,
    components: remoteSurfaceComponents.toList()..sort(),
    mediaFormats: remoteMediaFormats.toList()..sort(),
    webSnapshot: true,
    webInteractive: false,
    maxControlBytes: RemoteSurfaceLimits.maxControlBytes,
  );

  final int protocolMajor;
  final int protocolMinor;
  final List<String> components;
  final List<String> mediaFormats;
  final bool webSnapshot;
  final bool webInteractive;
  final int maxControlBytes;

  factory RemoteClientHello.fromJson(Map<String, Object?> map) {
    requireProtocol(map, remoteSurfaceHelloProtocol);
    requireKeys(
      map,
      required: {
        'protocol',
        'protocolMajor',
        'protocolMinor',
        'components',
        'mediaFormats',
        'webSnapshot',
        'webInteractive',
        'maxControlBytes',
      },
    );
    final components = stringList(map, 'components', max: 32);
    final mediaFormats = stringList(map, 'mediaFormats', max: 8);
    return RemoteClientHello(
      protocolMajor: intField(map, 'protocolMajor', min: 0, max: 32),
      protocolMinor: intField(map, 'protocolMinor', min: 0, max: 32),
      components: components,
      mediaFormats: mediaFormats,
      webSnapshot: boolField(map, 'webSnapshot'),
      webInteractive: boolField(map, 'webInteractive'),
      maxControlBytes: intField(map, 'maxControlBytes', min: 1),
    );
  }

  Map<String, Object?> toJson() => {
    'protocol': remoteSurfaceHelloProtocol,
    'protocolMajor': protocolMajor,
    'protocolMinor': protocolMinor,
    'components': components,
    'mediaFormats': mediaFormats,
    'webSnapshot': webSnapshot,
    'webInteractive': webInteractive,
    'maxControlBytes': maxControlBytes,
  };
}

String? selectRemoteSurfaceMode(List<String> modes, RemoteClientHello hello) {
  const order = ['declarative', 'media', 'web-snapshot', 'web-interactive'];
  for (final mode in order) {
    if (!modes.contains(mode)) continue;
    if (mode == 'web-snapshot' && !hello.webSnapshot) continue;
    if (mode == 'web-interactive' && !hello.webInteractive) continue;
    if (mode == 'media' && hello.mediaFormats.isEmpty) continue;
    return mode;
  }
  return null;
}

final class RemoteSurfaceAction {
  RemoteSurfaceAction({
    required this.id,
    required this.effect,
    required List<String> requiredPermissions,
    required Map<String, Object?> inputSchema,
  }) : requiredPermissions = List<String>.unmodifiable(requiredPermissions),
       inputSchema = Map<String, Object?>.unmodifiable(
         deepCopyJson(inputSchema)! as Map<String, Object?>,
       ) {
    actionIdValue(id, 'action.id');
    if (requiredPermissions.isEmpty) {
      throw const RemoteSurfaceFormatException(
        'action permissions must not be empty',
      );
    }
    if (!remoteActionEffects.contains(effect)) {
      throw const RemoteSurfaceFormatException('action effect is unsupported');
    }
    validateInputSchema(this.inputSchema);
  }

  final String id;
  final String effect;
  final List<String> requiredPermissions;
  final Map<String, Object?> inputSchema;

  factory RemoteSurfaceAction.fromJson(Object? value) {
    final map = objectMap(value, 'action');
    requireKeys(
      map,
      required: {'id', 'effect', 'requiredPermissions', 'inputSchema'},
    );
    return RemoteSurfaceAction(
      id: actionIdValue(stringField(map, 'id', max: 81), 'action.id'),
      effect: stringField(map, 'effect', max: 32),
      requiredPermissions: permissionList(map, 'requiredPermissions'),
      inputSchema: objectMap(map['inputSchema'], 'inputSchema'),
    );
  }

  Map<String, Object?> toJson() => {
    'id': id,
    'effect': effect,
    'requiredPermissions': requiredPermissions,
    'inputSchema': deepCopyJson(inputSchema),
  };
}

void validateInputSchema(Map<String, Object?> schema) {
  requireKeys(
    schema,
    required: {'type', 'additionalProperties', 'required', 'properties'},
  );
  if (schema['type'] != 'object' || schema['additionalProperties'] != false) {
    throw const RemoteSurfaceFormatException(
      'inputSchema must be a closed object',
    );
  }
  final requiredFields = stringList(
    schema,
    'required',
    max: 16,
    allowEmpty: true,
  );
  final properties = objectMap(schema['properties'], 'properties');
  if (properties.length > 16) {
    throw const RemoteSurfaceFormatException('inputSchema has too many fields');
  }
  for (final name in properties.keys) {
    fieldIdValue(name, 'inputSchema.properties');
  }
  for (final field in requiredFields) {
    fieldIdValue(field, 'inputSchema.required');
    if (!properties.containsKey(field)) {
      throw RemoteSurfaceFormatException(
        'inputSchema.required contains unknown field $field',
      );
    }
  }
  for (final entry in properties.entries) {
    final property = objectMap(entry.value, 'inputSchema.properties');
    final type = stringField(property, 'type', max: 16);
    switch (type) {
      case 'string':
        requireKeys(
          property,
          required: {'type'},
          optional: {'minLength', 'maxLength'},
        );
        final minLength = property.containsKey('minLength')
            ? intField(property, 'minLength', min: 0, max: 2000)
            : 0;
        final maxLength = property.containsKey('maxLength')
            ? intField(property, 'maxLength', min: 0, max: 2000)
            : RemoteSurfaceLimits.maxTextLength;
        if (minLength > maxLength) {
          throw const RemoteSurfaceFormatException(
            'inputSchema string bounds are inverted',
          );
        }
      case 'integer':
        requireKeys(
          property,
          required: {'type'},
          optional: {'minimum', 'maximum'},
        );
        final minimum = property.containsKey('minimum')
            ? intField(property, 'minimum', min: -9007199254740991)
            : null;
        final maximum = property.containsKey('maximum')
            ? intField(property, 'maximum', min: -9007199254740991)
            : null;
        if (minimum != null && maximum != null && minimum > maximum) {
          throw const RemoteSurfaceFormatException(
            'inputSchema integer bounds are inverted',
          );
        }
      case 'boolean':
        requireKeys(property, required: {'type'});
      default:
        throw const RemoteSurfaceFormatException(
          'inputSchema property type is unsupported',
        );
    }
  }
}

bool inputMatchesSchema(
  Map<String, Object?> schema,
  Map<String, Object?> input,
) {
  final requiredFields = (schema['required']! as List).cast<String>();
  final properties = (schema['properties']! as Map).cast<String, Object?>();
  for (final key in input.keys) {
    if (!properties.containsKey(key)) return false;
  }
  for (final field in requiredFields) {
    if (!input.containsKey(field)) return false;
  }
  for (final entry in input.entries) {
    final property = objectMap(properties[entry.key], 'input');
    final type = property['type'];
    final value = entry.value;
    if (type == 'string') {
      if (value is! String) return false;
      final minLength = property['minLength'] as int? ?? 0;
      final maxLength =
          property['maxLength'] as int? ?? RemoteSurfaceLimits.maxTextLength;
      if (value.length < minLength || value.length > maxLength) return false;
    } else if (type == 'integer') {
      if (value is! int) return false;
      final minimum = property['minimum'] as int?;
      final maximum = property['maximum'] as int?;
      if (minimum != null && value < minimum) return false;
      if (maximum != null && value > maximum) return false;
    } else if (type == 'boolean') {
      if (value is! bool) return false;
    } else {
      return false;
    }
  }
  return true;
}

final class RemoteSurfaceDescriptor {
  RemoteSurfaceDescriptor({
    required this.pluginId,
    required this.surfaceId,
    required this.title,
    required this.workspaceRef,
    required List<String> modes,
    required List<String> requiredCapabilities,
    required List<String> readPermissions,
    required List<RemoteSurfaceAction> actions,
  }) : modes = List<String>.unmodifiable(modes),
       requiredCapabilities = List<String>.unmodifiable(requiredCapabilities),
       readPermissions = List<String>.unmodifiable(readPermissions),
       actions = List<RemoteSurfaceAction>.unmodifiable(actions) {
    pluginIdField({'pluginId': pluginId}, 'pluginId');
    surfaceIdField({'surfaceId': surfaceId}, 'surfaceId');
    rejectDangerousText(title, 'title', max: 80);
    opaqueField({'workspaceRef': workspaceRef}, 'workspaceRef');
    if (modes.isEmpty ||
        modes.toSet().length != modes.length ||
        modes.any((mode) => !remoteSurfaceModes.contains(mode))) {
      throw const RemoteSurfaceFormatException(
        'modes must be unique known values',
      );
    }
    if (requiredCapabilities.toSet().length != requiredCapabilities.length ||
        requiredCapabilities.any(
          (item) => !remoteSurfaceComponents.contains(item),
        )) {
      throw const RemoteSurfaceFormatException(
        'requiredCapabilities must be unique known components',
      );
    }
    if (readPermissions.isEmpty) {
      throw const RemoteSurfaceFormatException(
        'read permissions must not be empty',
      );
    }
    if (actions.length > 32 ||
        actions.map((action) => action.id).toSet().length != actions.length) {
      throw const RemoteSurfaceFormatException(
        'actions must be unique and bounded',
      );
    }
  }

  final String pluginId;
  final String surfaceId;
  final String title;
  final String workspaceRef;
  final List<String> modes;
  final List<String> requiredCapabilities;
  final List<String> readPermissions;
  final List<RemoteSurfaceAction> actions;

  RemoteSurfaceAction? action(String id) {
    for (final item in actions) {
      if (item.id == id) return item;
    }
    return null;
  }

  factory RemoteSurfaceDescriptor.fromJson(Map<String, Object?> map) {
    requireProtocol(map, remoteSurfaceDescriptorProtocol);
    requireKeys(
      map,
      required: {
        'protocol',
        'pluginId',
        'surfaceId',
        'title',
        'workspaceRef',
        'modes',
        'requiredCapabilities',
        'readPermissions',
        'actions',
      },
    );
    final modes = stringList(map, 'modes', max: 4);
    final capabilities = stringList(
      map,
      'requiredCapabilities',
      max: 18,
      allowEmpty: true,
    );
    return RemoteSurfaceDescriptor(
      pluginId: pluginIdField(map, 'pluginId'),
      surfaceId: surfaceIdField(map, 'surfaceId'),
      title: stringField(map, 'title', max: 80),
      workspaceRef: opaqueField(map, 'workspaceRef'),
      modes: modes,
      requiredCapabilities: capabilities,
      readPermissions: permissionList(map, 'readPermissions'),
      actions: objectList(
        map['actions'],
        'actions',
      ).map(RemoteSurfaceAction.fromJson).toList(growable: false),
    );
  }

  Map<String, Object?> toJson() => {
    'protocol': remoteSurfaceDescriptorProtocol,
    'pluginId': pluginId,
    'surfaceId': surfaceId,
    'title': title,
    'workspaceRef': workspaceRef,
    'modes': modes,
    'requiredCapabilities': requiredCapabilities,
    'readPermissions': readPermissions,
    'actions': actions.map((action) => action.toJson()).toList(),
  };
}

final class RemoteSurfaceNode {
  RemoteSurfaceNode({
    required this.nodeId,
    required this.type,
    required this.requiredNode,
    required Map<String, Object?> props,
    required List<RemoteSurfaceNode> children,
  }) : props = Map<String, Object?>.unmodifiable(props),
       children = List<RemoteSurfaceNode>.unmodifiable(children);

  final String nodeId;
  final String type;
  final bool requiredNode;
  final Map<String, Object?> props;
  final List<RemoteSurfaceNode> children;

  Map<String, Object?> toJson() => {
    'nodeId': nodeId,
    'type': type,
    'required': requiredNode,
    'props': deepCopyJson(props),
    'children': children.map((child) => child.toJson()).toList(),
  };
}

RemoteSurfaceNode parseNode(Object? value, {required int depth}) {
  if (depth > RemoteSurfaceLimits.maxDepth) {
    throw const RemoteSurfaceFormatException('component tree is too deep');
  }
  final map = objectMap(value, 'node');
  requireKeys(
    map,
    required: {'nodeId', 'type', 'required', 'props', 'children'},
  );
  final type = componentTypeValue(stringField(map, 'type', max: 33), 'type');
  final requiredNode = boolField(map, 'required');
  if (requiredNode && !remoteSurfaceComponents.contains(type)) {
    throw const RemoteSurfaceFormatException(
      'required component is not in the v1 vocabulary',
      code: 'UNSUPPORTED_COMPONENT',
    );
  }
  final props = _parseProps(type, objectMap(map['props'], 'props'));
  final children = objectList(
    map['children'],
    'children',
  ).map((child) => parseNode(child, depth: depth + 1)).toList(growable: false);
  if (children.length > 32) {
    throw const RemoteSurfaceFormatException('node has too many children');
  }
  return RemoteSurfaceNode(
    nodeId: nodeIdValue(stringField(map, 'nodeId', max: 64), 'nodeId'),
    type: type,
    requiredNode: requiredNode,
    props: props,
    children: children,
  );
}

List<RemoteSurfaceNode> parseNodeList(Object? value) {
  final nodes = objectList(
    value,
    'nodes',
  ).map((node) => parseNode(node, depth: 1)).toList(growable: false);
  final ids = <String>{};
  var count = 0;
  void walk(RemoteSurfaceNode node, int depth) {
    if (depth > RemoteSurfaceLimits.maxDepth) {
      throw const RemoteSurfaceFormatException('component tree is too deep');
    }
    count += 1;
    if (count > RemoteSurfaceLimits.maxNodes) {
      throw const RemoteSurfaceFormatException(
        'component tree has too many nodes',
      );
    }
    if (!ids.add(node.nodeId)) {
      throw const RemoteSurfaceFormatException('duplicate nodeId');
    }
    for (final child in node.children) {
      walk(child, depth + 1);
    }
  }

  for (final node in nodes) {
    walk(node, 1);
  }
  return nodes;
}

void validateNodesForClient(
  List<RemoteSurfaceNode> nodes,
  Iterable<String> components,
) {
  void walk(RemoteSurfaceNode node) {
    if (node.requiredNode && !remoteComponentSupported(node.type, components)) {
      throw const RemoteSurfaceTreeRejection('UNSUPPORTED_COMPONENT');
    }
    for (final child in node.children) {
      walk(child);
    }
  }

  for (final node in nodes) {
    walk(node);
  }
}

Map<String, Object?> _parseProps(String type, Map<String, Object?> props) {
  if (remoteSurfaceComponents.contains(type)) {
    return switch (type) {
      'text' || 'markdown' => _textProps(props),
      'image' => _imageProps(props),
      'gallery' => _galleryProps(props),
      'video-player' => _videoProps(props),
      'document' || 'link' => _labelProps(props, media: type == 'document'),
      'list' || 'grid' => _itemsProps(props),
      'form' => _formProps(props),
      'stepper' => _stepperProps(props),
      'progress' => _progressProps(props),
      'status' => _statusProps(props),
      'diff' || 'compare' => _compareProps(props),
      'timeline-basic' => _timelineProps(props),
      'button' => _buttonProps(props, confirmation: false),
      'confirmation' => _buttonProps(props, confirmation: true),
      _ => throw RemoteSurfaceFormatException('unsupported component $type'),
    };
  }
  return _opaqueProps(props);
}

Map<String, Object?> _textProps(Map<String, Object?> props) {
  requireKeys(props, required: {'text'});
  return {'text': stringField(props, 'text')};
}

Map<String, Object?> _imageProps(Map<String, Object?> props) {
  requireKeys(props, required: {'mediaHandle'}, optional: {'alt'});
  return {
    'mediaHandle': mediaHandleValue(
      stringField(props, 'mediaHandle', max: 128),
      'mediaHandle',
    ),
    if (props.containsKey('alt')) 'alt': stringField(props, 'alt', max: 200),
  };
}

Map<String, Object?> _galleryProps(Map<String, Object?> props) {
  requireKeys(props, required: {'items'});
  final items = objectList(props['items'], 'items');
  if (items.isEmpty || items.length > 12) {
    throw const RemoteSurfaceFormatException('gallery items are out of range');
  }
  return {
    'items': [
      for (final item in items) _imageProps(objectMap(item, 'gallery item')),
    ],
  };
}

Map<String, Object?> _videoProps(Map<String, Object?> props) {
  requireKeys(props, required: {'posterHandle', 'durationMs'});
  return {
    'posterHandle': mediaHandleValue(
      stringField(props, 'posterHandle', max: 128),
      'posterHandle',
    ),
    'durationMs': intField(props, 'durationMs', min: 0),
  };
}

Map<String, Object?> _labelProps(
  Map<String, Object?> props, {
  required bool media,
}) {
  requireKeys(props, required: media ? {'label', 'mediaHandle'} : {'label'});
  return {
    'label': stringField(props, 'label', max: 200),
    if (media)
      'mediaHandle': mediaHandleValue(
        stringField(props, 'mediaHandle', max: 128),
        'mediaHandle',
      ),
  };
}

Map<String, Object?> _itemsProps(Map<String, Object?> props) {
  requireKeys(props, required: {'items'});
  final items = objectList(props['items'], 'items');
  if (items.length > 32) {
    throw const RemoteSurfaceFormatException('items exceed 32');
  }
  return {
    'items': [for (final item in items) _idLabel(objectMap(item, 'item'))],
  };
}

Map<String, Object?> _idLabel(Map<String, Object?> map) {
  requireKeys(map, required: {'id', 'label'});
  return {
    'id': fieldIdValue(stringField(map, 'id', max: 65), 'id'),
    'label': stringField(map, 'label', max: 200),
  };
}

Map<String, Object?> _formProps(Map<String, Object?> props) {
  requireKeys(props, required: {'fields'});
  final fields = objectList(props['fields'], 'fields');
  if (fields.isEmpty || fields.length > 16) {
    throw const RemoteSurfaceFormatException('form fields are out of range');
  }
  final parsed = [for (final field in fields) _formField(field)];
  if (parsed.map((field) => field['id']).toSet().length != parsed.length) {
    throw const RemoteSurfaceFormatException('form field ids must be unique');
  }
  return {'fields': parsed};
}

Map<String, Object?> _formField(Object? value) {
  final map = objectMap(value, 'field');
  final kind = stringField(map, 'kind', max: 16);
  if (!{'text', 'choice', 'switch', 'date'}.contains(kind)) {
    throw const RemoteSurfaceFormatException('form field kind is unsupported');
  }
  final id = fieldIdValue(stringField(map, 'id', max: 65), 'field.id');
  final label = stringField(map, 'label', max: 200);
  switch (kind) {
    case 'text':
      requireKeys(map, required: {'id', 'kind', 'label'}, optional: {'value'});
      final text = optionalString(map, 'value');
      return {'id': id, 'kind': kind, 'label': label, 'value': ?text};
    case 'date':
      requireKeys(map, required: {'id', 'kind', 'label'}, optional: {'value'});
      final date = optionalString(map, 'value', max: 10);
      if (date != null && !_calendarDate.hasMatch(date)) {
        throw const RemoteSurfaceFormatException('date value is invalid');
      }
      return {'id': id, 'kind': kind, 'label': label, 'value': ?date};
    case 'switch':
      requireKeys(map, required: {'id', 'kind', 'label'}, optional: {'value'});
      return {
        'id': id,
        'kind': kind,
        'label': label,
        if (map.containsKey('value')) 'value': boolField(map, 'value'),
      };
    default:
      requireKeys(
        map,
        required: {'id', 'kind', 'label', 'options'},
        optional: {'value'},
      );
      final options = stringList(map, 'options', max: 12);
      final selected = optionalString(map, 'value');
      if (selected != null && !options.contains(selected)) {
        throw const RemoteSurfaceFormatException(
          'choice value is not an option',
        );
      }
      return {
        'id': id,
        'kind': kind,
        'label': label,
        'options': options,
        'value': ?selected,
      };
  }
}

Map<String, Object?> _stepperProps(Map<String, Object?> props) {
  requireKeys(props, required: {'steps'});
  final steps = objectList(props['steps'], 'steps');
  if (steps.isEmpty || steps.length > 12) {
    throw const RemoteSurfaceFormatException('steps are out of range');
  }
  return {
    'steps': [for (final step in steps) _step(objectMap(step, 'step'))],
  };
}

Map<String, Object?> _step(Map<String, Object?> map) {
  requireKeys(map, required: {'id', 'label', 'state'});
  final state = stringField(map, 'state', max: 16);
  if (!{'pending', 'current', 'done'}.contains(state)) {
    throw const RemoteSurfaceFormatException('step state is unsupported');
  }
  return {
    'id': fieldIdValue(stringField(map, 'id', max: 65), 'step.id'),
    'label': stringField(map, 'label', max: 200),
    'state': state,
  };
}

Map<String, Object?> _progressProps(Map<String, Object?> props) {
  requireKeys(props, required: {'label', 'value'});
  return {
    'label': stringField(props, 'label', max: 200),
    'value': intField(props, 'value', min: 0, max: 100),
  };
}

Map<String, Object?> _statusProps(Map<String, Object?> props) {
  requireKeys(props, required: {'label', 'state'});
  final state = stringField(props, 'state', max: 32);
  if (!RegExp(r'^[a-z0-9_-]{1,32}$').hasMatch(state)) {
    throw const RemoteSurfaceFormatException('status state is invalid');
  }
  return {'label': stringField(props, 'label', max: 200), 'state': state};
}

Map<String, Object?> _compareProps(Map<String, Object?> props) {
  requireKeys(props, required: {'before', 'after'});
  return {
    'before': stringField(props, 'before'),
    'after': stringField(props, 'after'),
  };
}

Map<String, Object?> _timelineProps(Map<String, Object?> props) {
  requireKeys(props, required: {'clips'});
  final clips = objectList(props['clips'], 'clips');
  if (clips.isEmpty || clips.length > 32) {
    throw const RemoteSurfaceFormatException('clips are out of range');
  }
  return {
    'clips': [for (final clip in clips) _clip(objectMap(clip, 'clip'))],
  };
}

Map<String, Object?> _clip(Map<String, Object?> map) {
  requireKeys(map, required: {'id', 'label', 'startMs', 'endMs'});
  final start = intField(map, 'startMs', min: 0);
  final end = intField(map, 'endMs', min: 0);
  if (end < start) {
    throw const RemoteSurfaceFormatException('clip bounds are inverted');
  }
  return {
    'id': fieldIdValue(stringField(map, 'id', max: 65), 'clip.id'),
    'label': stringField(map, 'label', max: 200),
    'startMs': start,
    'endMs': end,
  };
}

Map<String, Object?> _buttonProps(
  Map<String, Object?> props, {
  required bool confirmation,
}) {
  requireKeys(
    props,
    required: confirmation
        ? {'label', 'actionId', 'prompt'}
        : {'label', 'actionId'},
    optional: confirmation
        ? {'input', 'inputFromFields'}
        : {'input', 'inputFromFields'},
  );
  final result = <String, Object?>{
    'label': stringField(props, 'label', max: 80),
    'actionId': actionIdValue(
      stringField(props, 'actionId', max: 81),
      'actionId',
    ),
    if (confirmation) 'prompt': stringField(props, 'prompt', max: 500),
  };
  if (props.containsKey('inputFromFields')) {
    result['inputFromFields'] = stringList(
      props,
      'inputFromFields',
      max: 16,
    ).map((field) => fieldIdValue(field, 'inputFromFields')).toList();
  }
  if (props.containsKey('input')) {
    result['input'] = _flatInput(objectMap(props['input'], 'input'));
  }
  return result;
}

Map<String, Object?> _flatInput(Map<String, Object?> input) {
  if (input.length > 16) {
    throw const RemoteSurfaceFormatException(
      'button input has too many fields',
    );
  }
  final result = <String, Object?>{};
  for (final entry in input.entries) {
    fieldIdValue(entry.key, 'input');
    final value = entry.value;
    if (value is String) {
      rejectDangerousText(value, 'input.${entry.key}');
      result[entry.key] = value;
    } else if (value is int) {
      if (value < -9007199254740991 || value > 9007199254740991) {
        throw const RemoteSurfaceFormatException(
          'input integer is out of range',
        );
      }
      result[entry.key] = value;
    } else if (value is bool) {
      result[entry.key] = value;
    } else {
      throw const RemoteSurfaceFormatException(
        'button input values must be flat JSON scalars',
      );
    }
  }
  return result;
}

Map<String, Object?> _opaqueProps(Map<String, Object?> props) {
  if (props.length > 8) {
    throw const RemoteSurfaceFormatException(
      'unknown component has too many props',
    );
  }
  final result = <String, Object?>{};
  for (final entry in props.entries) {
    fieldIdValue(entry.key, 'props');
    final value = entry.value;
    if (value is String) {
      rejectDangerousText(value, 'props.${entry.key}', max: 200);
      result[entry.key] = value;
    } else if (value is bool) {
      result[entry.key] = value;
    } else if (value is int && value >= 0 && value <= 9007199254740991) {
      result[entry.key] = value;
    } else {
      throw const RemoteSurfaceFormatException(
        'unknown component props must be short scalars',
      );
    }
  }
  return result;
}

final class RemoteSurfaceSnapshot {
  RemoteSurfaceSnapshot({
    required this.pluginId,
    required this.surfaceId,
    required this.surfaceSessionRef,
    required this.generation,
    required this.stateRevision,
    required this.mode,
    required List<RemoteSurfaceNode> nodes,
  }) : nodes = List<RemoteSurfaceNode>.unmodifiable(nodes);

  final String pluginId;
  final String surfaceId;
  final String surfaceSessionRef;
  final int generation;
  final String stateRevision;
  final String mode;
  final List<RemoteSurfaceNode> nodes;

  factory RemoteSurfaceSnapshot.fromJson(Map<String, Object?> map) {
    requireProtocol(map, remoteSurfaceSnapshotProtocol);
    requireKeys(
      map,
      required: {
        'protocol',
        'pluginId',
        'surfaceId',
        'surfaceSessionRef',
        'generation',
        'stateRevision',
        'mode',
        'nodes',
      },
    );
    final mode = stringField(map, 'mode', max: 32);
    if (!remoteSurfaceModes.contains(mode)) {
      throw const RemoteSurfaceFormatException('mode is unsupported');
    }
    return RemoteSurfaceSnapshot(
      pluginId: pluginIdField(map, 'pluginId'),
      surfaceId: surfaceIdField(map, 'surfaceId'),
      surfaceSessionRef: opaqueField(map, 'surfaceSessionRef'),
      generation: intField(map, 'generation', min: 1),
      stateRevision: opaqueField(map, 'stateRevision'),
      mode: mode,
      nodes: parseNodeList(map['nodes']),
    );
  }

  Map<String, Object?> toJson() => {
    'protocol': remoteSurfaceSnapshotProtocol,
    'pluginId': pluginId,
    'surfaceId': surfaceId,
    'surfaceSessionRef': surfaceSessionRef,
    'generation': generation,
    'stateRevision': stateRevision,
    'mode': mode,
    'nodes': nodes.map((node) => node.toJson()).toList(),
  };
}

final class RemoteControlRequest {
  RemoteControlRequest({
    required this.requestId,
    required this.surfaceSessionRef,
    required this.generation,
    required this.actionId,
    required Map<String, Object?> input,
    required this.expectedStateRevision,
    required this.idempotencyKey,
    required this.deadlineMs,
  }) : input = Map<String, Object?>.unmodifiable(
         _flatInput(Map<String, Object?>.from(input)),
       );

  final String requestId;
  final String surfaceSessionRef;
  final int generation;
  final String actionId;
  final Map<String, Object?> input;
  final String expectedStateRevision;
  final String idempotencyKey;
  final int deadlineMs;

  factory RemoteControlRequest.fromJson(Map<String, Object?> map) {
    requireProtocol(map, remoteControlRequestProtocol);
    requireKeys(
      map,
      required: {
        'protocol',
        'requestId',
        'surfaceSessionRef',
        'generation',
        'actionId',
        'input',
        'expectedStateRevision',
        'idempotencyKey',
        'deadlineMs',
      },
    );
    return RemoteControlRequest(
      requestId: opaqueField(map, 'requestId'),
      surfaceSessionRef: opaqueField(map, 'surfaceSessionRef'),
      generation: intField(map, 'generation', min: 1),
      actionId: actionIdValue(
        stringField(map, 'actionId', max: 81),
        'actionId',
      ),
      input: objectMap(map['input'], 'input'),
      expectedStateRevision: opaqueField(map, 'expectedStateRevision'),
      idempotencyKey: opaqueField(map, 'idempotencyKey'),
      deadlineMs: intField(map, 'deadlineMs', min: 1, max: 120000),
    );
  }

  Map<String, Object?> toJson() => {
    'protocol': remoteControlRequestProtocol,
    'requestId': requestId,
    'surfaceSessionRef': surfaceSessionRef,
    'generation': generation,
    'actionId': actionId,
    'input': deepCopyJson(input),
    'expectedStateRevision': expectedStateRevision,
    'idempotencyKey': idempotencyKey,
    'deadlineMs': deadlineMs,
  };
}

final class RemoteControlReceipt {
  const RemoteControlReceipt({
    required this.requestId,
    required this.idempotencyKey,
    required this.status,
    this.jobRef,
    this.stateRevision,
    this.decisionRef,
    this.errorCode,
  });

  final String requestId;
  final String idempotencyKey;
  final String status;
  final String? jobRef;
  final String? stateRevision;
  final String? decisionRef;
  final String? errorCode;

  factory RemoteControlReceipt.fromJson(Map<String, Object?> map) {
    requireProtocol(map, remoteControlReceiptProtocol);
    final status = stringField(map, 'status', max: 16);
    if (!remoteReceiptStatuses.contains(status)) {
      throw const RemoteSurfaceFormatException('receipt status is unsupported');
    }
    final accepted = status == 'accepted';
    requireKeys(
      map,
      required: {
        'protocol',
        'requestId',
        'idempotencyKey',
        'status',
        if (accepted) ...{'stateRevision', 'decisionRef'},
        if (!accepted) 'errorCode',
      },
      optional: {if (accepted) 'jobRef'},
    );
    if (!accepted && map.containsKey('jobRef')) {
      throw const RemoteSurfaceFormatException(
        'a rejected receipt cannot carry a job',
      );
    }
    return RemoteControlReceipt(
      requestId: opaqueField(map, 'requestId'),
      idempotencyKey: opaqueField(map, 'idempotencyKey'),
      status: status,
      jobRef: accepted && map.containsKey('jobRef')
          ? opaqueField(map, 'jobRef')
          : null,
      stateRevision: accepted ? opaqueField(map, 'stateRevision') : null,
      decisionRef: accepted ? opaqueField(map, 'decisionRef') : null,
      errorCode: accepted ? null : errorCodeField(map, 'errorCode'),
    );
  }

  Map<String, Object?> toJson() => {
    'protocol': remoteControlReceiptProtocol,
    'requestId': requestId,
    'idempotencyKey': idempotencyKey,
    'status': status,
    if (jobRef != null) 'jobRef': jobRef,
    if (stateRevision != null) 'stateRevision': stateRevision,
    if (decisionRef != null) 'decisionRef': decisionRef,
    if (errorCode != null) 'errorCode': errorCode,
  };
}

final class RemoteSurfaceEvent {
  const RemoteSurfaceEvent({
    required this.surfaceSessionRef,
    required this.generation,
    required this.seq,
    required this.state,
    required this.occurredAtMs,
    required this.stateRevision,
    this.jobRef,
  });

  final String surfaceSessionRef;
  final int generation;
  final int seq;
  final String state;
  final int occurredAtMs;
  final String stateRevision;
  final String? jobRef;

  factory RemoteSurfaceEvent.fromJson(Map<String, Object?> map) {
    requireProtocol(map, remoteControlEventProtocol);
    requireKeys(
      map,
      required: {
        'protocol',
        'surfaceSessionRef',
        'generation',
        'seq',
        'state',
        'occurredAtMs',
        'stateRevision',
      },
      optional: {'jobRef'},
    );
    final state = stringField(map, 'state', max: 32);
    if (!remoteEventStates.contains(state)) {
      throw const RemoteSurfaceFormatException('event state is unsupported');
    }
    return RemoteSurfaceEvent(
      surfaceSessionRef: opaqueField(map, 'surfaceSessionRef'),
      generation: intField(map, 'generation', min: 1),
      seq: intField(map, 'seq', min: 1),
      state: state,
      occurredAtMs: intField(map, 'occurredAtMs', min: 0),
      stateRevision: opaqueField(map, 'stateRevision'),
      jobRef: map.containsKey('jobRef') ? opaqueField(map, 'jobRef') : null,
    );
  }

  Map<String, Object?> toJson() => {
    'protocol': remoteControlEventProtocol,
    'surfaceSessionRef': surfaceSessionRef,
    'generation': generation,
    'seq': seq,
    if (jobRef != null) 'jobRef': jobRef,
    'state': state,
    'occurredAtMs': occurredAtMs,
    'stateRevision': stateRevision,
  };
}

Object parseRemoteSurfaceFixture(String kind, Map<String, Object?> value) {
  return switch (kind) {
    'hello' => RemoteClientHello.fromJson(value),
    'descriptor' => RemoteSurfaceDescriptor.fromJson(value),
    'snapshot' => RemoteSurfaceSnapshot.fromJson(value),
    'request' => RemoteControlRequest.fromJson(value),
    'receipt' => RemoteControlReceipt.fromJson(value),
    'event' => RemoteSurfaceEvent.fromJson(value),
    _ => throw RemoteSurfaceFormatException('unknown fixture kind $kind'),
  };
}
