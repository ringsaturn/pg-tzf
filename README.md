# tzf-rs' PG extension.

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

The result is 17k TPS, and could achieve higher throughput in a production
environment.

## LICENSE

- This project is licensed under the MIT License. See the [LICENSE](LICENSE)
  file for details.
- The timezone data is licensed under the ODbL. See the
  [LICENSE_DATA](LICENSE_DATA) file for details.
  - Data source: https://github.com/evansiroky/timezone-boundary-builder
