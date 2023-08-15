toolchain:
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-apple-darwin
	rustup target add x86_64-unknown-linux-gnu

build:
	cargo build --release --target x86_64-pc-windows-gnu
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target x86_64-unknown-linux-gnu
