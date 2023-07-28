watch:
	RUST_BACKTRACE=full cargo watch -s 'cargo test -- --nocapture'

next-watch:
	cargo watch -s 'cargo nextest run'

test-in-ci:
	cargo nextest run --verbose --locked

check-code:
	cargo fmt --check --all
	cargo clippy --all-features
	cargo audit

build:
	cargo build --locked

start-deps-bats:
	docker compose up bats-deps -d

clean-deps:
	docker compose down -t 3

reset-deps-bats: clean-deps start-deps-bats

bats:
	bats -t tests/e2e

e2e: build reset-deps-bats bats
