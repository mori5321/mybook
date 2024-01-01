FROM rust:latest

RUN cargo install mdbook --vers "^0.4.36"


COPY ./book.toml ./book.toml
COPY ./src ./src


CMD ["mdbook", "build"]


