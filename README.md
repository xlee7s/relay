## ✨ 功能亮点

- 🔀 **转发规则** — TCP/UDP 多目标转发，故障转移 / 轮询负载均衡，目标熔断自动恢复，域名目标跟随 DDNS
- 🚦 **连接治理** — 每条规则可设并发连接数上限；支持单条 / 批量 / 定时重启，重启会掐断旧连接并重建监听
- 🛒 **套餐与计费** — 用户自助买套餐、卡密充值余额；按「(上行 + 下行) × 线路倍率」扣额度；一人一个当前套餐，续费叠加、换套餐整体替换
- 📊 **流量可视** — 按规则 / 用户计量，近 1 / 7 / 30 天趋势图按线路分色堆叠，看得出是哪条线路在吃额度
- 🖥️ **节点管理** — 实时 CPU / 内存 / 连接数、地区识别、掉线推送 Telegram 或邮件、面板一键升级节点（免 SSH）
- 👤 **用户与分组** — 管理任意用户的规则与套餐、重置流量密码、封禁；设备分组可隐藏，节点卸载不影响规则
- 🗄️ **部署友好** — 内置 MySQL 5.7 Compose 服务；面板与节点均支持 amd64 / arm64
- 🔒 **安全** — 首次登录强制改密码，节点 Bearer Token 鉴权

---

## 🚀 快速开始

**一条命令部署：**

```bash
curl -fsSL https://raw.githubusercontent.com/xlee7s/relay/master/install.sh | bash
```

> 🔑 **默认账号 `admin` / `admin123`，首次登录强制修改密码。**

---

## 🏗️ 架构

```
  浏览器 (React UI)          relay-node (Tokio TCP/UDP)
       │                          ▲
       ▼                          │
   relay-panel  ◄─── WebSocket 配置推送 + HTTP 状态上报
   (Axum API)                     │
       │                          ▼
   MySQL 5.7                转发流量到真实目标
```

---

## 🔄 更新

**面板**：有新版本时，在仪表盘点「马上更新」即可，无需 SSH。更新前会通过 `mysqldump --single-transaction` 自动备份到 `backups/`（保留最近 5 份）。更新期间面板约半分钟打不开，节点转发不受影响。

一键更新依赖服务器上的更新服务，它会在手动更新时装好。所以 **v1.2.10 及更早版本需要先手动更新一次**，之后就能一键更新（更新前请备份 `.env` 和数据库）：

```bash
cd /opt/relay && git pull --quiet && ./deploy.sh
```

**节点**：面板 → 节点状态 → 点「升级」，无需 SSH。仅 systemd 节点可用（Docker 节点改为更新镜像）。node-v1.2.5 起，节点重启前会停止接新连接、补报已计数的流量，并给进行中的连接最多 5 秒收尾；长连接（隧道、VPN）仍会断开。详见 [转发节点文档](docs/NODE.zh-CN.md#更新)。

---

## 🗑️ 卸载

### 卸载面板端

以下命令会停止并删除面板、MySQL 和可选的节点/Caddy 容器，同时移除面板一键更新服务：

```bash
cd /opt/relay
docker compose -f docker-compose.release.yaml --profile node --profile caddy down --remove-orphans
systemctl disable --now relaypanel-updater.path relaypanel-updater.service 2>/dev/null || true
rm -f /etc/systemd/system/relaypanel-updater.path /etc/systemd/system/relaypanel-updater.service
systemctl daemon-reload
cd /
rm -rf /opt/relay
```

上面的命令默认保留 Docker 中的 MySQL 数据卷。确认不再需要数据库数据时，再执行：

```bash
docker volume rm relay_mysql_data
```

> ⚠️ 删除 `relay_mysql_data` 后，用户、规则、套餐和所有面板数据都无法恢复，请先备份数据库。

### 卸载节点端

默认服务名为 `relay-node`，卸载命令如下：

```bash
systemctl disable --now relay-node
rm -f /etc/systemd/system/relay-node.service
systemctl daemon-reload
rm -rf /opt/relay-node
```

如果安装节点时通过 `--service-name` 指定了其他名称，请将命令中的 `relay-node` 替换为对应名称。
