#!/bin/bash
export VITE_REMOTE=121.0.0.1
export VITE_REMOTE_PORT=3601 #intentionaly set wrong to test if env being picked up
export DATABASE_URL="sqlite:users.db"

echo "VITE_REMOTE: $VITE_REMOTE"
echo "VITE_REMOTE_PORT: $VITE_REMOTE_PORT"


serve frontend & ./backend/aqua-container