build:
  cargo build

dev:
  cargo watch -x "just test"

test:
  cargo nextest r

dec:
  cargo doc

lint:
  cargo clippy
