#!/bin/bash
export VITE_REMOTE=121.0.0.1
export VITE_REMOTE_PORT=3601 #intentionaly set wrong to test if env being picked up
# export DATABASE_URL="sqlite:users.db"

export DATABASE_URL="dalmas.db"
export FILE_MODE="private"
export CONTRACT_ADDRESS="0x045f59310ADD88E6d23ca58A0Fa7A55BEE6d2a611"
# for now the system only works with sepolia
export CHAIN="sepolia"
export THEME="dark"
export API_DOMAIN="fdsfds"

echo "VITE_REMOTE: $VITE_REMOTE"
echo "VITE_REMOTE_PORT: $VITE_REMOTE_PORT"


serve frontend & ./backend/aqua-container