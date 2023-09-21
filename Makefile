
build:
	cargo build

build-release:
	cargo build -r

clean:
	cargo clean

migrate: build
	./target/debug/thunderfury migrate

fmt:
	cargo fmt --all

lint:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings
