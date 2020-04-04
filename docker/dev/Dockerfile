FROM rust

RUN rustup install nightly
RUN cargo install cargo-watch
CMD cd /project && cargo +nightly watch -x run
