# tzf-rs' PG extension.

## Installation from source

### Prerequisites

- Rust
- Cargo
- PostgreSQL development headers (version 13, 14, 15, 16, 17)
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
   -- If you install old version of extension, you can drop it and install the new one.
   -- DROP EXTENSION tzf CASCADE;
   CREATE EXTENSION tzf;
   ```

## Installation from pre-built package

Please see [releases](https://github.com/ringsaturn/tzf-pg/releases) for the
pre-built packages.

The artifact is a tarball containing the following files:

```
tzf.so
tzf.control
tzf--{{ version }}.sql
```

## Usage

A full schema is available at [`sql/tzf.sql`](sql/tzf.sql).

The extension provides functions to find timezone names for given coordinates:

- Look up timezone for a coordinate (longitude, latitude):

  ```sql
  -- examples/query_a_coord.sql
  SELECT tzf_tzname(116.3883, 39.9289) AS timezone;
  ```

  Output:

  ```console
     timezone
  ---------------
  Asia/Shanghai
  (1 row)
  ```

- Look up timezone for a batch of coordinates(we use `unnest` to unroll the
  array):

  ```sql
  -- examples/query_a_batch_coords.sql
  SELECT unnest(
     tzf_tzname_batch(
        ARRAY[-74.0060, -118.2437, 139.6917],
        ARRAY[40.7128, 34.0522, 35.6895]
     )
  ) AS timezones;
  ```

  Output:

  ```console
        timezone
  ---------------------
  America/New_York
  America/Los_Angeles
  Asia/Tokyo
  (3 rows)
  ```

- Look up timezone for a point:

  ```sql
  -- examples/query_a_point.sql
  SELECT tzf_tzname_point(point(-74.0060, 40.7128)) AS timezone;
  ```

  Output:

  ```console
     timezone
  ------------------
  America/New_York
  (1 row)
  ```

- Look up timezone for a batch of points:

  ```sql
  -- examples/query_a_batch_points.sql
  SELECT unnest(
     tzf_tzname_batch_points(
        ARRAY[
              point(-74.0060, 40.7128),
              point(-118.2437, 34.0522),
              point(139.6917, 35.6895)
        ]
     )
  ) AS timezones;
  ```

  Output:

  ```console
        timezones
  ---------------------
  America/New_York
  America/Los_Angeles
  Asia/Tokyo
  (3 rows)
  ```

You can see my blog post
[Group world cities by timezone](https://blog.ringsaturn.me/en/posts/2025-05-04-world-city-group-by-timezone/)
for a more large scale usage of this extension.

## Performance

Runs under highly competitive CPU environment in GitHub Actions Runner:

```console
CREATE EXTENSION
Testing tzf_tzname function:
number of clients: 10
number of threads: 1
number of transactions actually processed: 177004
latency average = 0.564 ms
tps = 17726.775670 (without initial connection time)
Testing tzf_tzname_point function:
number of clients: 10
number of threads: 1
number of transactions actually processed: 176147
latency average = 0.567 ms
tps = 17629.349990 (without initial connection time)
Testing tzf_tzname_batch function:
number of clients: 10
number of threads: 1
number of transactions actually processed: 569
latency average = 193.407 ms
tps = 51.704527 (without initial connection time)
Testing tzf_tzname_batch_points function:
number of clients: 10
number of threads: 1
number of transactions actually processed: 360
latency average = 313.814 ms
tps = 31.866004 (without initial connection time)
```

| func                    | tps          | note                                   |
| ----------------------- | ------------ | -------------------------------------- |
| tzf_tzname              | 17726.775670 |                                        |
| tzf_tzname_point        | 17629.349990 |                                        |
| tzf_tzname_batch        | 51.704527    | batch size is 1000, means 51\*1000 TPS |
| tzf_tzname_batch_points | 31.866004    | batch size is 1000, means 31\*1000 TPS |

Please note that the result is under highly competitive CPU environment, so in
real production environment, the result may be better.

## LICENSE

- This project is licensed under the MIT License. See the [LICENSE](LICENSE)
  file for details.
- The timezone data is licensed under the ODbL. See the
  [LICENSE_DATA](LICENSE_DATA) file for details.
  - Data source: <https://github.com/evansiroky/timezone-boundary-builder>
