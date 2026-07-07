FROM rust:1.95

COPY . /usr/app
WORKDIR /usr/app

RUN cargo install --path .

CMD ["battlesnake"]
