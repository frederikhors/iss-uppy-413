FROM rust:1.75

COPY ./ ./

RUN cargo build

CMD ["./target/debug/iss-uppy-413"]
