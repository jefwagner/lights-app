#!/bin/bash

# Create the directory
mkdir -p /opt/lights-app/bin
mkdir -p /opt/lights-app/secrets
mkdir -p /opt/lights-app/www
mkdir -p /opt/lights-app/settings

# Copy over the back-end executable
cp rust/target/release/lights-app /opt/lights-app/bin/

# Copy over the front-end
cp -r frontend/dist/* /opt/lights-app/www/

# Copy over the key and cert
cp lights.key /opt/lights-app/secrets/
cp lights.crt /opt/lights-app/secrets/

# Copy over the systemd service file and start it
cp lights.service /etc/systemd/system/
systemctl daemon-reload

# Start and enable the service
systemctl start lights.service
systemctl enable lights.service
