import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it } from 'vitest';
import NumberWithUnit from './NumberWithUnit';

describe('NumberWithUnit', () => {
  it('builds one connected prefix/input/unit control', () => {
    const { container } = render(
      <NumberWithUnit prefixLabel="上行" unit="Mbps" aria-label="upload" />,
    );

    const combo = container.querySelector('.rp-number-combo');
    expect(combo).toHaveClass('rp-number-combo--prefixed');
    expect(combo).toHaveClass('rp-number-combo--suffixed');
    expect(screen.getByText('上行')).toHaveClass('rp-number-combo__addon--prefix');
    expect(screen.getByText('Mbps')).toHaveClass('rp-number-combo__addon--unit');
    expect(screen.getByRole('spinbutton', { name: 'upload' })).toBeInTheDocument();
  });

  it('supports unit-only controls without a blank prefix cell', () => {
    const { container } = render(<NumberWithUnit unit="GB" />);
    expect(container.querySelector('.rp-number-combo')).not.toHaveClass('rp-number-combo--prefixed');
    expect(container.querySelector('.rp-number-combo__addon--prefix')).not.toBeInTheDocument();
    expect(screen.getByText('GB')).toBeInTheDocument();
  });

  it('keeps the numeric field editable and the unit after it', async () => {
    const user = userEvent.setup();
    render(<NumberWithUnit unit="GB" aria-label="traffic" />);

    const input = screen.getByRole('spinbutton', { name: 'traffic' });
    await user.type(input, '100');

    expect(input).toHaveValue('100');
    expect(input.closest('.ant-input-number')?.nextElementSibling).toHaveTextContent('GB');
  });
});
