build:
	cargo build --locked

e2e: build
	bats -t tests/e2e
