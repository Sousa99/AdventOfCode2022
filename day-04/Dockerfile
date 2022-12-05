FROM rust:1.61

WORKDIR /project

COPY src /project/src
COPY Cargo.toml /project/
COPY input.txt /project/

RUN cargo build --release

CMD ["./target/release/day-04"]