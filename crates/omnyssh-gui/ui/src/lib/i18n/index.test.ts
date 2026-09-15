import { describe, expect, it } from 'vitest';
import { interpolate, messages } from './messages';
import { formatError } from './index';

function leafKeys(obj: unknown, prefix = ''): string[] {
  if (obj && typeof obj === 'object' && !Array.isArray(obj)) {
    return Object.entries(obj as Record<string, unknown>).flatMap(([k, v]) =>
      typeof v === 'string' ? [`${prefix}${k}`] : leafKeys(v, `${prefix}${k}.`)
    );
  }
  return [];
}

describe('i18n messages', () => {
  it('covers the same keys in zh-CN and en', () => {
    expect(leafKeys(messages['zh-CN'])).toEqual(leafKeys(messages.en));
  });

  it('interpolates placeholders', () => {
    expect(interpolate('{running}/{total} 运行中', { running: 2, total: 3 })).toBe('2/3 运行中');
  });

  it('formats form error codes', () => {
    expect(formatError('nameEmpty', messages['zh-CN'])).toBe('名称不能为空');
    expect(formatError('portRange:99x', messages.en)).toBe(
      "Port must be a number between 1 and 65535, got '99x'"
    );
  });
});
