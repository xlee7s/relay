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
  it('shows the flag and IP to an administrator without a country name', () => {
    const { container } = render(<NetworkCell row={row} t={t} showIp />);
    expect(container.querySelector('.country-flag-pill .fi-de')).toBeInTheDocument();
    expect(screen.getByText('82.139.204.2')).toBeInTheDocument();
    expect(screen.queryByText('德国')).not.toBeInTheDocument();
    expect(screen.queryByText('Germany')).not.toBeInTheDocument();
  });

  it('shows only the flag to a regular user', () => {
    const { container } = render(<NetworkCell row={row} t={t} />);
    expect(container.querySelector('.country-flag-pill .fi-de')).toBeInTheDocument();
    expect(screen.queryByText('82.139.204.2')).not.toBeInTheDocument();
    expect(screen.queryByText('德国')).not.toBeInTheDocument();
    expect(screen.queryByText('Germany')).not.toBeInTheDocument();
  });
});
