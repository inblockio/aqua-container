#!/bin/bash

# Set environment variables from .env file
export $(cat /app/.env | xargs)

# Start backend
./backend/aqua-container &

# Start frontend using Vite preview
cd /app/frontend
VITE_API_ENDPOINT=$VITE_API_ENDPOINT npm run preview --port 3000 --host 0.0.0.0 &

# Wait for background processes
wait