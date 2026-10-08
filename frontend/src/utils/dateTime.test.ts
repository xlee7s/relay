import { describe, expect, it } from 'vitest';
import { formatChinaTime } from './dateTime';

describe('formatChinaTime', () => {
  it('converts RFC3339 UTC timestamps to China Standard Time', () => {
    expect(formatChinaTime('2026-10-08T06:16:45.654243717+00:00')).toBe('2026-10-08 14:16:45');
  });

  it('treats timezone-free SQLite timestamps as UTC', () => {
    expect(formatChinaTime('2026-10-08 06:16:45')).toBe('2026-10-08 14:16:45');
  });

  it('keeps malformed values visible instead of hiding them', () => {
    expect(formatChinaTime('not-a-time')).toBe('not-a-time');
  });
});
