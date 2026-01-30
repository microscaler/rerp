export default {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'type-enum': [
      2,
      'always',
      [
        'build',
        'chore',
        'ci',
        'docs',
        'feat',
        'fix',
        'perf',
        'refactor',
        'revert',
        'style',
        'test',
        'tooling',
      ],
    ],
    'header-max-length': [2, 'always', 1500],
    'body-max-line-length': [2, 'always', 3000],
  },
};
