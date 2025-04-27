# postman-runner ⚡️

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-blue.svg)](https://github.com/prongbang/postman-runner)
[![Homebrew](https://img.shields.io/badge/Homebrew-available-green.svg)](https://brew.sh)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

> Run multiple Postman collections with a single command. Automate your API testing workflow efficiently.

![postman-runner preview](screenshots/preview.png)

## ✨ Features

- 🚀 **Multiple Collections** - Run multiple Postman collections in one go
- 📊 **Beautiful Reports** - Generate HTML reports with various styles
- ⚙️ **Flexible Configuration** - Easily configure through YAML files
- 🎯 **Selective Execution** - Run specific collections by name
- 🔄 **Sync Mode** - Run collections synchronously for local testing
- 📝 **Detailed Logging** - Comprehensive logs for debugging
- 🛡️ **Insecure Mode** - Support for self-signed certificates

## 🚀 Quick Start

1. Install postman-runner
2. Create a `config.yml` file
3. Run your collections!

```shell
# Run all collections
postman-runner --config config.yml

# Run specific collection
postman-runner --config config.yml --name collection-1

# Run in sync mode
postman-runner --sync
```

## 📦 Installation

### Via Homebrew (Recommended)

```shell
brew update
brew tap prongbang/homebrew-formulae
brew install postman-runner
```

### Via Cargo

```shell
cargo install postman-runner --git https://github.com/prongbang/postman-runner.git
```

### From Source

```shell
# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/prongbang/postman-runner.git
cd postman-runner
cargo build --release
```

## ⚙️ Configuration

Create a `config.yml` file in your project:

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

### Configuration Options

| Option | Description |
|--------|-------------|
| `name` | Unique identifier for each command |
| `collection: local` | Run the collection locally |
| `skipped: true` | Skip this collection during execution |
| `standalone: true` | Run the command independently |
| `insecure: true` | Allow insecure SSL certificates |

## 📊 Report Generation

### Prerequisites

Install Newman and desired reporters:

```shell
# Install Newman
brew install newman

# Install HTML reporters
npm install -g newman-reporter-html
npm install -g newman-reporter-htmlextra
```

### Available Reporters

- **html** - Basic HTML report
- **htmlextra** - Enhanced HTML report with additional features

### Report Structure

```
project/
├── config.yml
└── reporter/
    ├── get-weather-1.html
    ├── get-weather-2.html
    ├── get-weather-3.html
    └── weather-report.html
```

## 💻 Command Line Usage

### Commands

```shell
# Show help
postman-runner --help

# Run with custom config
postman-runner --config custom-config.yml

# Run specific collection
postman-runner --config config.yml --name collection-1

# Run in sync mode (local)
postman-runner --sync

# Enable verbose logging
postman-runner --config config.yml --verbose
```

### Options

| Option | Short | Description |
|--------|-------|-------------|
| `--config` | `-c` | Specify configuration file |
| `--name` | `-n` | Run specific collection by name |
| `--sync` | `-s` | Run collections synchronously |
| `--verbose` | `-v` | Enable verbose logging |
| `--help` | `-h` | Show help information |

## 🌟 Use Cases

- **API Testing** - Automate API testing across multiple collections
- **CI/CD Integration** - Integrate with your CI/CD pipeline
- **Regression Testing** - Run automated regression tests
- **Environment Testing** - Test APIs across different environments
- **Performance Testing** - Measure API performance
- **Integration Testing** - Test API integrations

## 🔍 Best Practices

1. **Organize Collections** - Group related APIs in collections
2. **Use Environment Variables** - Store sensitive data in environment files
3. **Regular Execution** - Schedule regular test runs
4. **Monitor Reports** - Review reports for failures
5. **Version Control** - Keep config files in version control

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- [Documentation](https://github.com/prongbang/postman-runner/wiki)
- [Issue Tracker](https://github.com/prongbang/postman-runner/issues)
- [Newman Documentation](https://learning.postman.com/docs/running-collections/using-newman-cli/command-line-integration-with-newman/)

## 💖 Support the Project

If you find this package helpful, please consider supporting it:

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/prongbang)

## 🙏 Acknowledgments

- Built with Rust 🦀
- IDE Support by [RustRover](https://www.jetbrains.com/rust/)

![RustRover](https://resources.jetbrains.com/help/img/idea/2024.3/RustRover_icon.svg)

---
