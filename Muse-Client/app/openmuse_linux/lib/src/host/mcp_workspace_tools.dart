/// Public Workspace/Registry tool schemas. No DevKit or resource-format business.
final mcpWorkspaceTools = <Map<String, Object?>>[
  for (final name in [
    'workspace_describe',
    'registry_search',
    'registry_install',
  ])
    {
      'name': name,
      'description': name == 'workspace_describe'
          ? 'Describe the current authorized workspace roots.'
          : name == 'registry_search'
          ? 'Search installed and reviewed online plugins.'
          : 'Request installation of a reviewed Registry plugin.',
      'inputSchema': {
        'type': 'object',
        'properties': {
          if (name == 'registry_search')
            'query': {'type': 'string', 'maxLength': 200},
          if (name == 'registry_install')
            'name': {'type': 'string', 'maxLength': 200},
        },
        'required': [
          if (name == 'registry_search') 'query',
          if (name == 'registry_install') 'name',
        ],
        'additionalProperties': false,
      },
      'outputSchema': {'type': 'object'},
    },
];
