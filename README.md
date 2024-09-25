# postman-runner

Run multiple Postman collections with a single command.

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/prongbang)

![preview.png](screenshots/preview.png)

## Run

- Run sync commands for run on local

```shell
postman-runner --sync
```

- Run multiple commands

```shell
postman-runner --config config.yml
```

- Run single command

```shell
postman-runner --config config.yml --name collection-1
```

## Configuration

- `config.yml` by default

```yaml
report:
  name: Weather Report
  filename: reporter/weather-report.html
  reporter: html
logger: true
commands:
  - name: newman
    standalone: true
    command: newman -v
  - name: collection-1
    collection: local
    command: newman run xxxx -e xxxx
  - name: collection-2
    skipped: true
    insecure: true
    command: newman run xxxx -e xxxx
```

** The `name` is unique, and there are no duplicates.
** The `collection: local` flag run the collection locally.
** The `skipped: true` flag indicates that the test collection is skipped.
** The `standalone: true` flag run the command only.
** The `insecure: true` flag allows the command to run with an insecure, self-signed certificate in the certificate chain.

### Reporter options by collection

- newman

```shell
brew install newman
```

- newman-reporter-html (`html`)

```shell
npm install -g newman-reporter-html
```

- newman-reporter-htmlextra (`htmlextra`)

```shell
npm install -g newman-reporter-htmlextra
```

### Report HTML

```shell
➜  automate-api git:(master) ✗ 
├── config.yml
└── reporter
    ├── get-weather-1.html
    ├── get-weather-2.html
    ├── get-weather-3.html
    └── weather-report.html
```

## Install

### Install with Homebrew

```shell
brew update
brew tap prongbang/homebrew-formulae
brew install postman-runner
```

or

### Install with Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### With Cargo Install

```shell
cargo install postman-runner --git https://github.com/prongbang/postman-runner.git
```

#### With Cargo Build

```shell
cargo build --release
```

