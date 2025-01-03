FROM rustlang/rust:nightly as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y libssl-dev pkg-config clang llvm pkg-config nettle-dev libnettle-dev
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev pkg-config clang llvm pkg-config nettle-dev libnettle-dev
COPY --from=builder /app/target/release/sops-gitops-github-action /usr/local/bin/sops-gitops-github-action

COPY scripts/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
