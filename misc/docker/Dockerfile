FROM rust:latest

WORKDIR /opt
COPY db/ /opt/db
COPY web/ /opt/web
COPY Cargo.toml /opt/Cargo.toml

WORKDIR /opt/web
RUN cargo install --path .
WORKDIR /opt/target/release

EXPOSE 3002

CMD ["web"]
#CMD ["sleep", "3600"]
