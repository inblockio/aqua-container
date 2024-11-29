FROM rust:latest as builder_rust

ENV DATABASE_URL="sqlite:pages.db"

WORKDIR /build
COPY . .

RUN rustup default nightly

RUN cargo build --package aqua-container --bin aqua-container --release

FROM node:latest as builder_npm

WORKDIR /build

COPY ./web .
RUN npm i
RUN npm run build

FROM node:latest

RUN export DATABASE_URL="/app/data/sqlite:pages.db"
EXPOSE 3000
EXPOSE 3600

WORKDIR /app

RUN mkdir /app/backend
COPY --from=builder_rust /build/target/release/aqua-container /app/backend/

RUN mkdir /app/frontend
COPY --from=builder_npm /build/dist /app/frontend

RUN npm install --global serve

RUN mkdir /app/data

COPY ./actionfiles/aqua_container/script/start_aqua.sh .
RUN chmod +x ./start_aqua.sh

CMD "./start_aqua.sh"