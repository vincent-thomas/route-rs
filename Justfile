build:
  cargo build

dev:
  cargo watch -x "just test"

test:
	@cargo nextest r && cargo test --doc

dec:
  cargo doc

lint:
  cargo clippy
