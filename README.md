# pi-lite-monitor
Lightweight system resource monitoring (CPU, RAM, Disk, Temp, Network) via a web UI Target: Raspberry Pi  and minimal Linux systems

# ✅ Project Overview
Name: pi-lite-monitor
Goal: Lightweight system resource monitoring (CPU, RAM, Disk, Temp, Network) via a web UI
Target: Raspberry Pi Zero 2W and minimal Linux systems
Language Stack:

Backend: Rust (efficient, low resource use)

Web Server: Axum or tiny-http

Frontend: HTML + Vanilla JS (or optionally Svelte Lite or htmx)

System Info: Use sysinfo or heim crate in Rust

Optional: Serve via Unix Socket or TCP, can use nginx as a proxy

# 🔎 Core Features
CPU usage (per core)

Memory usage

Disk usage (root and mount points)

Temperature

Network traffic (Down/Up)

## 🚀 Quick Install

Run this on your Raspberry Pi:

```bash
curl -sSL https://raw.githubusercontent.com/fixxyblogger/pi-lite-monitor/main/install.sh | bash
```