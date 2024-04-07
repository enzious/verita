// @ts-check

import tseslint from 'typescript-eslint';
import fuzionRecommended from 'fuzionkit-build/eslint/recommended.js';

export default tseslint.config(
  ...fuzionRecommended,
);
