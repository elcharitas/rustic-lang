all: build

build:
	cargo build

run: build
	cargo run

test: build
	cargo test

clean:
	cargo clean

bundle:
	rustc --crate-name rustic --crate-type bin src/main.rs --out-dir bin
