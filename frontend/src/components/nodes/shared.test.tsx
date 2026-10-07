import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import { NetworkCell } from './shared';
import type { NodeDisplayRow } from '../../api/types';

const t = (key: string) => (key === 'regionUnknown' ? '未知' : key);
const row: NodeDisplayRow = {
  group_id: 1,
  public_ipv4: '82.139.204.2',
  ipv4_country_code: 'DE',
  ipv4_country_name: '德国',
};

afterEach(cleanup);

describe('NetworkCell role-based IP visibility', () => {
  it('shows country and IP to an administrator', () => {
    render(<NetworkCell row={row} t={t} showIp />);
    expect(screen.getByText('德国')).toBeInTheDocument();
    expect(screen.getByText('82.139.204.2')).toBeInTheDocument();
  });

  it('shows the country but never the IP to a regular user', () => {
    render(<NetworkCell row={row} t={t} />);
    expect(screen.getByText('德国')).toBeInTheDocument();
    expect(screen.queryByText('82.139.204.2')).not.toBeInTheDocument();
  });
});
