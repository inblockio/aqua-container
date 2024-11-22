#!/bin/bash

#Remove this! Handle migrations etc in code
/root/.cargo/bin/sqlx database create
/root/.cargo/bin/sqlx migrate run

serve frontend & ./backend/aqua-container