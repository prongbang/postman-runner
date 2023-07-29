# postman-runner

Run multiple Postman collections with a single command.

![preview.png](screenshots%2Fpreview.png)

## Configuration

- `config.yml` by default

```yaml
report:
  name: Weather Report
  filename: reporter/weather-report.html
  reporter: html
logger: true
commands:
  - name: collection-1
    command: newman run xxxx -e xxxx
  - name: collection-2
    command: newman run xxxx -e xxxx
```

** The `name` is unique, and there are no duplicates.

### Reporter options by collection

- newman-reporter-html (`html`)

```shell
npm install -g newman-reporter-html
```

- newman-reporter-htmlextra (`htmlextra`)

```shell
npm install -g newman-reporter-htmlextra
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
