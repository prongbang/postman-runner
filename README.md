# postman-runner

Run multiple Postman collections with a single command.

## Configuration

- `config.yml` by default

```yaml
commands:
  - name: collection-1
    command: ppostman collection run xxxx -e xxxx
  - name: collection-2
    command: ppostman collection run xxxx -e xxxx
```

## Run

```shell
postman-runner --config config.yml
```