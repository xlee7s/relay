import { afterEach, describe, expect, it, vi } from 'vitest';
import { copyText } from './clipboard';

describe('copyText', () => {
  afterEach(() => {
    vi.restoreAllMocks();
    document.body.innerHTML = '';
  });

  it('copies the full textarea value with the HTTP fallback', async () => {
    const command = 'bash <(curl -fsSL https://example.com/install.sh) -t token';
    let copied = '';

    Object.defineProperty(document, 'execCommand', {
      configurable: true,
      value: vi.fn(() => {
        const active = document.activeElement as HTMLTextAreaElement;
        copied = active.value.slice(active.selectionStart, active.selectionEnd);
        return true;
      }),
    });

    await expect(copyText(command)).resolves.toBe(true);
    expect(copied).toBe(command);
    expect(document.querySelector('textarea')).toBeNull();
  });

  it('reports a failed legacy copy', async () => {
    Object.defineProperty(document, 'execCommand', {
      configurable: true,
      value: vi.fn(() => false),
    });

    await expect(copyText('command')).resolves.toBe(false);
    expect(document.querySelector('textarea')).toBeNull();
  });
});
