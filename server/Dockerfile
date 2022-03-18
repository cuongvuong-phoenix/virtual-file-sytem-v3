# ----------------------------------------------------------------
# chef
# ----------------------------------------------------------------
FROM clux/muslrust as chef
RUN cargo install cargo-chef

# ----------------------------------------------------------------
# planner
# ----------------------------------------------------------------
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ----------------------------------------------------------------
# builder
# ----------------------------------------------------------------
FROM chef as builder
COPY --from=planner /volume/recipe.json .
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --target x86_64-unknown-linux-musl

# ----------------------------------------------------------------
# runtime
# ----------------------------------------------------------------
FROM alpine as runtime
COPY --from=builder \
    /volume/target/x86_64-unknown-linux-musl/release/virtual-file-system-v3-server \
    /bin

EXPOSE 80
CMD [ "/bin/virtual-file-system-v3-server" ]