/* eslint-disable react-refresh/only-export-components */
import { Tag, Tooltip, Typography } from 'antd';
import type { Tfn } from './types';
import type { NodeDisplayRow } from '../../api/types';
import { CountryFlag } from './CountryFlag';

const { Text } = Typography;

/** Dual-stack network cell. Administrators see flag + IP; regular users see
 *  the flag only. The API also strips user-view IP fields, so showIp is a
 *  presentation aid rather than the privacy boundary. */
export function NetworkCell({ row, t, showIp = false }: { row: NodeDisplayRow; t: Tfn; showIp?: boolean }) {
  const v4 = row.public_ipv4 ?? row.public_ip;
  const v6 = row.public_ipv6;
  const all = [
    { key: 'v4', ip: v4, code: row.ipv4_country_code, name: row.ipv4_country_name },
    { key: 'v6', ip: v6, code: row.ipv6_country_code, name: row.ipv6_country_name },
  ].filter((item) => item.ip || item.code || item.name);
  const seen = new Set<string>();
  const entries = showIp ? all : all.filter((item) => {
    const country = item.code || item.name || 'unknown';
    if (seen.has(country)) return false;
    seen.add(country);
    return true;
  });
  if (entries.length === 0) return <Text type="secondary">{t('regionUnknown')}</Text>;
  return (
    <>
      {entries.map((item) => (
        <div key={item.key} style={{ fontSize: 12, lineHeight: '20px', display: 'flex', alignItems: 'center', gap: 6 }}>
          <CountryFlag code={item.code} />
          {showIp && item.ip ? (
            <span className="rp-mono" style={{ whiteSpace: 'nowrap' }}>{item.ip}</span>
          ) : null}
        </div>
      ))}
    </>
  );
}

/** v1.2.12: an online node whose WS control channel is down. It forwards and
 *  reports fine over HTTP, so without this it looks healthy right up until an
 *  upgrade fails. Admin view only (`ws_connected` is absent elsewhere). */
export function wsDownTag(r: NodeDisplayRow, t: Tfn) {
  if (!r.online || r.ws_connected !== false) return null;
  return (
    <Tooltip title={t('nodeWsDownTip')}>
      <Tag color="orange">{t('nodeWsDown')}</Tag>
    </Tooltip>
  );
}

/** Status tag with protocol-mismatch detection. */
export function statusTag(r: NodeDisplayRow, t: Tfn, panelProtocol: number) {
  const v = r.config_protocol_version;
  if (v != null && panelProtocol > 0 && v !== panelProtocol) {
    return <Tag color="red">{t('protocolIncompatible')}</Tag>;
  }
  return r.online ? (
    <>
      <Tag color="green">{t('online')}</Tag>
      {wsDownTag(r, t)}
    </>
  ) : (
    <Tag>{t('offline')}</Tag>
  );
}
