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

start-deps:
	docker compose -p $$(basename "$$PWD") -f ./vendor/galoy-quickstart/docker-compose.yml up -d

clean-deps:
	docker compose -p $$(basename "$$PWD") -f ./vendor/galoy-quickstart/docker-compose.yml down -t 3

reset-deps: clean-deps start-deps

bats:
	bats -t tests/e2e

e2e: build reset-deps bats

prep-deps:
	cd dev && vendir sync
	cd dev/vendor \
		&& source envs/.envrc \
		&& envsubst < envs/.env.ci > ../.env.galoy \
		&& rm -rf envs

build-x86_64-unknown-linux-musl-release:
	SQLX_OFFLINE=true cargo build --release --locked --target x86_64-unknown-linux-musl

build-x86_64-apple-darwin-release:
	bin/osxcross-compile.sh

build-x86_64-pc-windows-gnu-release:
	bin/wincross-compile.sh
