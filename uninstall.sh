#!/bin/bash

# stop and disable the service
systemctl stop lights.service
systemctl disable lights.service

# remove the service file
rm /etc/systemd/system/lights.service
systemctl daemon-reload

# remove the lights-app directory
rm -r /opt/lights-app
