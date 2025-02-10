# Qearn API

This is the API for the Qearn Dapp and transaction indexer of Qearn Smart Contract. It is built using [Axum](https://github.com/tokio-rs/axum), [PostgreSQL](https://www.postgresql.org/), [Prisma Rust](https://github.com/Brendonovich/prisma-client-rust).

## Getting Started

To begin with this project:

### Prisma

This project uses Prisma to generate the database client. To use prisma in rust, it is necessary to install the `prisma-client-rust-cli` package. But new version of prisma-client-rust-cli does not support global installation(`cargo install prisma-client-rust-cli`). So we need to install it in the project directory.
Refer this [docs](https://prisma.brendonovich.dev/getting-started/installation) for more details.

This project already added alias for `prisma` in `.cargo/config.toml`. So you can use `prisma` command to run prisma-cli.

```shell
$ cargo prisma generate
```

(if you installed prisma-client-rust-cli globally, you can use `prisma-cli` command to run prisma-cli)

```shell
$ prisma-cli generate
```

To push db schema to database, you can use the following command:

```shell
$ cargo prisma db push
```

### Run Postgres

The most straightforward way to run Postgres is by using a container with a pre-built image. The command below will start latest version of Postgres using [Docker](https://www.docker.com/):

```shell
$ docker run -d -p 5432:5432 -e POSTGRES_PASSWORD=password postgres
```

### Configure the Application

```shell
$ cp .env.sample .env
```

### Starting the Application

With everything else set up, all you need to do now is:

```shell
$ cargo run
```

