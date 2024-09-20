toolchain:
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-unknown-linux-gnu

toolchain_macos:
	rustup target add x86_64-apple-darwin
	rustup target add aarch64-apple-darwin

build:
	cargo build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-unknown-linux-gnu

# make build_macos version=0.3.0
build_macos:
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target aarch64-apple-darwin
	make zip_macos_x86_64 version=$(version)
	make zip_macos_arm64 version=$(version)

# make zip_macos_x86_64 version=0.3.0
zip_macos_x86_64:
	cd target/x86_64-apple-darwin/release && \
	tar -zcvf $(version)_Darwin_x86_64.tar.gz postman-runner && \
	cd ../../../

# make zip_macos_arm64 version=0.3.0
zip_macos_arm64:
	cd target/aarch64-apple-darwin/release && \
	tar -zcvf $(version)_Darwin_arm64.tar.gz postman-runner && \
	cd ../../../

# make build_macos version=0.3.1
build_macos_release:
	make build_macos version=0.3.1

install:
	cargo install --path .