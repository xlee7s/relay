import { Layout, Menu, Button, Space, Modal, Form, Input, message, Spin, Badge, Tooltip, Avatar } from 'antd';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import { useState, Suspense } from 'react';
import {
  DashboardOutlined,
  ApiOutlined,
  CloudServerOutlined,
  UserOutlined,
  LogoutOutlined,
  LockOutlined,
  SettingOutlined,
  ShoppingOutlined,
  TeamOutlined,
  NotificationOutlined,
  GithubOutlined,
  MenuFoldOutlined,
  MenuUnfoldOutlined,
} from '@ant-design/icons';
import { useI18n } from '../i18n/context';
import api from '../api/client';
import type { ApiEnvelope } from '../api/types';
import { useAuth } from '../auth/useAuth';
import { useSite } from '../hooks/useSite';
import { useAnnouncementBadge } from '../hooks/useAnnouncementBadge';
import { makePasswordValidator } from '../utils/password';

const { Sider, Content, Header } = Layout;
export default function MainLayout() {
  const navigate = useNavigate();
  const location = useLocation();
  const { t } = useI18n();
  const { isAdmin, user, logout: authLogout } = useAuth();
  const site = useSite();
  // Marked seen by the archive page itself, so the dot clears only once the
  // notices have actually been shown.
  const { unread } = useAnnouncementBadge(user?.id ?? null);
  const [changePwOpen, setChangePwOpen] = useState(false);
  const [navigationOpen, setNavigationOpen] = useState(true);
  const [pwForm] = Form.useForm();
  const [pwSubmitting, setPwSubmitting] = useState(false);

  // v0.4.11 PR2: role-based navigation.
  // Admin: Dashboard → 个人中心, 转发规则, 设备分组, 节点状态, 隧道配置, 用户管理, 系统设置
  // Regular: 个人中心, 我的规则, 可用节点
  // v1.0.7: 仪表盘 (/) is admin-only — the regular-user dashboard was removed
  // (redirects to /account), so regular users no longer get this menu entry.
  const dashboardItem = { key: '/', icon: <DashboardOutlined />, label: t('dashboard') };
  const sharedItems = [
    { key: '/account', icon: <UserOutlined />, label: t('personalCenter') },
    { key: '/shop', icon: <ShoppingOutlined />, label: t('shop') },
    { key: '/rules', icon: <ApiOutlined />, label: t('myRules') },
    { key: '/nodes', icon: <CloudServerOutlined />, label: t('availableNodes') },
  ];
  // v1.2.4: the admin list had grown to seven top-level entries. Grouped into
  // two submenus by what they ARE, not just to shorten the list:
  //
  //   billing  — records you create, edit and delete day to day
  //   system   — configuration you set once, plus the audit trail
  //
  // Plans and redeem codes deliberately did NOT go under 系统设置: generating
  // codes is a routine task, and burying a daily CRUD page inside "system
  // settings" costs a click every time and misfiles it besides.
  const billingChildren = [
    { key: '/users', label: t('users') },
    { key: '/plans', label: t('planManagement') },
    { key: '/redeem-codes', label: t('redeemCodes') },
  ];
  const systemChildren = [
    { key: '/site-settings', label: t('siteSettings') },
    { key: '/settings', label: t('basicSettings') },
    { key: '/notify-settings', label: t('notifySettings') },
    { key: '/announcement-admin', label: t('announcementAdmin') },
    { key: '/audit-log', label: t('auditLog') },
  ];
  const adminOnlyItems = [
    { key: '/groups', icon: <CloudServerOutlined />, label: t('deviceGroups') },
    {
      key: 'grp-billing',
      icon: <TeamOutlined />,
      label: t('userAndBilling'),
      children: billingChildren,
    },
    {
      key: 'grp-system',
      icon: <SettingOutlined />,
      label: t('systemSettings'),
      children: systemChildren,
    },
  ];
  const menuItems = isAdmin
    ? [dashboardItem, ...sharedItems, ...adminOnlyItems]
    : sharedItems;

  const routeLabels: Record<string, string> = {
    '/': t('dashboard'),
    '/account': t('personalCenter'),
    '/shop': t('shop'),
    '/rules': t('myRules'),
    '/nodes': t('availableNodes'),
    '/groups': t('deviceGroups'),
    '/users': t('users'),
    '/plans': t('planManagement'),
    '/redeem-codes': t('redeemCodes'),
    '/site-settings': t('siteSettings'),
    '/settings': t('basicSettings'),
    '/notify-settings': t('notifySettings'),
    '/announcement-admin': t('announcementAdmin'),
    '/announcements': t('announcements'),
    '/audit-log': t('auditLog'),
  };
  const activePageLabel = routeLabels[location.pathname] || site.site_name || t('brand');

  // Expand whichever group holds the current page. Computed from the path, not
  // stored, so landing on /site-settings from a bookmark opens 系统设置 too.
  const openKeys = [
    billingChildren.some((c) => c.key === location.pathname) ? 'grp-billing' : '',
    systemChildren.some((c) => c.key === location.pathname) ? 'grp-system' : '',
  ].filter(Boolean);

  const logout = () => {
    authLogout();
    navigate('/login');
  };

  const handleChangePassword = async (values: { current_password: string; new_password: string }) => {
    setPwSubmitting(true);
    try {
      const res = await api.put<unknown, ApiEnvelope<null>>('/user/password', values);
      if (res.code !== 0) {
        message.error(res.message);
        return;
      }
      message.success(t('passwordChanged'));
      setChangePwOpen(false);
      pwForm.resetFields();
    } catch {
      message.error(t('passwordChangeFailed'));
    } finally {
      setPwSubmitting(false);
    }
  };

  return (
    <Layout className={`rp-app-shell ${navigationOpen ? 'rp-nav-open' : 'rp-nav-closed'}`}>
      <button
        type="button"
        className="rp-sidebar-backdrop"
        aria-label={t('cancel')}
        onClick={() => setNavigationOpen(false)}
      />
      <Sider
        trigger={null}
        breakpoint="lg"
        collapsedWidth={0}
        collapsed={!navigationOpen}
        onBreakpoint={(broken) => setNavigationOpen(!broken)}
        width={272}
        className="rp-sidebar"
      >
        <div className="rp-brand">
          <span className="rp-brand-mark"><ApiOutlined /></span>
          <span className="rp-brand-name">{site.site_name || t('brand')}</span>
        </div>
        <Menu
          theme="light"
          mode="inline"
          selectedKeys={[location.pathname]}
          defaultOpenKeys={openKeys}
          items={menuItems}
          // Parent entries carry a `grp-` key and no route — clicking one only
          // expands it, so navigating on them would 404.
          onClick={({ key }) => {
            if (key.startsWith('grp-')) return;
            navigate(key);
            // On narrow screens the sider is a modal-style navigation layer.
            // Leave the destination visible after a menu choice instead of
            // keeping the old page dimmed beneath the navigation.
            if (window.matchMedia('(max-width: 991px)').matches) {
              setNavigationOpen(false);
            }
          }}
          className="rp-main-menu"
          style={{ borderRight: 0 }}
        />
        <div className="rp-sidebar-profile">
          <Avatar size={36} icon={<UserOutlined />} className="rp-sidebar-avatar" />
          <div className="rp-sidebar-profile-copy">
            <strong>{user?.username || (isAdmin ? t('admin') : t('user'))}</strong>
            <span>{isAdmin ? t('admin') : t('user')}</span>
          </div>
        </div>
      </Sider>
      <Layout className="rp-main-stage">
        <Header className="rp-topbar">
          <div className="rp-topbar-context">
            <Button
              type="text"
              className="rp-navigation-toggle"
              icon={navigationOpen ? <MenuFoldOutlined /> : <MenuUnfoldOutlined />}
              onClick={() => setNavigationOpen((open) => !open)}
              aria-label={activePageLabel}
            />
            <div className="rp-topbar-route">
              <span>{site.site_name || t('brand')}</span>
              <strong>{activePageLabel}</strong>
            </div>
          </div>
          <Space size="middle" className="rp-topbar-actions">
            {/* v1.2.4: announcements live here rather than in the sidebar.
                The banner already carries the current notice, so a menu row
                would only be a route to the archive — a low-frequency
                destination sitting beside pages people use daily. A bell also
                does what a menu row cannot: say that something is new.

                Labelled, not bare: every other control in this header is icon
                + text, so an icon alone read as decoration and people did not
                find it. The Tooltip is gone with it — a tooltip that repeats
                the visible label is noise. */}
            <Badge dot={unread} offset={[-4, 2]}>
              <Button
                type="text"
                size="small"
                icon={<NotificationOutlined />}
                onClick={() => navigate('/announcements')}
              >
                {t('announcements')}
              </Button>
            </Badge>
            <Button type="text" size="small" icon={<LockOutlined />} onClick={() => setChangePwOpen(true)}>
              {t('changePassword')}
            </Button>
            {/* Icon-only by request, unlike the labelled controls beside it, so
                the tooltip and aria-label carry the name. The login page already
                links the same repository to everyone. */}
            <Tooltip title="GitHub">
              <Button
                type="text"
                size="small"
                icon={<GithubOutlined />}
                href="https://github.com/xlee7s/relay"
                target="_blank"
                rel="noopener noreferrer"
                aria-label="GitHub"
              />
            </Tooltip>
            <Button type="text" size="small" icon={<LogoutOutlined />} onClick={logout}>
              {t('logout')}
            </Button>
          </Space>
        </Header>
        <Content className="rp-content">
          {/* v1.2 (PR4): lazy-loaded pages (router.tsx) suspend here on first
              navigation to their chunk, showing a centered spinner instead of a
              blank pane. */}
          <Suspense fallback={<div style={{ textAlign: 'center', padding: 48 }}><Spin /></div>}>
            <div className="rp-content-inner">
              <div className="rp-workspace"><Outlet /></div>
            </div>
          </Suspense>
        </Content>
      </Layout>

      <Modal
        title={t('changePassword')}
        open={changePwOpen}
        onCancel={() => { setChangePwOpen(false); pwForm.resetFields(); }}
        onOk={() => pwForm.submit()}
        okText={t('save')}
        cancelText={t('cancel')}
        confirmLoading={pwSubmitting}
      >
        <Form form={pwForm} onFinish={handleChangePassword} layout="vertical">
          <Form.Item
            name="current_password"
            label={t('currentPassword')}
            rules={[{ required: true }]}
          >
            <Input.Password autoComplete="current-password" />
          </Form.Item>
          <Form.Item
            name="new_password"
            label={t('newPassword')}
            rules={[
              { required: true, message: t('passwordRequired') },
              { validator: makePasswordValidator(t('newPasswordTooShort'), t('passwordTooLong')) },
            ]}
          >
            <Input.Password autoComplete="new-password" />
          </Form.Item>
          <Form.Item
            name="confirm_password"
            label={t('confirmPassword')}
            dependencies={['new_password']}
            rules={[
              { required: true },
              ({ getFieldValue }) => ({
                validator(_, value) {
                  if (!value || getFieldValue('new_password') === value) {
                    return Promise.resolve();
                  }
                  return Promise.reject(new Error(t('passwordsDoNotMatch')));
                },
              }),
            ]}
          >
            <Input.Password autoComplete="new-password" />
          </Form.Item>
        </Form>
      </Modal>
    </Layout>
  );
}
