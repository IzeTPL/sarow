FROM ekidd/rust-musl-builder as builder
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/sarow-server /sarow-server
COPY config.toml .
CMD ["/sarow-server"]
