FROM rust

RUN rustup install nightly
RUN git clone https://github.com/Ghabriel/rust-rocket-postgres-example.git
WORKDIR rust-rocket-postgres-example
RUN cargo +nightly build
CMD cargo +nightly run
