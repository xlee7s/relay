#!/bin/bash
set -e

# 颜色输出
info() { echo -e "\033[32m\033[01m$*\033[0m"; }   # 绿色
error() { echo -e "\033[31m\033[01m$*\033[0m"; exit 1; } # 红色

# 检查是否为 root
if [ "$(id -u)" != "0" ]; then
    error "请使用 root 用户运行此脚本。"
fi

info "正在优化系统参数并启用 BBR..."

rm -f /etc/sysctl.d/ny.conf
cat > /etc/sysctl.d/ny.conf <<'EOF'
net.core.default_qdisc = fq
net.ipv4.tcp_congestion_control = bbr

net.ipv4.conf.all.rp_filter = 0
net.ipv4.tcp_no_metrics_save = 1
net.ipv4.tcp_ecn = 0
net.ipv4.tcp_frto = 0
net.ipv4.tcp_mtu_probing = 0
net.ipv4.tcp_rfc1337 = 1
net.ipv4.tcp_sack = 1
net.ipv4.tcp_fack = 1
net.ipv4.tcp_window_scaling = 1
net.ipv4.tcp_adv_win_scale = -2
net.ipv4.tcp_moderate_rcvbuf = 1
net.ipv4.tcp_rmem = 4096 65536 16777216
net.ipv4.tcp_wmem = 4096 65536 16777216
net.core.rmem_max = 16777216
net.core.wmem_max = 16777216
net.ipv4.udp_rmem_min = 8192
net.ipv4.udp_wmem_min = 8192
net.ipv4.ip_local_port_range = 1024 65535
net.ipv4.tcp_timestamps = 1
net.ipv4.tcp_tw_reuse = 1
net.ipv4.tcp_max_syn_backlog = 4096
net.core.somaxconn = 4096
net.ipv4.tcp_abort_on_overflow = 1

vm.swappiness = 10
fs.file-max = 6553560
EOF

sysctl --system

info "BBR 启用完成，当前拥塞控制算法："
info "$(cat /proc/sys/net/ipv4/tcp_congestion_control)"