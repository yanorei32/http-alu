# syntax=docker/dockerfile:1.17
FROM rust:1.88.0-bookworm as build-env
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

FROM debian:bookworm-slim@sha256:6ac2c08566499cc2415926653cf2ed7c3aedac445675a013cc09469c9e118fdd

WORKDIR /

COPY --chown=root:root --from=build-env \
	/usr/src/http-alu/CREDITS \
	/usr/src/http-alu/LICENSE \
	/usr/share/licenses/http-alu/

COPY --chown=root:root --from=build-env \
	/usr/src/http-alu/target/release/http-alu \
	/usr/bin/http-alu

CMD ["/usr/bin/http-alu"]
