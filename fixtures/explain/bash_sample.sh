#!/bin/bash

cleanup() {
    echo "Cleaning up temporary files..."
    rm -f /tmp/myapp_*
    exit 0
}

trap cleanup SIGINT SIGTERM

while true; do
    echo "Running..."
    sleep 5
done
