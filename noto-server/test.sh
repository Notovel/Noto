#!/bin/bash
echo "Starting Rocket server..."
cargo run &
SERVER_PID=$!
sleep 3

zsh

echo "Server PID: $SERVER_PID"
echo "Try connecting to: http://$TAILSCALE_IP:3000"
echo "Press Enter to kill server..."
read
kill $SERVER_PID
