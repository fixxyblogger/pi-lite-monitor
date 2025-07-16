#!/bin/bash

set -e

echo "🔧 Installing Pi Lite Monitor..."

# Install Rust if missing
if ! command -v cargo >/dev/null 2>&1; then
  echo "🔹 Installing Rust (this may take a while)..."
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source $HOME/.cargo/env
fi

# Clone and build the app
git clone https://github.com/YOUR_USERNAME/pi-lite-monitor.git
cd pi-lite-monitor

echo "📦 Building in release mode..."
cargo build --release

# Move binary and static files
sudo cp target/release/pi-lite-monitor /usr/local/bin/
sudo mkdir -p /opt/pi-lite-monitor/static
sudo cp -r static/* /opt/pi-lite-monitor/static/

# Create systemd service
sudo tee /etc/systemd/system/pi-lite-monitor.service > /dev/null <<EOF
[Unit]
Description=Pi Lite Monitor
After=network.target

[Service]
ExecStart=/usr/local/bin/pi-lite-monitor
WorkingDirectory=/opt/pi-lite-monitor
Restart=on-failure
User=pi

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable pi-lite-monitor
sudo systemctl start pi-lite-monitor

echo "✅ Installed! Open in browser: http://<your-pi-ip>:9000"