FROM rust:latest as builder_rust

ENV DATABASE_URL="aquafier.db"

WORKDIR /build
COPY . .

RUN rustup default nightly

RUN cargo build --package aqua-container --bin aqua-container --release

FROM node:latest as builder_npm

WORKDIR /build

COPY ./web .

# Accept the environment variable during build
ARG VITE_API_ENDPOINT
ENV VITE_API_ENDPOINT=${VITE_API_ENDPOINT}

RUN npm i
RUN npm run build
RUN npm run preview -- --port 3000 & # This is just for reference, we'll move this to the start script

FROM node:latest

EXPOSE 3000
EXPOSE 3600

WORKDIR /app

RUN mkdir /app/backend
COPY --from=builder_rust /build/target/release/aqua-container /app/backend/

RUN mkdir /app/frontend
COPY --from=builder_npm /build/dist /app/frontend

RUN npm install --global pm2 vite

RUN mkdir /app/data

COPY ./actionfiles/aqua_container/script/start_aqua.sh .
RUN chmod +x ./start_aqua.sh

CMD "./start_aqua.sh"