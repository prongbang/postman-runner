# postman-runner

Run multiple Postman collections with a single command.

## Configuration

- `config.yml` by default

```yaml
report: reporter/report.html
logger: true
commands:
  - name: collection-1
    command: newman run xxxx -e xxxx
  - name: collection-2
    command: newman run xxxx -e xxxx
```

## Install

### With Cargo Install

```shell
cargo install postman-runner --git https://github.com/prongbang/postman-runner.git
```

### With Cargo Build

```shell
cargo build --release
```

## Run

```shell
postman-runner --config config.yml
```
