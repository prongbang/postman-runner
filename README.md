# postman-runner

Run multiple Postman collections with a single command.

## Configuration

- `config.yml` by default

```yaml
commands:
  - name: postman-login
    command: postman login --with-api-key xxxx
  - name: collection-1
    command: postman collection run xxxx -e xxxx
  - name: collection-2
    command: postman collection run xxxx -e xxxx
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
