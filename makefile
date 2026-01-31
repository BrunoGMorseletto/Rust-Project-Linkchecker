all: build

build:
	cargo build --release

run:
	cargo run

test:
	cargo test 

clean:
	cargo clean

help:
	@echo "Usage:"
	@echo "  make build  - Build the release binary"
	@echo "  make run    - Run the project"
	@echo "  make clean  - Remove build artifacts"