<h1 align="center">Virtual File System (v3)</h1>

<p align="center">
  <img src="./.github/images/logo.png" alt="Vite-CVP image">
</p>

<p align="center" style="font-size: 1.5rem">
  A simple <em>Virtual File System</em> implementation based on requirements <em>version 3</em>
  <br />Built with <strong>Vue, Rust, PostgreSQL</stro>
</p>

## Documentation

The documentation is available [here](./docs/README.md).

## Development

### Prerequisites

- [*Node.JS*](https://nodejs.org/en/) (>= 16.14).
- [*Rust*](https://www.rust-lang.org/) (MSRV: 1.59.0).
- [*PostgreSQL*](https://postgresql.org/) (>= 13.6).

### Client

Navigate to the [`./client`](./client) folder and in there, install required dependencies and run the *Vite Dev Server*:

```sh
cd ./client
pnpm install
pnpm dev
```

### Server

1. First, install [*SQLx CLI*](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli) binary:

    ```sh
    # supports all databases supported by SQLx
    cargo install sqlx-cli

    # only for postgres
    cargo install sqlx-cli --no-default-features --features native-tls,postgres
    ```

2. Navigate to the [`./server`](./server) folder and in there, create database (if not exist) and run migrations:

    ```sh
    cd ./server
    sqlx database create
    sqlx migrate run
    ```

3. Build and run the *Server*:

    ```sh
    cargo run
    ```
