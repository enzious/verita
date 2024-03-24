// @ts-check

import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';
import stylisticJs from '@stylistic/eslint-plugin-js';

export default tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ['**/*.js', '**/*.ts'],
    rules: {
      semi: 'error',
      'no-unused-vars': [ 'error', { argsIgnorePattern: '^_', varsIgnorePattern: '^_' } ],
      '@typescript-eslint/no-unused-vars': [ 'error', { argsIgnorePattern: '^_', varsIgnorePattern: '^_' } ],
    },
  },
  {
    plugins: { '@stylistic/js': stylisticJs },
    rules:{
      quotes: [ 'error', 'single' ],
      indent: [
        'error',
        2,
        { ignoredNodes: [ 'TemplateResult *', 'TaggedTemplateExpression *', 'PropertyDefinition[decorators]' ] },
      ],
      'no-mixed-spaces-and-tabs': 'error',
      'no-multi-spaces': 'error',
      'object-curly-spacing': [ 'error', 'always' ],
      'space-infix-ops': 'error',
      'no-trailing-spaces': 'error',
      'keyword-spacing': [ 'error', { before: true, after: true } ],
      'comma-dangle': [ 'error', 'always-multiline' ],
    },
  },
);
