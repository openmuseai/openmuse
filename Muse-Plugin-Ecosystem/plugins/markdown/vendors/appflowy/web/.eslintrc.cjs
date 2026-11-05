module.exports = {
  // https://eslint.org/docs/latest/use/configure/configuration-files
  env: {
    browser: true,
    es6: true,
    node: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:import/recommended',
    'plugin:import/typescript',
    'plugin:react/recommended',
    'plugin:react/jsx-runtime',
    'plugin:react-hooks/recommended'
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    project: 'tsconfig.json',
    sourceType: 'module',
    tsconfigRootDir: __dirname,
    extraFileExtensions: ['.json'],
  },
  plugins: ['@typescript-eslint', 'react-hooks', 'import', 'unused-imports'],
  settings: {
    'import/resolver': {
      typescript: {
        alwaysTryTypes: true,
        project: 'tsconfig.json',
      },
    },
    "react": {
      "createClass": "createReactClass", // Regex for Component Factory to use,
      // default to "createReactClass"
      "pragma": "React",  // Pragma to use, default to "React"
      "fragment": "Fragment",  // Fragment to use (may be a property of <pragma>), default to "Fragment"
      "version": "detect", // React version. "detect" automatically picks the version you have installed.
      // You can also use `16.0`, `16.3`, etc, if you want to override the detected value.
      // Defaults to the "defaultVersion" setting and warns if missing, and to "detect" in the future
      "defaultVersion": "", // Default React version to use when the version you have installed cannot be detected.
      // If not provided, defaults to the latest React version.
      "flowVersion": "0.53" // Flow version
    },
    "propWrapperFunctions": [
      // The names of any function used to wrap propTypes, e.g. `forbidExtraProps`. If this isn't set, any propTypes wrapped in a function will be skipped.
      "forbidExtraProps",
      { "property": "freeze", "object": "Object" },
      { "property": "myFavoriteWrapper" },
      // for rules that check exact prop wrappers
      { "property": "forbidExtraProps", "exact": true }
    ],
    "componentWrapperFunctions": [
      // The name of any function used to wrap components, e.g. Mobx `observer` function. If this isn't set, components wrapped by these functions will be skipped.
      "observer", // `property`
      { "property": "styled" }, // `object` is optional
      { "property": "observer", "object": "Mobx" },
      { "property": "observer", "object": "<pragma>" } // sets `object` to whatever value `settings.react.pragma` is set to
    ],
    "formComponents": [
      // Components used as alternatives to <form> for forms, eg. <Form endpoint={ url } />
      "CustomForm",
      { "name": "SimpleForm", "formAttribute": "endpoint" },
      { "name": "Form", "formAttribute": ["registerEndpoint", "loginEndpoint"] }, // allows specifying multiple properties if necessary
    ],
    "linkComponents": [
      // Components used as alternatives to <a> for linking, eg. <Link to={ url } />
      "Hyperlink",
      { "name": "MyLink", "linkAttribute": "to" },
      { "name": "Link", "linkAttribute": ["to", "href"] }, // allows specifying multiple properties if necessary
    ]

  },
  rules: {

    'react-hooks/rules-of-hooks': 'error',
    'react-hooks/exhaustive-deps': 'error',
    'react/display-name': 'off',
    'react/prop-types': 'off',
    '@typescript-eslint/adjacent-overload-signatures': 'error',
    '@typescript-eslint/no-empty-function': 'error',
    '@typescript-eslint/no-empty-interface': 'error',
    '@typescript-eslint/no-floating-promises': 'error',
    '@typescript-eslint/await-thenable': 'error',
    '@typescript-eslint/no-namespace': 'error',
    '@typescript-eslint/no-unnecessary-type-assertion': 'error',
    '@typescript-eslint/no-redeclare': 'error',
    '@typescript-eslint/prefer-for-of': 'error',
    '@typescript-eslint/triple-slash-reference': 'error',
    '@typescript-eslint/unified-signatures': 'error',
    'no-shadow': 'off',
    '@typescript-eslint/no-shadow': 'off',
    'constructor-super': 'error',
    eqeqeq: ['error', 'always'],
    'no-cond-assign': 'error',
    'no-duplicate-case': 'error',
    // replaced by import/no-duplicates
    'no-duplicate-imports': 'off',
    'no-empty': [
      'error',
      {
        allowEmptyCatch: true,
      },
    ],
    'no-invalid-this': 'error',
    'no-new-wrappers': 'error',
    'no-param-reassign': 'error',
    'no-sequences': 'error',
    'no-throw-literal': 'error',
    'no-unsafe-finally': 'error',
    'no-unused-labels': 'error',
    'no-var': 'error',
    'no-void': 'off',
    'prefer-const': 'error',
    'prefer-spread': 'off',
    '@typescript-eslint/no-unused-vars': [
      'error',
      {
        argsIgnorePattern: '^_',
      },
    ],
    'padding-line-between-statements': [
      'error',
      { blankLine: 'always', prev: ['const', 'let', 'var'], next: '*' },
      { blankLine: 'any', prev: ['const', 'let', 'var'], next: ['const', 'let', 'var'] },
      { blankLine: 'always', prev: 'import', next: '*' },
      { blankLine: 'any', prev: 'import', next: 'import' },
      { blankLine: 'always', prev: 'block-like', next: '*' },
      { blankLine: 'always', prev: 'block', next: '*' },
    ],
    'import/no-unresolved': ['error', {
      ignore: ['\\.svg$', 'bun']
    }],
    'import/named': 'warn',
    'import/namespace': 'warn',
    'import/default': 'warn',
    'import/export': 'warn',
    'import/no-duplicates': 'error',
    // Detect whether there are modules that are exported but not used.
    'import/no-unused-modules': 'warn',

    // unused-imports should be error level so eslint can auto fix it
    'unused-imports/no-unused-imports': 'error',
    'unused-imports/no-unused-vars': [
      'error',
      {
        vars: 'all',
        varsIgnorePattern: '^_',
        args: 'after-used',
        argsIgnorePattern: '^_'
      }
    ],
    '@typescript-eslint/no-unused-vars': ['error', {
      vars: 'all',
      varsIgnorePattern: '^_',
      args: 'after-used',
      argsIgnorePattern: '^_'
    }],

    'import/order': ['warn', {
      'groups': [
        'builtin',
        'external',
        'internal',
        'parent',
        'sibling',
        'index',
        'object',
        'type'
      ],
      'newlines-between': 'always',
      'alphabetize': {
        'order': 'asc',
        'caseInsensitive': true
      },
      'pathGroups': [
        {
          'pattern': '@/**',
          'group': 'internal',
          'position': 'after'
        },
        {
          'pattern': 'src/**',
          'group': 'internal',
          'position': 'after'
        }
      ],
      'pathGroupsExcludedImportTypes': ['builtin']
    }]
  },
  ignorePatterns: ['src/**/*.test.ts', '**/__tests__/**/*.json', 'package.json', '__mocks__/**/*.ts'],
};
