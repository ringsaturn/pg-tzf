# tzf-rs' PG extension.

## Installation from source

### Prerequisites

- Rust
- Cargo
- PostgreSQL development headers (version 13, 14, 15, or 16)
- [cargo-pgrx](https://github.com/pgcentralfoundation/pgrx), version should be
  same as [`Cargo.toml`](Cargo.toml)

### Installing cargo-pgrx

```bash
# Please see [Cargo.toml](Cargo.toml) for the version of cargo-pgrx.
cargo install cargo-pgrx --version={version}

# Please note that you may need to init for your real PostgreSQL version.
# For example, if you are using PostgreSQL 15 on macOS and install it via Homebrew:
# you may need to run `cargo pgrx init --pg15 "$(brew --prefix postgresql@15)/bin/pg_config"`.
cargo pgrx init
```

### Build and install

1. Clone the repository:
   ```bash
   git clone https://github.com/ringsaturn/tzf-pg.git
   cd tzf-pg
   ```
2. Build and install the extension:
   ```bash
   cargo pgrx install
   ```
3. Use in PostgreSQL:
   ```sql
   CREATE EXTENSION tzf_pg;
   ```

## Installation from pre-built package

Please see [releases](https://github.com/ringsaturn/tzf-pg/releases) for the
pre-built packages.

## Usage

The extension provides functions to find timezone names for given coordinates:

```sql
-- Find timezone for a point (latitude, longitude)
SELECT tzf_tzname(latitude, longitude);

-- Find timezone using point syntax
SELECT tzf_tzname_point(point(longitude, latitude));
```

## Performance

Runs under highly competitive CPU environment in GitHub Actions Runner:

```console
CREATE EXTENSION
Testing tzf_tzname function:
number of clients: 10
number of threads: 1
maximum number of tries: 1
number of transactions actually processed: 174479
number of failed transactions: 0 (0.000%)
latency average = 0.572 ms
tps = 17475.038735 (without initial connection time)
Testing tzf_tzname_point function:
number of clients: 10
number of threads: 1
maximum number of tries: 1
number of transactions actually processed: 173546
number of failed transactions: 0 (0.000%)
latency average = 0.575 ms
tps = 17376.419570 (without initial connection time)
```

The result is 17K TPS, and could achieve higher throughput in a production
environment.

## LICENSE

- This project is licensed under the MIT License. See the [LICENSE](LICENSE)
  file for details.
- The timezone data is licensed under the ODbL. See the
  [LICENSE_DATA](LICENSE_DATA) file for details.
  - Data source: <https://github.com/evansiroky/timezone-boundary-builder>
