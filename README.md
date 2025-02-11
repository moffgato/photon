# Photon

Solana indexer for general compression

## Installation

First install dependencies (Ubuntu):

```bash
apt install -y build-essential pkg-config libssl-dev
```

Then run:
```bash
cargo install photon-indexer
```

### Running Photon 

To run photon run:

```bash
# Against localnet
photon

# Against devnet
photon --rpc-url=https://api.devnet.solana.com

# Streaming new blocks using gRPC instead of polling. GRPC_X_TOKEN env variable must be set.
photon --rpc-url=https://api.devnet.solana.com --grpc-url=<grpc_url>

# Using your local Postgres database instead of the default temporary SQL database
photon --db-url=postgres://postgres@localhost/postgres

# Specifying a start slot. Defaults to 0 for localnet and current for devnet/mainnet
photon --start-slot=123 

# To see more configuration options
photon --help
```

### Photon Snapshots

Photon supports snapshots, which enable Photon operators to load Photon more quickly. Otherwise,
operators need to index all of the Solana blocks since the ZK Compression contract was released.

Helius supports a public endpoint for loading snapshots. To load a snapshot simply run:
```bash
photon-snapshot-loader  --snapshot-dir=~/snapshot --snapshot-server-url=https://photon-devnet-snapshot.helius-rpc.com
```

Then use the snapshot to load Photon faster:
```bash
# Photon will disregard the snapshot if the database is more up-to-date.
photon-indexer --snapshot-dir=~/snapshot --r=https://api.devnet.solana.com --db-url=postgres://postgres@localhost/postgres
```

To create snapshots you can run the Photon snapshotter:
```bash
photon-snapshotter --snapshot-dir=~/snapshot
```

You can also specify an R2 bucket for storing the snapshot files instead of a local directory:
```bash
# If you specify an R2 bucket, you must set the R2_ACCESS_KEY, R2_ACCOUNT_ID and R2_SECRET_KEY
photon-snapshotter --r2-bucket=some-bucket --r2-prefix=prefix
```


### Database Management

We support both Postgres and SQLite as database backends. Photon uses a auto-configured SQLite
in-memory database by default. To specify another database backend run migrations and specify the
database url when running Photon.

```bash
export DATABASE_URL="postgres://postgres@localhost/postgres" # Or your SQLlite database url
photon-migration up
photon --db-url=$DATABASE_URL
```

## Local Development

### Running Tests

To run tests, install and run Postgres and SQLlite locally. For MacOS users, we recommend using
Homebrew to run local Postgres: https://wiki.postgresql.org/wiki/Homebrew.

Then export environment variables to configure your RPC and your test Postgres url. For SQLlite testing,
we always use an in-memory SQLlite database, so there is no need to configure a test url.

```bash
export MAINNET_RPC_URL=https://api.devnet.solana.com
export DEVNET_RPC_URL=https://api.mainnet-beta.solana.com
export TEST_DATABASE_URL="postgres://postgres@localhost/postgres"
```

Additionally, for tests we use `swagger-cli` to validate our OpenAPI schemas. So please install it:
```bash
npm install -g @apidevtools/swagger-cli
```

Run minio to test snapshotting:
```bash
wget https://dl.min.io/server/minio/release/linux-amd64/minio
chmod +x minio
```

Finally run the Gnark prover, which is needed for integration tests:
```bash
docker run -p 3001:3001 docker.io/pmantica1/light-prover:1
```

After finishing setup simply run:
```bash
cargo test
```

Note that for both Postgres and SQLlite all migrations will run automatically during tests. So no
prior configuration is needed.

### Database Model Autogeneration

To generate database models first install sea-orm-cli:
```bash
cargo install sea-orm-cli --version 0.10.6
```

Then run:
```bash
sea-orm-cli generate entity -o src/dao/generated
```


### Documentation Generation

In order to update the OpenAPI schemas for the API please first install the `swagger-cli` through:

```bash
npm install -g @apidevtools/swagger-cli
```

Then run:
```bash
cargo run --bin=photon-openapi
```
