FROM rust:latest

WORKDIR /usr/src/mycetes
COPY . .
RUN cargo install --path .

EXPOSE 3002

CMD ["mycetes"]
