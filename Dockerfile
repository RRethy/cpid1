FROM rust:1.67 as builder
RUN cargo install --git https://github.com/RRethy/cpid1.git

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
ENTRYPOINT ["/usr/local/bin/cpid1"]
CMD ["yourapp", "yourapp-args", "--and", "--more", "--args"]
