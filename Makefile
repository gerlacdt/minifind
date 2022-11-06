.PHONY=dev db-migrate db-run db-connect docker-build docker-run
dev:
	# cargo watch -x check -x clippy -x "test -- --test-threads 1"
	cargo watch -x build -x clippy -x

test:
	# no parallel test possible because of the clean_db() dependency of integration tests
	cargo test  -- --test-threads 1
