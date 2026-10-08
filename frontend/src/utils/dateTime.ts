const CHINA_TIME_ZONE = 'Asia/Shanghai';

const formatter = new Intl.DateTimeFormat('zh-CN', {
  timeZone: CHINA_TIME_ZONE,
  year: 'numeric',
  month: '2-digit',
  day: '2-digit',
  hour: '2-digit',
  minute: '2-digit',
  second: '2-digit',
  hourCycle: 'h23',
});

/**
 * Formats an API/database timestamp for display in China Standard Time.
 *
 * SQLite timestamps in this project have no offset but are stored as UTC, so
 * append `Z` before parsing them. RFC3339 node timestamps already carry their
 * own offset and are parsed unchanged.
 */
export function formatChinaTime(value: string | null | undefined): string {
  if (!value) return '-';

  const normalized = /^\d{4}-\d{2}-\d{2}[ T]\d{2}:\d{2}:\d{2}(?:\.\d+)?$/.test(value)
    ? `${value.replace(' ', 'T')}Z`
    : value;
  const date = new Date(normalized);
  if (Number.isNaN(date.getTime())) return value;

  const parts = Object.fromEntries(
    formatter.formatToParts(date)
      .filter((part) => part.type !== 'literal')
      .map((part) => [part.type, part.value]),
  );
  return `${parts.year}-${parts.month}-${parts.day} ${parts.hour}:${parts.minute}:${parts.second}`;
}
