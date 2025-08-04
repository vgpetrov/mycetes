FROM rust:latest

WORKDIR /opt
COPY db/ /opt/db
COPY web/ /opt/web
COPY Cargo.toml /opt/Cargo.toml

WORKDIR /opt/web
RUN cargo install --path .

EXPOSE 3002

CMD ["mycetes"]
