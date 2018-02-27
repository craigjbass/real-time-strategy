test-once:
	@echo "==== [ unit tests ] ==============================================="
	@echo "==================================================================="
	@echo "                                                                   "
	@(cd game/use_cases && cargo test) || true
	@echo "==== [ acceptance tests] =========================================="
	@echo "==================================================================="
	@echo "                                                                   "
	@cargo test

test:
	cargo watch -s "make test-once"

ci: test-once