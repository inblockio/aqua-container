#!/bin/bash

if [[ -z "${BACKEND_URL}" ]]; then
  export BACKEND_URL=http://127.0.0.1:3600
else
  export BACKEND_URL=https://${BACKEND_URL}
fi

sed -i -e "s|BACKEND_URL_PLACEHOLDER|$BACKEND_URL|g" /app/frontend/config.json

# serve frontend & ./backend/aqua-container
serve -s frontend --single & ./backend/aqua-container