FROM rust:latest

WORKDIR /opt
COPY db/ /opt/db
COPY api/ /opt/api
COPY stats/ /opt/stats
COPY .env /opt/.env
COPY Cargo.toml /opt/Cargo.toml

WORKDIR /opt/api
RUN cargo install --path .
WORKDIR /opt/target/release

EXPOSE 3002

CMD ["api"]
#CMD ["sleep", "3600"]
