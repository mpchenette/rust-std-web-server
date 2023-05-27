FROM rust as builder
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /target/x86_64-unknown-linux-musl/release/server .
COPY --from=builder /site/ ./site/
EXPOSE 80
CMD ["./server"]

# works for
# docker build . -t <image-name>
# docker run -p 80:80 <image-name>