import type { ThemeConfig } from 'antd';

// RelayPanel — deep ocean navigation and luminous blue content surfaces.
// Shared tokens keep every page and every Ant Design component visually aligned.
export const rpTheme: ThemeConfig = {
  token: {
    colorPrimary: '#4f6df5',
    colorInfo: '#4f6df5',
    colorSuccess: '#0da678',
    colorWarning: '#e58b24',
    colorError: '#ed5a65',
    colorLink: '#4f6df5',
    colorLinkHover: '#3c5ce7',
    colorBgLayout: '#f3f6fb',
    colorBgContainer: '#ffffff',
    colorBgElevated: '#ffffff',
    colorText: '#152238',
    colorTextSecondary: '#526178',
    colorTextTertiary: '#718096',
    colorTextDescription: '#526178',
    colorBorder: '#d5deeb',
    colorBorderSecondary: '#e2e8f2',
    borderRadius: 11,
    borderRadiusLG: 16,
    controlHeight: 38,
    fontSize: 14,
    fontWeightStrong: 650,
    boxShadow: '0 12px 34px rgba(29, 45, 73, 0.09)',
    boxShadowSecondary: '0 20px 55px rgba(20, 35, 65, 0.14)',
    fontFamily:
      'Inter, "Noto Sans SC", system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", ' +
      '"PingFang SC", "Microsoft YaHei", sans-serif',
  },
  components: {
    Menu: {
      darkItemBg: 'transparent',
      darkSubMenuItemBg: 'transparent',
      darkItemColor: '#91a1b8',
      darkItemHoverColor: '#f7f9ff',
      darkItemHoverBg: 'rgba(255,255,255,0.065)',
      darkItemSelectedBg: 'rgba(92,115,245,0.21)',
      darkItemSelectedColor: '#f3f6ff',
      itemBorderRadius: 12,
    },
    Button: {
      primaryShadow: '0 7px 16px rgba(79, 109, 245, 0.23)',
      defaultShadow: 'none',
      dangerShadow: 'none',
      borderRadius: 11,
    },
    Card: {
      borderRadiusLG: 16,
      headerBg: 'rgba(250, 252, 255, 0.76)',
    },
    Table: {
      headerBg: '#f7f9fd',
      headerColor: '#5d6b80',
      rowHoverBg: '#f5f8ff',
      borderColor: '#e2e8f2',
      headerSplitColor: '#e2e8f2',
    },
    Layout: {
      bodyBg: '#f3f6fb',
      headerBg: 'rgba(255,255,255,0.8)',
      siderBg: '#081426',
      triggerBg: '#06101f',
      triggerColor: '#91a1b8',
    },
    Tabs: {
      inkBarColor: '#4f6df5',
      itemSelectedColor: '#4f6df5',
      itemHoverColor: '#3c5ce7',
    },
    Input: {
      activeBorderColor: '#4f6df5',
      hoverBorderColor: '#9daefa',
      activeShadow: '0 0 0 3px rgba(79, 109, 245, 0.13)',
    },
    Select: {
      activeBorderColor: '#4f6df5',
      hoverBorderColor: '#9daefa',
      activeOutlineColor: 'rgba(79, 109, 245, 0.13)',
    },
    Modal: {
      borderRadiusLG: 16,
    },
    Drawer: {
      colorBgElevated: '#ffffff',
    },
    Pagination: {
      itemActiveBg: '#eef2ff',
    },
  },
};
