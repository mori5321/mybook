FROM rust:latest

RUN cargo install mdbook --vers "^0.4.36"

CMD ["mdbook", "build"]


