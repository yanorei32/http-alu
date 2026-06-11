# syntax=docker/dockerfile:1.24
FROM rust:1.96.0-bookworm as build-env
LABEL maintainer="yanorei32"

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

WORKDIR /usr/src
RUN cargo new http-alu
COPY LICENSE Cargo.toml Cargo.lock /usr/src/http-alu/
WORKDIR /usr/src/http-alu
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN	cargo install cargo-license && cargo license \
	--authors \
	--do-not-bundle \
	--avoid-dev-deps \
	--avoid-build-deps \
	--filter-platform "$(rustc -vV | sed -n 's|host: ||p')" \
	> CREDITS

RUN cargo build --release
COPY src/ /usr/src/http-alu/src/
RUN touch src/**/* src/* && cargo build --release

FROM debian:bookworm-slim@sha256:66117fe525ba266a4d9de1dc238fa9b9d2fe78ff9d0836b8348d133e836f39b5

WORKDIR /

COPY --chown=root:root --from=build-env \
	/usr/src/http-alu/CREDITS \
	/usr/src/http-alu/LICENSE \
	/usr/share/licenses/http-alu/

COPY --chown=root:root --from=build-env \
	/usr/src/http-alu/target/release/http-alu \
	/usr/bin/http-alu

CMD ["/usr/bin/http-alu"]
