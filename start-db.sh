#!/bin/bash
sudo service mysql start 2>/dev/null || sudo mysqld_safe --skip-syslog &
sleep 2
echo "MySQL started"
