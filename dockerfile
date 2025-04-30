FROM rust:slim
WORKDIR /usr/src/gameshare
COPY . .

RUN cargo install --path .

CMD ["boardgamesharing"]