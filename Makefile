clippy:
	cargo clippy --all --all-features --tests -- -D warnings

fmt_check:
	cargo fmt --all -- --check

fmt:
	cargo fmt --all

run_server:
	cargo run --package power-switch --bin server -- -a "127.0.0.1:53453" -d "In Bathroom" -p 125.3

run_client_cli:
	cargo run --package power-switch --bin client -- -a "127.0.0.1:53453"

run_client_ui:
	cargo run --package ui -- -a "127.0.0.1:53453"


