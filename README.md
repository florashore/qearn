# Qearn API

This is the API for the Qearn Dapp application. It is built using [Axum](https://github.com/tokio-rs/axum) and [PostgreSQL](https://www.postgresql.org/).

## Getting Started

To begin with this project:

### Install `sqlx-cli`

SQLx offers a command-line tool for creating and managing databases as well as migrations. It is available on the Cargo crates registry as `sqlx-cli` and can be installed as follows:

```shell
$ cargo install sqlx-cli --features postgres
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

### Set Up the Application Database

With `sqlx-cli` installed and your `.env` file set up, you only need to run the following command to prepare the Postgres database for use:

```shell
$ sqlx db setup
```

### Sqlx offline mode

To avoid the need of having a development database around to compile the project even when no modifications (to the database-accessing parts of the code) are done, this projects enables "offline mode" to cache the results of the SQL query analysis using the sqlx command-line tool. See [sqlx-cli/README.md](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#enable-building-in-offline-mode-with-query) for more details.

```shell
$ cargo sqlx prepare
```

### Starting the Application

With everything else set up, all you need to do now is:

```shell
$ cargo run
```

### Autoreloading

To start the server and autoreload on code changes:

```shell
$ cargo install cargo-watch
$ cargo watch -q -x run
```

To format `.json` logs using [`jq`](https://github.com/jqlang/jq):

```shell
$ cargo watch -q -x run | jq .
```

## Example

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExampleReq {
    pub input: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExampleResp {
    pub output: String,
}

pub async fn example(
    State(state): State<AppState>,
    req: Result<Json<ExampleReq>, JsonRejection>,
) -> Result<Json<ExampleResp>, ApiError> {
    // Returns ApiError::InvalidJsonBody if the Axum built-in extractor
    // returns an error.
    let Json(req) = req?;

    // Proceed with additional validation.
    if req.input.is_empty() {
        return Err(ApiError::InvalidRequest(
            "'input' should not be empty".to_string(),
        ));
    }

    // Anyhow errors are by default converted into ApiError::InternalError and assigned a 500 HTTP status code.
    let data: anyhow::Result<()> = Err(anyhow!("Some internal error"));
    let data = data?;

    let resp = ExampleResp {
        output: "hello".to_string(),
    };
    Ok(Json(resp))
}
```
