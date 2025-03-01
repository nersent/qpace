// https://eslint.org/docs/latest/rules/
// https://eslint.org/docs/latest/developer-guide/shareable-configs
// https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/eslint-plugin/src/configs/eslint-recommended.ts

// https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/eslint-plugin/docs/rules/naming-convention.md#enforce-that-interface-names-do-not-begin-with-an-i
module.exports = {
  root: true,
  parser: "@typescript-eslint/parser",
  plugins: ["@typescript-eslint", "prettier", "import"],
  settings: {
    "import/resolver": {
      typescript: {},
    },
  },
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:prettier/recommended",
    "plugin:import/recommended",
    "plugin:import/errors",
    "plugin:import/warnings",
    "plugin:import/typescript",
  ],
  parserOptions: {
    ecmaVersion: "latest",
    sourceType: "module",
    project: ["./tsconfig.json"],
    extraFileExtensions: [".json", ".bazed"]
  },
  globals: {
    __DEV__: false,
    jasmine: false,
    beforeAll: false,
    afterAll: false,
    beforeEach: false,
    afterEach: false,
    test: false,
    expect: false,
    describe: false,
    jest: false,
    it: false,
  },
  rules: {
    "array-callback-return": "error",
    "no-debugger": "off",
    "@typescript-eslint/no-unused-vars": "off",
    "@typescript-eslint/explicit-function-return-type": "error",
    "@typescript-eslint/no-explicit-any": "off",
    "@typescript-eslint/no-var-requires": "off",
    "no-empty": "off",
    "@typescript-eslint/no-empty-function": "off",
    "@typescript-eslint/no-inferrable-types": "off",
    '@typescript-eslint/no-empty-interface': 'off',
    '@typescript-eslint/no-this-alias': 'off',
    'no-async-promise-executor': 'off',
    'prefer-const': 'off',
    "@typescript-eslint/no-namespace": "off",
    "import/order": [
      "error",
      {
        "newlines-between": "always",
        alphabetize: {
          order: "asc",
          caseInsensitive: true,
        },
      },
    ],
    "no-constant-condition": "off",
    "prettier/prettier": [
      "error",
      {
        endOfLine: "auto",
      },
    ],
    // "import/no-relative-parent-imports": "error"
//  `   "import/no-relative-parent-imports": [
//       "error",
//       {
//         patterns: [
//           {
//             group: ['../*'],
//             message: 'Usage of relative parent imports is not allowed.',
//           }
//         ],
//       }
//     ],`
    // 'no-restricted-imports': ['error', {'patterns': ['..*']}],
    // ban-types. allow {} empty object. ALLOW IT
    // "@typescript-eslint/ban-types": [
    //   {
    //     types: {
    //       object: false,
    //     },
    //   },
    // ],
  },
  overrides: [
    {
      files: ["*.tsx", "*.test.ts"],
      rules: {
        "@typescript-eslint/explicit-function-return-type": "off",
      },
    },
  ],
};
