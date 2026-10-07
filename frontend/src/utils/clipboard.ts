/**
 * Copy text to clipboard with robust fallback for non-HTTPS contexts.
 *
 * navigator.clipboard.writeText is only available in secure contexts (HTTPS
 * or localhost). On HTTP deployments (e.g. http://server-ip:18888), it throws
 * or silently fails. This function tries multiple methods:
 *
 *   1. navigator.clipboard.writeText (modern, HTTPS only)
 *   2. hidden textarea + execCommand('copy') (works on HTTP)
 * Returns true only when the copy genuinely succeeded.
 */
export async function copyText(text: string): Promise<boolean> {
  if (!text || text.length === 0) return false;

  // Method 1: modern Clipboard API (HTTPS / localhost only)
  if (navigator.clipboard && window.isSecureContext) {
    try {
      await navigator.clipboard.writeText(text);
      return true;
    } catch {
      // Fall through to fallback methods
    }
  }

  // Method 2: off-screen textarea + execCommand('copy'). This runs
  // synchronously inside the click event, which is important on browsers that
  // require a user gesture. Do not replace the textarea's native selection
  // with a DOM Range: a textarea's value is not a child text node, so doing so
  // selects an empty element and can make the browser copy an empty string.
  let textarea: HTMLTextAreaElement | null = null;
  try {
    textarea = document.createElement('textarea');
    textarea.value = text;
    textarea.setAttribute('readonly', '');
    textarea.setAttribute('aria-hidden', 'true');
    textarea.style.position = 'fixed';
    textarea.style.top = '0';
    textarea.style.left = '-9999px';
    textarea.style.width = '2px';
    textarea.style.height = '2px';
    textarea.style.padding = '0';
    textarea.style.border = 'none';
    textarea.style.outline = 'none';
    textarea.style.boxShadow = 'none';
    textarea.style.background = 'transparent';
    document.body.appendChild(textarea);

    textarea.focus({ preventScroll: true });
    textarea.select();
    textarea.setSelectionRange(0, textarea.value.length);

    const ok = document.execCommand('copy');
    if (ok) return true;
  } catch {
    // Fall through
  } finally {
    textarea?.remove();
  }

  // All methods failed
  return false;
}
