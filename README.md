<p align="center">
  <img src="frontend/public/favicon.svg" width="80" height="80" alt="RelayPanel Logo" />
</p>

<h1 align="center">RelayPanel</h1>

<p align="center">
  ⚡ 自托管 TCP/UDP 端口转发管理面板 ⚡
</p>

<p align="center">
  <a href="https://github.com/xlee7s/relay/releases/latest"><img src="https://img.shields.io/github/v/release/xlee7s/relay?style=flat-square&label=Release&color=blue" alt="Release" /></a>
  <a href="https://github.com/xlee7s/relay/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/xlee7s/relay/ci.yml?style=flat-square&label=CI" alt="CI" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/xlee7s/relay?style=flat-square&label=License&color=blue" alt="License" /></a>
</p>

<p align="center">
  用 Rust 编写，通过 Web UI 管理转发规则、设备分组、流量配额和实时节点状态。<br/>
  轻量：Panel + Node。部署方式：Docker Compose。数据库：SQLite。
</p>

---

## ✨ 功能亮点

- 🔀 **转发规则** — TCP/UDP 多目标转发，故障转移 / 轮询负载均衡，目标熔断自动恢复，域名目标跟随 DDNS
- 🚦 **连接治理** — 每条规则可设并发连接数上限；支持单条 / 批量 / 定时重启，重启会掐断旧连接并重建监听
- 🛒 **套餐与计费** — 用户自助买套餐、卡密充值余额；按「(上行 + 下行) × 线路倍率」扣额度
- 📊 **流量可视** — 按规则 / 用户计量，近 1 / 7 / 30 天趋势图按线路分色堆叠
- 🖥️ **节点管理** — 实时 CPU / 内存 / 连接数、地区识别、掉线通知、面板一键升级节点
- 👤 **用户与分组** — 管理用户、规则和套餐；支持设备分组和节点状态管理
- 🗄️ **部署友好** — SQLite 零配置持久化；面板与节点均支持 amd64 / arm64
- 🔒 **安全** — 首次登录强制修改密码，节点 Bearer Token 鉴权

完整功能说明与使用文档：**[项目文档](docs/DEPLOYMENT.md)**

---

## 🚀 快速开始

```bash
curl -fsSL https://raw.githubusercontent.com/xlee7s/relay/master/install.sh | bash
```

> 🔑 默认账号为 `admin` / `admin123`，首次登录后必须修改密码。

---

## 🏗️ 架构

```text
  浏览器 (React UI)          relay-node (Tokio TCP/UDP)
       │                          ▲
       ▼                          │
   relay-panel  ◄─── WebSocket 配置推送 + HTTP 状态上报
   (Axum API)                     │
       │                          ▼
     SQLite                 转发流量到真实目标
```

---

## 🔄 更新

**面板**：有新版本时，在仪表盘点击「马上更新」即可。更新前会自动备份 SQLite 数据库到 `backups/`，保留最近 5 份。

旧版本首次启用一键更新时，可手动执行：

```bash
cd /opt/relay && git pull --quiet && ./deploy.sh
```

**节点**：面板 → 节点状态 → 点击「升级」。仅 systemd 节点支持面板一键升级。

---

## 🗑️ 卸载

### 卸载面板端

以下命令会删除面板容器和程序目录，但默认保留名为 `relay_panel_data` 的 SQLite Docker 数据卷：

```bash
cd /opt/relay
docker compose -f docker-compose.release.yaml --profile node --profile caddy down --remove-orphans
systemctl disable --now relaypanel-updater.path relaypanel-updater.service 2>/dev/null || true
rm -f /etc/systemd/system/relaypanel-updater.path /etc/systemd/system/relaypanel-updater.service
systemctl daemon-reload
cd /
rm -rf /opt/relay
```

确认不再需要数据库后，可继续删除数据卷：

```bash
docker volume rm relay_panel_data
```

> ⚠️ 删除 `relay_panel_data` 后，用户、规则、套餐和所有面板数据都无法恢复。

### 卸载节点端

```bash
systemctl disable --now relay-node
rm -f /etc/systemd/system/relay-node.service
systemctl daemon-reload
rm -rf /opt/relay-node
```

如果安装节点时使用 `--service-name` 指定了其他名称，请将命令中的 `relay-node` 替换为对应名称。

---

## 🛠️ 本地开发

```bash
cargo build && cargo run -p relay-panel &
cd frontend && npm install && npm run dev
python3 tests/e2e_test.py
```

---

## 📦 技术栈

Rust · Axum · Tokio · sqlx · SQLite · JWT · React 19 · TypeScript · Ant Design · Docker Compose

---

## 📄 许可证与免责声明

Apache-2.0 —— 详见 [LICENSE](LICENSE)。

本仓库基于 [MoeShinX/relay-panel](https://github.com/MoeShinX/relay-panel) 二次开发，原项目版权及 Apache-2.0 许可声明予以保留。

本项目仅供个人学习与研究使用，请在合法合规前提下使用，风险自负。

完整 **[免责声明](docs/DISCLAIMER.md)**
