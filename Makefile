.PHONY=dev test
dev:
	cargo watch -x build -x clippy -x test

test:
	# sequential tests
	cargo test  -- --test-threads 1
