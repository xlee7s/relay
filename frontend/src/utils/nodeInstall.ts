const INSTALL_SCRIPT_URL = 'https://raw.githubusercontent.com/xlee7s/relay/master/scripts/relay-node-install.sh';
const BBR_SCRIPT_URL = 'https://raw.githubusercontent.com/xlee7s/relay/master/bbr.sh';

export function buildInstallCommand(token: string, panelUrl: string): string {
  return `bash <(curl -fsSL ${BBR_SCRIPT_URL}) && bash <(curl -fsSL ${INSTALL_SCRIPT_URL}) -t ${token} -u ${panelUrl}`;
}
